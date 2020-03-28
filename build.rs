extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::convert::{TryInto, TryFrom};

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut map = phf_codegen::Map::new();
    let lines = include_str!("table");
    for line in lines.split_ascii_whitespace() {
        let key: char = u32::from_str_radix(&line[..4], 16).unwrap().try_into().unwrap();
        let mut value = line[5..].split('&').map(|s| u32::from_str_radix(s, 16).unwrap()).map(|u| char::try_from(u).unwrap()).collect::<String>();
        value.insert(0, '"');
        value.push('"');
        map.entry(key, &value);
    }

    writeln!(
        &mut file,
        "static TABLE: phf::Map<char, &'static str> = \n{};\n",
        map.build()
    ).unwrap();
}