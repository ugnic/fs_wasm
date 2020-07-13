mod byteorder;
mod zip_crypto;
mod passwd;
mod uuid;
mod crc;
mod mt;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

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
#[wasm_bindgen]
pub struct LocalHeader {
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
#[wasm_bindgen]
pub struct CentralDirectoryEntry {
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
#[wasm_bindgen]
pub struct EndOfCentralDirectory {
    signature: u32,
    num_of_disk: u16,
    disk_where_cd_start: u16,
    num_of_cd_on_disk: u16,
    num_of_cd: u16,
    size_of_cd: u32,
    offset_of_cd: u32,
    zip_comment_length: u16,
    zip_comment: Vec<u8>
}

const DATA_DESCRIPTOR_SIGNATURE: u32 = 0x08074b50;
#[wasm_bindgen]
pub struct DataDescriptor {
    signature: u32,
    crc: u32,
    compressed_size: u32,
    un_compressed_size: u32
}

const ZIP64_END_OF_CENTRAL_DIRECTORY_RECORD_SIGNATURE: u32 = 0x06064b50;
#[wasm_bindgen]
pub struct Zip64EndOfCentralDirectoryRecord {
    signature: u32,
    own_size: u64,
    version_made_by: u16,
    version_needed: u16,
    num_of_disk: u32,
    disk_where_cd_start: u32,
    num_of_cd_on_disk: u64,
    num_of_cd: u64,
    size_of_cd: u64,
    offset_of_cd: u64,
    zip64_extensible_data_sector: Vec<u8>
}

const ZIP64_END_OF_CENTRAL_DIRECTORY_LOCATOR_SIGNATURE: u32 = 0x07064b50;
#[wasm_bindgen]
pub struct Zip64EndOfCentralDirectoryLocator {
    signature: u32,
    disk_where_zip64_cd_start: u32,
    relative_offset_of_zip64_eocd_record: u64,
    total_num_of_disk: u32
}

#[wasm_bindgen]
pub struct FileEntry {
    local_header: LocalHeader,
    body: Vec<u8>,
    offset: u32
}

#[wasm_bindgen]
pub struct Archive {
    file_entry: Vec<FileEntry>,
    central_dir_entry: Vec<CentralDirectoryEntry>,
    end_of_cd: EndOfCentralDirectory
}

#[wasm_bindgen]
pub struct File {
    file_name: String,
    file_name_raw: String,
    file_size: u32,
    last_mod_time: u16,
    last_mod_date: u16,
    file_raw: Vec<u8>,
    deflate: Vec<u8>,
    encrypt: Vec<u8>,
    crc: u32
}

impl File {
    fn new() -> File {
        File {
            file_name: "".to_string(),
            file_name_raw: "".to_string(),
            file_size: 0,
            last_mod_time: 0,
            last_mod_date: 0,
            file_raw: vec![],
            deflate: vec![],
            encrypt: vec![],
            crc: 0
        }
    }

    fn set_meta(&mut self, uuid: &String, file_name: &str, file_size: u64, last_modified: &[u16]) {
        self.file_name_raw = file_name.parse().unwrap();
        self.file_name = format!("{}{}", *uuid, self.file_name_raw);

        self.file_size = if file_size < 0xffffffff { file_size as u32 } else { 0xffffffff };

        self.last_mod_date = ((last_modified[0] - 1980) << 9) + (last_modified[1] << 5) + (last_modified[2] << 0);
        self.last_mod_time = (last_modified[5] >> 1) + (last_modified[4] << 5) + (last_modified[3] << 11);
    }

    fn set_file_raw(&mut self, file_raw: Vec<u8>, level: u32) {
        self.file_raw = file_raw.clone();
        // ToDo 果てしなく難しいかもしれないけどDeflateの計算はStreamで出来そう
        self.deflate = zip_crypto::ZipCrypto::deflate_encode_raw(level, file_raw);

        // ToDo CRCの計算も比較的簡単にStreamでできそう
        let mut crc: u32 = 0xffffffff;
        // for file_slice in self.file_raw.iter() { for byte in file_slice.iter() { crc = crc::crc32(crc, *byte); }; };
        for byte in self.file_raw.iter() { crc = crc::crc32(crc, *byte); };
        self.crc = !crc;
    }

    fn calc_file_raw(&mut self, level: u32) {
        let file_raw = self.file_raw.clone();
        // ToDo 果てしなく難しいかもしれないけどDeflateの計算はStreamで出来そう
        self.deflate = zip_crypto::ZipCrypto::deflate_encode_raw(level, file_raw);

        // ToDo CRCの計算も比較的簡単にStreamでできそう
        let mut crc: u32 = 0xffffffff;
        // for file_slice in self.file_raw.iter() { for byte in file_slice.iter() { crc = crc::crc32(crc, *byte); }; };
        for byte in self.file_raw.iter() { crc = crc::crc32(crc, *byte); };
        self.crc = !crc;
    }
}

#[wasm_bindgen]
pub struct FileSlice {
    slice: Vec<u8>
}

#[wasm_bindgen]
pub struct FileStream {
    file_name: String,
    stream: Vec<FileSlice>
}

#[wasm_bindgen]
pub struct Zip {
    seed: u32,
    uuid: String,
    passwd: String,
    files: Vec<File>,
    is_zip64: bool,
    is_compress: bool,
    compress_level: u32,
    file_stream: Vec<FileStream>
}

#[wasm_bindgen]
impl Zip {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Zip {
        let uuid = uuid::parse(uuid::generate(seed));
        let passwd = passwd::generate(16, seed);
        Zip { seed, uuid, passwd, files: vec![], is_zip64: false, is_compress: true, compress_level: 6, file_stream: vec![] }
    }

    pub fn get_uuid(&mut self) -> String {
        format!("{}", self.uuid)
    }

    pub fn get_passwd(&mut self) -> String {
        format!("{}", self.passwd)
    }

    pub fn get_compress_level(&mut self) -> u32 {
        self.compress_level
    }

    pub fn set_passwd(&mut self, passwd: &str) {
        self.passwd = passwd.parse().unwrap();
    }

    fn set_compress_level(&mut self, level: i8) {
        let level: u32 = if level < 0 { 0 } else if 9 < level { 9 } else { level as u32 };
        self.is_compress = if 0 < level { true } else { false };
        self.compress_level = level;
    }

    pub fn create_file(&mut self, file_name: &str, file_size: u64, last_modified: &[u16]) {
        let mut file = File::new();
        file.set_meta(&self.uuid, file_name, file_size, last_modified);
        let stream = FileStream {
            file_name: file_name.to_string(),
            stream: vec![]
        };
        self.file_stream.push(stream);
        self.files.push(file);
    }

    // pub fn add_file_raw_from_stream(&mut self, file_name: &str, file_raw: &[u8]) {
    //     for file in self.files.iter_mut() {
    //         if file.file_name == file_name.to_string() {
    //             file.file_raw.extend(file_raw);
    //             break;
    //         }
    //     }
    // }
    pub fn add_file_raw_from_stream(&mut self, file_name: &str, file_raw: &[u8]) {
        let slice = FileSlice{
            slice: Vec::from(file_raw)
        };
        for stream in self.file_stream.iter_mut() {
            if stream.file_name == file_name.to_string() {
                stream.stream.push(slice);
                break;
            }
        }
    }

    // pub fn done_of_file_stream(&mut self, file_name: &str) {
    //     for file in self.files.iter_mut() {
    //         if file.file_name == file_name.to_string() {
    //             file.calc_file_raw(self.compress_level);
    //         }
    //     }
    // }
    pub fn done_of_file_stream(&mut self, file_name: &str) {
        let mut file_raw: Vec<Vec<u8>> = vec![];
        for stream in self.file_stream.iter_mut() {
            if stream.file_name == file_name.to_string() {
                for slice in stream.stream.iter_mut() {
                    console_log!("{}", slice.slice.len());
                }
                break;
            }
        }
        // for file in self.files.iter_mut() {
        //     if file.file_name == file_name.to_string() {
        //         file.set_file_raw(file_raw.clone(), self.compress_level);
        //         break;
        //     }
        // }
    }

    pub fn save(&mut self) -> Vec<u8> {
        let mut mt = mt::MersenneTwister::new(self.seed);
        let mut file_entry: Vec<FileEntry> = vec![];
        let mut central_dir_entry: Vec<CentralDirectoryEntry> = vec![];
        let mut offset = 0;
        for mut data in self.files.iter_mut() {
            let mut zc = zip_crypto::ZipCrypto::new(mt.next());
            let file_raw = data.deflate.clone();
            data.encrypt = zc.encrypt(&*file_raw, &*self.passwd, data.crc);
            let body = data.encrypt.clone();
            let local_header = Zip::make_local_header(self.is_zip64, self.is_compress, data);
            let central_dir = Zip::make_central_directory_entry(self.is_zip64, &local_header, offset);
            let file_entry_length = 30 + (byteorder::le_from_u16(local_header.file_name_length) + local_header.extra_field_length) as u32 + body.len() as u32;
            file_entry.push(FileEntry{ local_header, body, offset });
            offset += file_entry_length;
            central_dir_entry.push(central_dir);
        }
        let end_of_cd = Zip::make_end_of_central_directory(self.is_zip64, &file_entry, &central_dir_entry);
        let archive = Archive{
            file_entry,
            central_dir_entry,
            end_of_cd
        };
        let zip = self.check_archive(archive);
        zip
    }

    pub fn check_archive(&mut self, archive: Archive) -> Vec<u8> {
        let mut zip: Vec<u8> = vec![];
        for fe in archive.file_entry.iter() {
            let lh = &fe.local_header;
            // console_log!("{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}",
            //          lh.signature, lh.version_needed, lh.general_flag, lh.compression_method, lh.last_mod_time, lh.last_mod_date, lh.crc,
            //          lh.compressed_size, lh.un_compressed_size, lh.file_name_length, lh.extra_field_length);
            zip.extend(lh.signature.to_be_bytes().iter());
            zip.extend(lh.version_needed.to_be_bytes().iter());
            zip.extend(lh.general_flag.to_be_bytes().iter());
            zip.extend(lh.compression_method.to_be_bytes().iter());
            zip.extend(lh.last_mod_time.to_be_bytes().iter());
            zip.extend(lh.last_mod_date.to_be_bytes().iter());
            zip.extend(lh.crc.to_be_bytes().iter());
            zip.extend(lh.compressed_size.to_be_bytes().iter());
            zip.extend(lh.un_compressed_size.to_be_bytes().iter());
            zip.extend(lh.file_name_length.to_be_bytes().iter());
            zip.extend(lh.extra_field_length.to_be_bytes().iter());
            for byte in &lh.file_name {
                // console_log!("{:x}", *byte);
                zip.push(*byte);
            }
            for byte in &fe.body {
                // console_log!("{:x} ", *byte);
                zip.push(*byte);
            }
        }
        for cd in archive.central_dir_entry.iter() {
            // console_log!("{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} ",
            //        cd.signature, cd.version_made_by, cd.version_needed, cd.general_flag, cd.compression_method, cd.last_mod_time, cd.last_mod_date, cd.crc,
            //        cd.compressed_size, cd.un_compressed_size, cd.file_name_length, cd.extra_field_length, cd.file_comment_length, cd.disk_number_start,
            //        cd.internal_file_attributes, cd.external_file_attributes, cd.relative_offset_of_local_header);
            zip.extend(cd.signature.to_be_bytes().iter());
            zip.extend(cd.version_made_by.to_be_bytes().iter());
            zip.extend(cd.version_needed.to_be_bytes().iter());
            zip.extend(cd.general_flag.to_be_bytes().iter());
            zip.extend(cd.compression_method.to_be_bytes().iter());
            zip.extend(cd.last_mod_time.to_be_bytes().iter());
            zip.extend(cd.last_mod_date.to_be_bytes().iter());
            zip.extend(cd.crc.to_be_bytes().iter());
            zip.extend(cd.compressed_size.to_be_bytes().iter());
            zip.extend(cd.un_compressed_size.to_be_bytes().iter());
            zip.extend(cd.file_name_length.to_be_bytes().iter());
            zip.extend(cd.extra_field_length.to_be_bytes().iter());
            zip.extend(cd.file_comment_length.to_be_bytes().iter());
            zip.extend(cd.disk_number_start.to_be_bytes().iter());
            zip.extend(cd.internal_file_attributes.to_be_bytes().iter());
            zip.extend(cd.external_file_attributes.to_be_bytes().iter());
            zip.extend(cd.relative_offset_of_local_header.to_be_bytes().iter());
            for byte in &cd.file_name {
                // console_log!("{:x} ", byte);
                zip.push(*byte);
            }
        }
        let eocd = &archive.end_of_cd;
        // console_log!("{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x} ",
        //        eocd.signature, eocd.num_of_disk, eocd.disk_where_cd_star, eocd.num_of_cd_on_disk, eocd.num_of_cd, eocd.size_of_cd, eocd.offset_of_cd, eocd.zip_comment_length);
        zip.extend(eocd.signature.to_be_bytes().iter());
        zip.extend(eocd.num_of_disk.to_be_bytes().iter());
        zip.extend(eocd.disk_where_cd_start.to_be_bytes().iter());
        zip.extend(eocd.num_of_cd_on_disk.to_be_bytes().iter());
        zip.extend(eocd.num_of_cd.to_be_bytes().iter());
        zip.extend(eocd.size_of_cd.to_be_bytes().iter());
        zip.extend(eocd.offset_of_cd.to_be_bytes().iter());
        zip.extend(eocd.zip_comment_length.to_be_bytes().iter());
        zip
    }

    fn make_local_header(is_zip64: bool, is_compress: bool,  file: &File) -> LocalHeader {
        LocalHeader {
            signature: byteorder::le_from_u32(LOCAL_HEADER_SIGNATURE),
            version_needed: byteorder::le_from_u16( if is_zip64 { 0x002e } else { 0x0014 }),
            general_flag: byteorder::le_from_u16(0x0001),
            compression_method: byteorder::le_from_u16(if is_compress { 0x0008} else { 0x0000 }),
            last_mod_time: byteorder::le_from_u16(file.last_mod_time),
            last_mod_date: byteorder::le_from_u16(file.last_mod_date),
            crc: byteorder::le_from_u32(file.crc),
            compressed_size: byteorder::le_from_u32(file.encrypt.len() as u32),
            un_compressed_size: byteorder::le_from_u32(file.file_size),
            file_name_length: byteorder::le_from_u16(file.file_name.len() as u16),
            extra_field_length: byteorder::le_from_u16(0x0000),
            file_name: Vec::from(file.file_name.clone()),
            extra_field: vec![]
        }
    }

    fn make_central_directory_entry(is_zip64: bool, local_header: &LocalHeader, offset: u32) -> CentralDirectoryEntry {
        CentralDirectoryEntry {
            signature: byteorder::le_from_u32(CENTRAL_DIRECTORY_ENTRY_SIGNATURE),
            version_made_by: byteorder::le_from_u16(0x003f),
            version_needed: local_header.version_needed,
            general_flag: local_header.general_flag,
            compression_method:local_header.compression_method,
            last_mod_time: local_header.last_mod_time,
            last_mod_date: local_header.last_mod_date,
            crc: local_header.crc,
            compressed_size: local_header.compressed_size,
            un_compressed_size: local_header.un_compressed_size,
            file_name_length: local_header.file_name_length,
            extra_field_length: local_header.extra_field_length,
            file_comment_length: byteorder::le_from_u16(0x0000),
            disk_number_start: byteorder::le_from_u16(if is_zip64 { 0xffff } else { 0x0000 }),
            internal_file_attributes: byteorder::le_from_u16(0x0000),
            external_file_attributes: byteorder::le_from_u32(0x00000000),
            relative_offset_of_local_header: byteorder::le_from_u32(if is_zip64 { 0xffffffff } else { offset }),
            file_name: local_header.file_name.clone(),
            extra_field: vec![],
            file_comment: vec![]
        }
    }

    fn make_end_of_central_directory(is_zip64:bool, file_entry: &Vec<FileEntry>, central_dir_entry: &Vec<CentralDirectoryEntry>) -> EndOfCentralDirectory {
        let mut size_of_cd: u32 = 0;
        for cd in central_dir_entry {
            size_of_cd += (46 + byteorder::le_from_u16(cd.file_name_length) + cd.extra_field_length + cd.file_comment_length) as u32;
        };

        let mut offset_of_cd: u32 = 0;
        for fe in file_entry {
            offset_of_cd += 30 + (byteorder::le_from_u16(fe.local_header.file_name_length) + fe.local_header.extra_field_length) as u32  + fe.body.len() as u32;
        }

        EndOfCentralDirectory {
            signature: byteorder::le_from_u32(END_OF_CENTRAL_DIRECTORY_SIGNATURE),
            num_of_disk: byteorder::le_from_u16(if is_zip64 { 0xffff } else { 0x0000 }),
            disk_where_cd_start: byteorder::le_from_u16(if is_zip64 { 0xffff } else { 0x0000 }),
            num_of_cd_on_disk: byteorder::le_from_u16(if is_zip64 { 0xffff } else { central_dir_entry.len() as u16 }),
            num_of_cd: byteorder::le_from_u16(if is_zip64 { 0xffff } else { central_dir_entry.len() as u16 }),
            size_of_cd: byteorder::le_from_u32(if is_zip64 { 0xffffffff } else { size_of_cd }),
            offset_of_cd: byteorder::le_from_u32(if is_zip64 { 0xffffffff } else { offset_of_cd }),
            zip_comment_length: byteorder::le_from_u16(0x0000),
            zip_comment: vec![]
        }
    }
}