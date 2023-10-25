use crate::memory::*;

const IO: u16 = 1;
const DATA_DIR: u16 = 0;

const LOW: u8 = 1;
const CHAR: u8 = 2;
const HIGH: u8 = 4;

#[derive(Clone, Copy)]
pub struct C64Mem {
   pub ram: [u8; 0x10000],
   rom: [u8; 0x10000],
   io: [u8; 0x1000], 
}

impl C64Mem {
   pub fn new() -> Self {
      let mut ram = [0u8; 0x10000];
      ram[DATA_DIR as usize] = 0b00101111;
      ram[IO as usize] = 0b00110111;

      let rom = [0u8; 0x10000];
      let mut io = [0u8; 0x1000];

      Self {
         ram: ram,
         rom: rom,
         io: io
      }
   }

   pub fn load_ram(&mut self, data: &Vec<u8>, address: u16) {
      for i in 0..data.len() {
         self.ram[address as usize + i] = data[i];
      }
   }

   pub fn load_rom(&mut self, data: &Vec<u8>, address: u16) {
      for i in 0..data.len() {
         self.rom[address as usize + i] = data[i];
      }
   }
}

impl IOMem for C64Mem {
   fn read_u8(&self, addr: u16) -> u8 {
      match addr {
         DATA_DIR        => self.ram[DATA_DIR as usize],
         IO              => self.ram[IO as usize],
         0xA000..=0xBFFF => if (self.ram[IO as usize] & (1 << 0)) > 0 { self.rom[addr as usize] } else { self.ram[addr as usize] },
         0xE000..=0xFFFF => if (self.ram[IO as usize] & (1 << 1)) > 0 { self.rom[addr as usize] } else { self.ram[addr as usize] },
         0xD000..=0xDFFF => if (self.ram[IO as usize] & (1 << 2)) > 0 { self.io[addr as usize - 0xD000] } else { self.ram[addr as usize] },
         _               => self.ram[addr as usize]
      }
   }

   fn read_u16(&self, addr: u16) -> u16 {
      let low = self.read_u8(addr) as u16;
      let high = self.read_u8(addr+1) as u16;
      (high << 8) | low
   }

   fn write_u8(&mut self, addr: u16, value: u8) {
      match addr {
         DATA_DIR        => self.ram[DATA_DIR as usize] = value,
         IO              => self.ram[IO as usize] = value,
         0xD000..=0xDFFF => if (self.ram[IO as usize] & (1 << 2)) > 0 { self.io[addr as usize - 0xD000] = value } else { self.ram[addr as usize] = value },
         _               => self.ram[addr as usize] = value
      }
   }
}

impl IOCia for C64Mem {
   fn read(&self, address: u16) -> u8 {
      match address {
         0xDC00..=0xDDFF => self.io[address as usize - 0xD000],
         _               => panic!("CIA-II READ Out Of Range: {:#X}", address)
      }
   }

   fn write(&mut self, address: u16, value: u8) {
      match address {
         0xDC00..=0xDDFF => self.io[address as usize - 0xD000] = value,
         _               => panic!("CIA-II WRITE Out Of Range: {:#X}", address)
      }
   }
}

impl IOVic for C64Mem {
   fn read(&self, addr: u16) -> u8 {
      match addr {
         0xD000..=0xD02E => self.io[addr as usize - 0xD000],
         _               => panic!("VIC REGISTER READ Out Of Range: {:#X}", addr)   
      }
   }

   fn write(&mut self, addr: u16, value: u8) {
      match addr {
         0xD000..=0xD02E => self.io[addr as usize - 0xD000] = value,
         _               => panic!("VIC REGISTER WRITE Out Of Range: {:#X}", addr)  
      }
   }
}