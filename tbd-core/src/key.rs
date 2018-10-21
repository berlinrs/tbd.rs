use byteorder::{ReadBytesExt, LittleEndian};

pub trait Key {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl Key for i64 {
    fn from_bytes(bytes: &[u8]) -> i64 {
        use std::io::Cursor;
        let mut rdr = Cursor::new(bytes);
        rdr.read_i64::<LittleEndian>().unwrap()
    }
}
