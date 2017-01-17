use std::error::Error;
use std::mem;

use rom;

const RAM_SIZE: usize = 0x100 * 0x10000;
const LOROM_CART_INFO_START: usize = 0x7fc0;
const HIROM_CART_INFO_START: usize = 0xffc0;

pub struct Arch {
    cpu: CPU,
    ram: Vec<u8>,
    // TODO
}

impl Arch {
    pub fn new() -> Arch {
        Arch {
            cpu: CPU::new(),
            ram: vec![0; RAM_SIZE],
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.reset();

        println!("rom is {} Mb long", rom.len() * 8 / (1024 * 1024));

        let cart_type = self.get_rom_type(&rom);
        println!("cart is a {:?}", cart_type);

        // TODO: put rom in correct place in memory
    }

    fn get_rom_type(&self, rom: &Vec<u8>) -> rom::CartType {
        // Fetch LoROM and HiROM cartridge infos.
        let get_info = |rom: &Vec<u8>, addr| {
            rom::CartridgeInfo::from(
                &rom[addr .. addr + mem::size_of::<rom::CartridgeInfo>()]
            )
        };
        let lorom_info = get_info(&rom, LOROM_CART_INFO_START);
        let hirom_info = get_info(&rom, HIROM_CART_INFO_START);

        // Check which one is valid.
        let lorom_valid = lorom_info.checksum_is_valid();
        let hirom_valid = hirom_info.checksum_is_valid();
        assert!(lorom_valid ^ hirom_valid);

        if lorom_valid {
            rom::CartType::LoROM
        } else {
            rom::CartType::HiROM
        }
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        // TODO
        Ok(())
    }

    fn reset(&mut self) {
        // TODO
    }
}

struct CPU {
    // TODO
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
        }
    }
}
