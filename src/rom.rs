use std::fmt;
use std::mem;

const CARTRIDGE_INFO_SIZE: usize = 36;
#[test]
fn test_cartridge_info_size() {
    assert_eq!(CARTRIDGE_INFO_SIZE, mem::size_of::<CartridgeInfo>());
}

#[derive(Debug)]
pub enum CartType {
    LoROM,
    HiROM,
    // TODO: support other rom types
}

pub struct CartridgeInfo {
    pub title: [u8; 21],
    pub rom_makeup: u8,
    pub rom_type: u8,
    pub rom_size: u8,
    pub sram_size: u8,
    pub country: u8,
    pub license: u8,
    pub game_version: u8,
    pub inverse_rom_checksum: u16,
    pub rom_checksum: u16,
    pub nmi_vbl_vector: u16,
    pub reset_vector: u16,
}

impl CartridgeInfo {
    pub fn from(raw_memory: &[u8]) -> CartridgeInfo {
        let data = array_ref!(raw_memory, 0, CARTRIDGE_INFO_SIZE);
        unsafe {
            mem::transmute(*data.clone())
        }
    }

    pub fn checksum_is_valid(&self) -> bool {
        (self.inverse_rom_checksum ^ self.rom_checksum) == 0xffff
    }
}

impl fmt::Debug for CartridgeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CartridgeInfo")
            .field("title", &String::from_utf8_lossy(&self.title))
            .field("rom_makeup", &self.rom_makeup)
            .field("rom_type", &self.rom_type)
            .field("rom_size", &self.rom_size)
            .field("sram_size", &self.sram_size)
            .field("country", &self.country)
            .field("license", &self.license)
            .field("game_version", &self.game_version)
            .field("inverse_rom_checksum", &self.inverse_rom_checksum)
            .field("rom_checksum", &self.rom_checksum)
            .field("nmi_vbl_vector", &self.nmi_vbl_vector)
            .field("reset_vector", &self.reset_vector)
            .finish()
    }
}
