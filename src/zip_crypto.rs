use crate::mt;
use crate::crc;
use flate2::Compression;
use flate2::write::DeflateEncoder;
use std::io::Write;

pub struct ZipCrypto {
    seed: u32,
    key_0: u32,
    key_1: u32,
    key_2: u32,
}

impl ZipCrypto {
    pub fn new(seed: u32) -> ZipCrypto {
        ZipCrypto {
            seed,
            key_0: 0x12345678,
            key_1: 0x23456789,
            key_2: 0x34567890,
        }
    }

    pub fn deflate_encode_raw_stream(level: u32, file_raw: Vec<Vec<u8>>) -> Vec<u8> {
        let mut e = DeflateEncoder::new(Vec::new(), Compression::new(level));
        for base_raw in file_raw.iter() { e.write_all(&*base_raw).unwrap(); };
        let compressed_bytes = e.finish();
        return compressed_bytes.unwrap();
    }

    pub fn deflate_encode_raw(level: u32, file_raw: Vec<u8>) -> Vec<u8> {
        let mut e = DeflateEncoder::new(Vec::new(), Compression::new(level));
        e.write_all(&*file_raw).unwrap();
        let compressed_bytes = e.finish();
        return compressed_bytes.unwrap();
    }

    pub fn encrypt(&mut self, file_raw: &[u8], passwd: &str, crc32: u32) -> Vec<u8> {
        for byte in passwd.bytes() {
            self.update(byte);
        }
        let random_header = self.random_header(crc32);
        let encrypt_header =self.upd(random_header, true);
        let encrypt_body: Vec<u8> = self.upd(Vec::from(file_raw), true);
        let mut encrypt: Vec<u8> = vec![];
        encrypt.extend(encrypt_header);
        encrypt.extend(encrypt_body);
        encrypt
    }

    fn update(&mut self, input: u8) {
        self.key_0 = crc::crc32(self.key_0, input);
        self.key_1 = (self.key_1 + (self.key_0 & 0xff)) * 0x08088405 + 1;
        self.key_2 = crc::crc32(self.key_2, (self.key_1 >> 24) as u8);
    }

    fn decrypt_key(&mut self) -> u8 {
        let temp: u16 = (self.key_2 as u16 | 2) & 0xffff;
        ((temp * (temp ^ 1)) >> 8) as u8
    }

    fn upd(&mut self, buf: Vec<u8>, enc_type: bool) -> Vec<u8>{
        let mut out_buf: Vec<u8> = vec![];
        for mut byte in buf {
            let stream = self.decrypt_key();
            if !enc_type {
                byte ^= stream;
            }
            self.update(byte);
            if enc_type {
                byte ^= stream;
            }
            out_buf.push(byte);
        };
        out_buf
    }

    fn random_header(&mut self, crc: u32) -> Vec<u8> {
        let mut mt = mt::MersenneTwister::new(self.seed);
        let mut i: Vec<u8> = Vec::with_capacity(12);
        let mut counter = 0;

        while counter < 11 {
            counter += 1;
            i.push((mt.next() & 0xff) as u8);
        }

        i.push(((crc >> 24) & 0xff) as u8);
        i
    }
}