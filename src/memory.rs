pub trait IOMem {
   fn read_u8(&self, addr: u16) -> u8;
   fn read_u16(&self, addr: u16) -> u16;
   fn write_u8(&mut self, addr: u16, value: u8);
}
