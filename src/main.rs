use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process;

mod types;

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

    println!("rom {} is {} KB long", rom_filename, rom.len() / 1024);
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
