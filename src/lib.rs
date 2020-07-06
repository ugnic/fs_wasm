mod zip_crypto;
mod passwd;
mod uuid;
mod crc;
mod mt;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const LOCAL_HEADER_SIGNATURE: u32 = 0x04034b50;
struct LocalHeader {
    signature: u32,
    version_needed: u16,
    general_flag: u16,
    compression_method: u16,
    last_mod_time: u16,
    last_mod_date: u16,
    crc: u32,
    compressed_size: u32,
    un_compressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    file_name: Vec<u8>,
    extra_field: Vec<u8>
}

const CENTRAL_DIRECTORY_ENTRY_SIGNATURE: u32 = 0x02014b50;
struct CentralDirectoryEntry {
    signature: u32,
    version_made_by: u16,
    version_needed: u16,
    general_flag: u16,
    compression_method: u16,
    last_mod_time: u16,
    last_mod_date: u16,
    crc: u32,
    compressed_size: u32,
    un_compressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    file_comment_length: u16,
    disk_number_start: u16,
    internal_file_attributes: u16,
    external_file_attributes: u32,
    relative_offset_of_local_header: u32,
    file_name: Vec<u8>,
    extra_field: Vec<u8>,
    file_comment: Vec<u8>
}

const END_OF_CENTRAL_DIRECTORY_SIGNATURE: u32 = 0x06054b50;
struct EndOfCentralDirectory {
    signature: u32,
    num_of_disk: u16,
    disk_where_cd_star: u16,
    num_of_cd_on_disk: u16,
    num_of_cd: u16,
    size_of_cd: u32,
    offset_of_cd: u32,
    zip_comment_length: u16,
    zip_comment: Vec<u8>
}

const DATA_DESCRIPTOR_SIGNATURE: u32 = 0x08074b50;
struct DataDescriptor {
    signature: u32,
    crc: u32,
    compressed_size: u32,
    un_compressed_size: u32
}

const ZIP64_END_OF_CENTRAL_DIRECTORY_SIGNATURE: u32 = 0x06064b50;
const ZIP64_END_OF_CENTRAL_DIRECTORY_LOCATOR_SIGNATURE: u32 = 0x07064b50;

struct FileEntry {
    local_header: LocalHeader,
    body: Vec<u8>
}

#[wasm_bindgen]
pub struct Archive {
    file_entry: Vec<FileEntry>,
    central_dir_entry: Vec<CentralDirectoryEntry>,
    end_of_cd: EndOfCentralDirectory
}

struct File {
    file_name: String,
    file_size: u32,
    last_mod_time: u16,
    last_mod_date: u16,
    file_raw: Vec<u8>,
    encrypt: Vec<u8>,
    crc: u32
}

#[wasm_bindgen]
pub struct Zip {
    uuid: String,
    passwd: String,
    files: Vec<File>,
    is_zip64: bool,
    compress_level: usize
}

impl Zip {
    pub fn new() -> Zip {
        let uuid = uuid::parse(uuid::generate());
        let passwd = passwd::generate(16);
        Zip {
            uuid,
            passwd,
            files: vec![],
            is_zip64: false,
            // ToDo 圧縮レベルのデフォルト値は、圧縮処理実装以降は6
            compress_level: 0
        }
    }

    pub fn add_file(&mut self, file_name: &str, file_size: u64, file_raw: &[u8], last_modified: [u16; 8]) {
        let file_name = file_name.parse().unwrap();

        let file_size: u32 = if file_size <= 0xffffffff {
            file_size as u32
        } else {
            self.is_zip64 = true;
            0xffffffff
        };

        let last_mod_date: u16 = ((last_modified[0] - 1980) << 9) + (last_modified[1] << 5) + (last_modified[2] << 0);
        let last_mod_time: u16 = (last_modified[3] << 1) + (last_modified[4] << 5) + (last_modified[6] >> 1);

        let file_raw = Vec::from(file_raw);
        let encrypt: Vec<u8> = vec![];

        let mut crc: u32 = 0xffffffff;
        for byte in &file_raw { crc = crc::crc32(crc, *byte); };
        let crc = !crc;

        self.files.push(File { file_name, file_size, last_mod_time, last_mod_date, file_raw, encrypt, crc });
    }

    pub fn set_passwd(&mut self, passwd: &str) {
        self.passwd = passwd.parse().unwrap();
    }

    pub fn set_compress_level(&mut self, level: usize) {
        let level: usize = if level < 0 { 0 } else if 10 < level { 10 } else { level };
        self.compress_level = level;
    }

    pub fn exec(&mut self) -> Archive {
        let mut file_entry: Vec<FileEntry> = vec![];
        let mut central_dir_entry: Vec<CentralDirectoryEntry> = vec![];
        for mut data in self.files {
            let mut zc = zip_crypto::ZipCrypto::new();
            data.encrypt = zc.encrypt(&*data.file_raw, &*self.passwd, data.crc);
            let local_header = self.make_local_header(data);
            let body = data.encrypt.clone();
            let central_directory = self.make_central_directory_entry(&local_header);
            file_entry.push(FileEntry{ local_header, body });
            central_dir_entry.push(central_directory);
        }
        let end_of_cd = self.make_end_of_central_directory(&file_entry ,&central_dir_entry);
        Archive {
            file_entry,
            central_dir_entry,
            end_of_cd
        }
    }

    fn make_local_header(&mut self, file: File) -> LocalHeader {
        LocalHeader {
            signature: LOCAL_HEADER_SIGNATURE,
            version_needed: if self.is_zip64 { 0x002e } else { 0x0014 },
            general_flag: 0x0001,
            compression_method: 0x0000,
            last_mod_time: file.last_mod_time,
            last_mod_date: file.last_mod_date,
            crc: file.crc,
            compressed_size: file.encrypt.len() as u32,
            un_compressed_size: file.file_raw.len() as u32,
            file_name_length: file.file_name.len() as u16,
            extra_field_length: 0x0000,
            file_name: Vec::from(file.file_name),
            extra_field: vec![]
        }
    }

    fn make_central_directory_entry(&mut self, local_header: &LocalHeader) -> CentralDirectoryEntry {
        CentralDirectoryEntry {
            signature: CENTRAL_DIRECTORY_ENTRY_SIGNATURE,
            version_made_by: 0x003f,
            version_needed: local_header.version_needed,
            general_flag: local_header.general_flag,
            compression_method: local_header.compression_method,
            last_mod_time: local_header.last_mod_time,
            last_mod_date: local_header.last_mod_date,
            crc: local_header.crc,
            compressed_size: local_header.compressed_size,
            un_compressed_size: local_header.un_compressed_size,
            file_name_length: local_header.file_name_length,
            extra_field_length: local_header.extra_field_length,
            file_comment_length: 0x0000,
            disk_number_start: if self.is_zip64 { 0xffff } else { 0x0000 },
            internal_file_attributes: 0x0000,
            external_file_attributes: 0x00000000,
            // ToDo 引数で渡ってきたLocalHeaderのオフセット
            relative_offset_of_local_header: 0x00000000,
            file_name: local_header.file_name.clone(),
            extra_field: vec![],
            file_comment: vec![]
        }
    }

    fn make_end_of_central_directory(&mut self, _file_entry: &Vec<FileEntry>, central_dir_entry: &Vec<CentralDirectoryEntry>) -> EndOfCentralDirectory {
        EndOfCentralDirectory {
            signature: END_OF_CENTRAL_DIRECTORY_SIGNATURE,
            num_of_disk: if self.is_zip64 { 0xffff } else { 0x0000 },
            disk_where_cd_star: if self.is_zip64 { 0xffff } else { 0x0000 },
            num_of_cd_on_disk: if self.is_zip64 { 0xffff } else { central_dir_entry.len() as u16 },
            num_of_cd: if self.is_zip64 { 0xffff } else { central_dir_entry.len() as u16 },
            size_of_cd: if self.is_zip64 { 0xffffffff } else { 0x00 },
            offset_of_cd: if self.is_zip64 { 0xffffffff } else { 0x00 },
            zip_comment_length: 0x0000,
            zip_comment: vec![]
        }
    }
}

// #[wasm_bindgen]
// pub fn make_zip(file_name: &str, file_raw: &[u8], passwd: &str, last_modified: [u16; 8]) -> Vec<u8> {
//     let mut crc = 0xffffffff;
//     for byte in file_raw {
//         crc = crc::crc32(crc, *byte);
//     }
//     let mut crc = transform_u32_to_array_of_u8(!crc);
//     crc.reverse();
//
//     let mut zc = ZipCryptoKeys::new();
//     let encrypt_raw = zc.encrypt(file_raw, passwd);
//
//     let file_size: u32 = file_raw.len() as u32;
//     let compress_size: u32 = encrypt_raw.len() as u32;
//
//     let datepart: u16 = ((last_modified[0] - 1980) << 9) + (last_modified[1] << 5) + (last_modified[2] << 0);
//     let timepart: u16 = (last_modified[3] << 1) + (last_modified[4] << 5) + (last_modified[6] >> 1);
// }
//
// // ToDo リファクタしろ!
// #[wasm_bindgen]
// pub fn end_of_directory(cd_size: u32, cd_offset: u32) -> Vec<u8> {
//     let mut eocd: Vec<u8> = Vec::new();
//     //---
//     let signature: Vec<u8> = vec![0x50, 0x4b, 0x05, 0x06];
//     let number_of_disk: Vec<u8> = vec![0x00, 0x00];
//     let number_of_disk_central_dir: Vec<u8> = vec![0x00, 0x00];
//     let total_number_of_central_dir_disk: Vec<u8> = vec![0x01, 0x00];
//     let total_number_of_central_dir: Vec<u8> = vec![0x01, 0x00];
//     let mut central_dir_size: Vec<u8> = transform_u32_to_array_of_u8(cd_size);
//     central_dir_size.reverse();
//     let mut central_dir_offset: Vec<u8> = transform_u32_to_array_of_u8(cd_offset);
//     central_dir_offset.reverse();
//     let comment_len: Vec<u8> = vec![0x00, 0x00];
//     eocd.extend(signature);
//     eocd.extend(number_of_disk);
//     eocd.extend(number_of_disk_central_dir);
//     eocd.extend(total_number_of_central_dir_disk);
//     eocd.extend(total_number_of_central_dir);
//     eocd.extend(central_dir_size);
//     eocd.extend(central_dir_offset);
//     eocd.extend(comment_len);
//     eocd
// }
//
// // ToDo リファクタしろ!
// #[wasm_bindgen]
// pub fn central_dir_entry(file_name: &str, file_raw: &[u8], file_size: &[u32]) -> Vec<u8> {
//     let mut central_header: Vec<u8> = Vec::new();
//     //---
//     let signature: Vec<u8> = vec![0x50, 0x4b, 0x01, 0x02];
//     let version_made_by: Vec<u8> = vec![0x3f, 0x00];
//     // ローカルヘッダの項目と同じ--------------------
//     let version_needed: Vec<u8> = vec![0x0a, 0x00];
//     let general_flag: Vec<u8> = vec![0x01, 0x00];
//     let compression_method: Vec<u8> = vec![0x00, 0x00];
//     let last_modify_time: Vec<u8> = vec![0x00, 0x00];
//     let last_modify_date: Vec<u8> = vec![0x00, 0x00];
//     let crc: u32 = crc32(file_raw);
//     let mut crc: Vec<u8> = transform_u32_to_array_of_u8(crc);
//     crc.reverse();
//     let mut compress_size: Vec<u8> = transform_u32_to_array_of_u8(file_size[0]);
//     compress_size.reverse();
//     let mut un_compress_size: Vec<u8> = transform_u32_to_array_of_u8(file_size[0]);
//     un_compress_size.reverse();
//     let file_name: Vec<u8> = file_name_to_vec(file_name);
//     let mut file_name_length: Vec<u8> = transform_usize_to_array_of_u8(file_name.len());
//     file_name_length.reverse();
//     let expansion_length: Vec<u8> = vec![0x00, 0x00];
//     //　ここまで------------------------------------
//     let comment_len: Vec<u8> = vec![0x00, 0x00];
//     let disk_number: Vec<u8> = vec![0x00, 0x00];
//     let internal_attribute: Vec<u8> = vec![0x00, 0x00];
//     let external_attribute: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
//     // ToDo ファイル数に応じて可変
//     let relative_offset_of_local_header: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
//     central_header.extend(signature);
//     central_header.extend(version_made_by);
//     central_header.extend(version_needed);
//     central_header.extend(general_flag);
//     central_header.extend(compression_method);
//     central_header.extend(last_modify_time);
//     central_header.extend(last_modify_date);
//     central_header.extend(crc);
//     central_header.extend(compress_size);
//     central_header.extend(un_compress_size);
//     central_header.extend(file_name_length);
//     central_header.extend(expansion_length);
//     central_header.extend(comment_len);
//     central_header.extend(disk_number);
//     central_header.extend(internal_attribute);
//     central_header.extend(external_attribute);
//     central_header.extend(relative_offset_of_local_header);
//     central_header.extend(file_name);
//     central_header
// }
//
// // ToDo リファクタしろ!
// #[wasm_bindgen]
// pub fn body(file_raw: &[u8]) -> Vec<u8> {
//     let passwd = "1234";
//     let mut zc = ZipCryptoKeys::new();
//     let body = zc.encrypt(file_raw, passwd);
//     body
// }
//
// // ToDo リファクタしろ!
// #[wasm_bindgen]
// pub fn local_file_header(file_name: &str, file_raw: &[u8], file_size: &[u32]) -> Vec<u8> {
//     let mut local_header: Vec<u8> = Vec::new();
//     let signature: Vec<u8> = vec![0x50, 0x4b, 0x03, 0x04];
//     // ToDo 圧縮レベルと暗号化の有無に応じて可変
//     let need_version: Vec<u8> = vec![0x0a, 0x00];
//     // ToDo 圧縮や暗号化の有無に応じて可変
//     let general_flag: Vec<u8> = vec![0x01, 0x00];
//     // ToDo 圧縮の有無とレベルに応じて可変
//     let compression_method: Vec<u8> = vec![0x00, 0x00];
//     // ToDo ファイルから最終更新日時を取得し可変
//     let last_modify_time: Vec<u8> = vec![0x00, 0x00];
//     let last_modify_date: Vec<u8> = vec![0x00, 0x00];
//     let crc: u32 = crc32(file_raw);
//     let mut crc: Vec<u8> = transform_u32_to_array_of_u8(crc);
//     crc.reverse();
//     // ToDo 圧縮処理実装後に圧縮後のサイズを求める
//     let mut compress_size: Vec<u8> = transform_u32_to_array_of_u8(file_size[0]);
//     compress_size.reverse();
//     let mut un_compress_size: Vec<u8> = transform_u32_to_array_of_u8(file_size[0]);
//     un_compress_size.reverse();
//     let file_name: Vec<u8> = file_name_to_vec(file_name);
//     let mut file_name_length: Vec<u8> = transform_usize_to_array_of_u8(file_name.len());
//     file_name_length.reverse();
//     let expansion_length: Vec<u8> = vec![0x00, 0x00];
//     local_header.extend(signature);
//     local_header.extend(need_version);
//     local_header.extend(general_flag);
//     local_header.extend(compression_method);
//     local_header.extend(last_modify_time);
//     local_header.extend(last_modify_date);
//     local_header.extend(crc);
//     local_header.extend(compress_size);
//     local_header.extend(un_compress_size);
//     local_header.extend(file_name_length);
//     local_header.extend(expansion_length);
//     local_header.extend(file_name);
//     local_header
// }
//
// fn transform_u32_to_array_of_u8(x:u32) -> Vec<u8> {
//     let mut u32_to_u8: Vec<u8> = Vec::with_capacity(4);
//     u32_to_u8.push(((x >> 24) & 0xff) as u8);
//     u32_to_u8.push(((x >> 16) & 0xff) as u8);
//     u32_to_u8.push(((x >> 8) & 0xff) as u8);
//     u32_to_u8.push((x & 0xff) as u8);
//     u32_to_u8
// }
//
// fn transform_usize_to_array_of_u8(x:usize) -> Vec<u8> {
//     let mut usize_to_u8: Vec<u8> = Vec::with_capacity(2);
//     usize_to_u8.push(((x >> 8) & 0xff) as u8);
//     usize_to_u8.push((x & 0xff) as u8);
//     usize_to_u8
// }
//
// fn file_name_to_vec(file_name: &str) -> Vec<u8> {
//     ["test/", file_name].concat().as_bytes().to_vec()
// }