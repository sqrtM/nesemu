use serde::{Deserialize, Serialize};

use nesemu_core::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct CpuMemory {
    #[serde(with = "array_serde")]
    main_ram: [u8; 0x0800],
    #[serde(with = "array_serde")]
    main_ram_mirror: [u8; 0x1800],
    #[serde(with = "array_serde")]
    ppu_registers: [u8; 0x0008],
    #[serde(with = "array_serde")]
    ppu_mirrors: [u8; 0x1FF8],
    #[serde(with = "array_serde")]
    apu_io_registers: [u8; 0x0018],
    #[serde(with = "array_serde")]
    apu_io_expansion: [u8; 0x0008],
    #[serde(with = "array_serde")]
    cartridge_space: [u8; 0xBFE0],
}

mod array_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(array: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        array.serialize(serializer)
    }

    pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<u8> = Vec::deserialize(deserializer)?;
        if vec.len() == N {
            let mut array = [0u8; N];
            array.copy_from_slice(&vec);
            Ok(array)
        } else {
            Err(serde::de::Error::custom(format!(
                "Expected array of length {}, found {}",
                N,
                vec.len()
            )))
        }
    }
}

impl Read for CpuMemory {
    fn read(&self, address: u16, _read_only: bool) -> u8 {
        match address {
            0x0000..=0x07FF => self.main_ram[address as usize & 0x07FF],
            0x0800..=0x1FFF => self.main_ram_mirror[address as usize & 0x17FF],
            0x2000..=0x2007 => self.ppu_registers[address as usize & 0x0006],
            0x2008..=0x3FFF => self.ppu_mirrors[address as usize & 0x1FF7],
            0x4000..=0x4017 => self.apu_io_registers[address as usize & 0x0016],
            0x4018..=0x401F => self.apu_io_expansion[address as usize & 0x0006],
            0x4020..=0xFFFF => self.cartridge_space[address as usize & 0xBFDE],
            _ => 0x0000,
        }
    }
}

impl Write for CpuMemory {
    fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x07FF => self.main_ram[address as usize & 0x07FF] = data,
            0x0800..=0x1FFF => self.ppu_registers[address as usize & 0x0007] = data,
            0x2000..=0x2007 => self.ppu_mirrors[address as usize & 0x0017] = data,
            0x2008..=0x3FFF => self.ppu_mirrors[address as usize & 0x1FF7] = data,
            0x4000..=0x4017 => self.apu_io_registers[address as usize & 0x0016] = data,
            0x4018..=0x401F => self.apu_io_expansion[address as usize & 0x0006] = data,
            0x4020..=0xFFFF => self.cartridge_space[address as usize & 0xBFDE] = data,
            _ => {}
        }
    }
}

impl CpuMemory {
    pub fn new() -> Self {
        Self {
            main_ram: [0; 0x0800],
            main_ram_mirror: [0; 0x1800],
            ppu_registers: [0; 0x0008],
            ppu_mirrors: [0; 0x1FF8],
            apu_io_registers: [0; 0x0018],
            apu_io_expansion: [0; 0x0008],
            cartridge_space: [0; 0xBFE0],
        }
    }

    pub fn main_ram(&self) -> &[u8; 2048] {
        &self.main_ram
    }

    pub fn main_ram_mirror(&self) -> &[u8; 6144] {
        &self.main_ram_mirror
    }

    pub fn ppu_registers(&self) -> &[u8; 8] {
        &self.ppu_registers
    }

    pub fn ppu_mirrors(&self) -> &[u8; 8184] {
        &self.ppu_mirrors
    }

    pub fn apu_io_registers(&self) -> &[u8; 24] {
        &self.apu_io_registers
    }

    pub fn apu_io_expansion(&self) -> &[u8; 8] {
        &self.apu_io_expansion
    }

    pub fn cartridge_space(&self) -> &[u8; 49120] {
        &self.cartridge_space
    }
}
