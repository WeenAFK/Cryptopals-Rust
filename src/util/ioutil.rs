use std::io;
use std::io::{ BufReader, Error, ErrorKind };
use std::io::prelude::*;
use std::fs::File;

use util::base64::Base64Parseable;
use util::hex::HexParseable;

/// Attempts to read a file and interpret its contents as Base64-encoded.
pub fn read_base64(file: &str) -> io::Result<Vec<u8>> {
    let f = try!(File::open(file));
    let reader = BufReader::new(f);
    let input = try!(reader.lines().collect::<io::Result<String>>());
    input.parse_base64().map_err(|msg| Error::new(ErrorKind::Other, msg))
}

/// Reads a file encoded in hexadecimal and returns a Vec of the lines in the file.
pub fn read_lines_hex(file: &str) -> io::Result<Vec<Vec<u8>>> {
    let f = try!(File::open(file));
    BufReader::new(f).lines()
        .map(|res| res.and_then(|s| s.parse_hex()))
        .collect::<io::Result<Vec<Vec<u8>>>>()
}

/// Creates an Err of type io::Result with the given string message and ErrorKind Other.
pub fn err<T>(msg: &str) -> io::Result<T> {
    Err(error(msg))
}

/// Creates an io::Error.
pub fn error(msg: &str) -> Error {
    Error::new(ErrorKind::Other, msg)
}
