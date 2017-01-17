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

        let rom_type = self.get_rom_type(&rom);
        println!("cart is a {:?}", rom_type);

        // Load rom into memory.
        match rom_type {
            rom::CartType::LoROM => {
                for offset in 0 .. rom.len() {
                    self.ram[0x80_000 + offset] = rom[offset];
                }

                let cart_info = self.get_cart_info(
                    &rom, LOROM_CART_INFO_START
                );
                self.cpu.pc = cart_info.reset_vector;
            },
            _ => println!("rom type {:?} not handled", rom_type),
        }
    }

    fn get_cart_info(&self, rom: &Vec<u8>, addr: usize)
        -> rom::CartridgeInfo
    {
        rom::CartridgeInfo::from(
            &rom[addr .. addr + mem::size_of::<rom::CartridgeInfo>()]
        )
    }

    fn get_rom_type(&self, rom: &Vec<u8>) -> rom::CartType {
        // Fetch LoROM and HiROM cartridge infos.
        let lorom_info = self.get_cart_info(&rom, LOROM_CART_INFO_START);
        let hirom_info = self.get_cart_info(&rom, HIROM_CART_INFO_START);

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
        loop {
            try!(self.cpu.run_instruction());
        }
        Ok(())
    }

    fn reset(&mut self) {
        // TODO
    }
}

struct CPU {
    acc: u16,
    index_x: u16,
    index_y: u16,
    stack_ptr: u16,
    dbr: u8,
    direct: u16,
    pbr: u8,
    pc: u16,

    // flags
    flag_carry: bool,
    flag_negative: bool,
    flag_overflow: bool,
    flag_zero: bool,
    flag_decimal: bool,
    flag_irq_disable: bool,
    flag_mem_acc: bool,
    flag_index: bool,
    flag_emulation: bool,
    flag_break: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            acc: 0,
            index_x: 0,
            index_y: 0,
            stack_ptr: 0,
            dbr: 0,
            direct: 0,
            pbr: 0,
            pc: 0,

            // flags
            flag_carry: false,
            flag_negative: false,
            flag_overflow: false,
            flag_zero: false,
            flag_decimal: false,
            flag_irq_disable: false,
            flag_mem_acc: false,
            flag_index: false,
            flag_emulation: false,
            flag_break: false,
        }
    }

    pub fn run_instruction(&mut self) -> Result<(), Box<Error>> {
        // TODO
        Ok(())
    }
}
