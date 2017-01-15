#[macro_use]
extern crate arrayref;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process;

mod rom;

fn main() {
    let mut args = env::args();
    let exe_name = args.next().unwrap();

    let rom_filename: String;
    match args.next() {
        Some(f) => rom_filename = f,
        None => {
            usage(exe_name);
            process::exit(-1);
        },
    }

    let rom: Vec<u8>;
    match read_rom(&rom_filename) {
        Ok(bytes) => rom = bytes,
        Err(err) => {
            error(&*err);
            return; // unreachable
        },
    }

    println!(
        "rom {} is {} Mb long",
        rom_filename,
        rom.len() * 8 / (1024 * 1024)
    );

    let info = rom::CartridgeInfo::from(&rom[0x7fc0 .. 0x7fe4]);
    println!("{:#?}", info);
}

fn read_rom<P: AsRef<Path>>(rom_fn: P) -> Result<Vec<u8>, Box<Error>> {
    let mut f = try!(fs::File::open(rom_fn));
    let mut bytes = Vec::new();
    try!(f.read_to_end(&mut bytes));
    Ok(bytes)
}

fn usage(exe_name: String) {
    println!("usage: {} <rom_filename>", exe_name);
}

fn error(err: &Error) {
    println!("error: {}", err);
    process::exit(-1);
}
