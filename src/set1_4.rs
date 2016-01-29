#![allow(dead_code)]

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use set1_3;
use set1_3::XorDecRes;

pub fn main() -> io::Result<()> {
    let f = try!(File::open("src/4.txt"));
    let reader = BufReader::new(f);

    let res: XorDecRes = reader.lines()
        .filter_map(|line| line.ok())
        .map(|line| set1_3::find_best(&line))
        .filter_map(|val| val.ok())
        .max_by_key(|val| val.pts).unwrap();

    println!("Best is: \"{}\" with key {} ({})", res.text.unwrap().trim(), res.key as char, res.key);
    Ok(())
}
