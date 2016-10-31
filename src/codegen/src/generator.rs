use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use phf_codegen;


/// Generate the table of Unicode East Asian Width,
/// this table will be included in the later compilation
pub fn generate_width_table() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("east_asian_width_table.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut f = File::open("data/EastAsianWidth.txt").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();

    write!(&mut file, "static EAST_ASIAN_WIDTH_TABLE: phf::Map<u32, EastAsianWidth> = ").unwrap();

    let mut codegen = phf_codegen::Map::new();

    for (c, w) in data.lines()
                      .filter(|&s| !s.starts_with("#"))   // comments
                      .map(str::trim)
                      .filter(|&s| !s.is_empty())         // empty lines
                      .map(|s| {
                          let mut tmp = s.splitn(2, ';');
                          (tmp.next().unwrap(),
                           tmp.next().unwrap()
                              .splitn(2, ' ').take(1).next().unwrap())
                      }) {

        if c.contains("..") {
            let mut tmp = c.splitn(2, "..");
            let (start, end) = (tmp.next().unwrap(), tmp.next().unwrap());
            let (start, end) = (u32::from_str_radix(start, 16).unwrap(),
                                u32::from_str_radix(end, 16).unwrap());
            for i in start..end {
                codegen.entry(i,
                              format!("EastAsianWidth::{}", w).as_str());
            }
        } else {
            codegen.entry(u32::from_str_radix(c, 16).unwrap(),
                          format!("EastAsianWidth::{}", w).as_str());
        }
    }

    codegen.build(&mut file).unwrap();

    write!(&mut file, ";\n").unwrap();
}

pub fn generate_unicode_version() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("version.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    write!(&mut file, "pub const UNICODE_VERSION: (u64, u64, u64) = (9, 0, 0);").unwrap();
}
