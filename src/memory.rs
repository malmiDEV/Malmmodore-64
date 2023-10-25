pub trait IOMem {
   fn read_u8(&self, addr: u16) -> u8;
   fn read_u16(&self, addr: u16) -> u16;
   fn write_u8(&mut self, addr: u16, value: u8);
}

pub trait IOCia {
   fn read(&self, address: u16) -> u8;
   fn write(&mut self, address: u16, value: u8);
}

pub trait IOVic {
   fn read(&self, addr: u16) -> u8;
   fn write(&mut self, addr: u16, value: u8);
}
