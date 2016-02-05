use std::io;
use std::io::{ BufReader, Error, ErrorKind };
use std::io::prelude::*;
use std::fs::File;

use util::base64::Base64Parseable;

/// Attempts to read a file and interpret its contents as Base64-encoded.
pub fn read_file_base64(file: &str) -> io::Result<Vec<u8>> {
    let f = try!(File::open(file));
    let reader = BufReader::new(f);
    let input = try!(reader.lines().collect::<io::Result<String>>());
    input.parse_base64().map_err(|msg| Error::new(ErrorKind::Other, msg))
}
