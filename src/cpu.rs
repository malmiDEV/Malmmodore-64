use crate::opcode::*;
use crate::memory::*;

const STACK_POINTER : u16 = 0x100;

const NEGATIV       : u8  = 1 << 7;
const OVERFLW       : u8  = 1 << 6;
const BREAK         : u8  = 1 << 4;
const DECIMAL       : u8  = 1 << 3;
const INTERRUPT     : u8  = 1 << 2;
const ZERO          : u8  = 1 << 1;
const CARRY         : u8  = 1 << 0;

pub struct Cpu {
   pc: u16,
   sp: u16,
   a: u8,
   x: u8,
   y: u8,
   status: u8
}

impl Cpu {
   pub fn new() -> Self {
      Self {
         pc: 0,
         sp: 0,
         a: 0,
         x: 0,
         y: 0,
         status: 0b0010000
      }
   }

   pub fn set_pc(&mut self, address: u16) {
      self.pc = address;
   }

   pub fn set_sp(&mut self, address: u16) {
      self.sp = address;
   }

   pub fn execute_instructions(&mut self, memory: &mut dyn IOMem) -> Result<(), String> {
      let byte = memory.read_u8(self.pc);
      let opcode = match get_opcode(byte) {
         Some(op) => op,
         None =>
            return 
               Err(
                  format!("Unsupproted Instruction: {:#X}", byte)
               )
      };
      
      self.pc += 1;
      let address = self.get_mode(memory, opcode);
      println!("ADDRMOD: {:?}, ADDR: {:#X}, INSTRUCTION: {}, OP: {:#X} BYTE: {:#X}, PC: {:#X}, A: {:#X}, X: {:#X}, Y: {:#X}, SP:{:#X}", 
         opcode.mode,
         address,
         opcode.mnemonic,
         opcode.code,
         opcode.bytes,
         self.pc - 1,
         self.a,
         self.x,
         self.y,
         self.sp);
      self.pc += (opcode.bytes - 1) as u16;
      
      match opcode.mnemonic {
         "ADC" => self.adc(memory, address),
         "AND" => self.and(memory, address),
         "ASL" => self.asl(memory, address),
         "BCC" => self.bcc(memory, address),
         "BCS" => self.bcs(memory, address),
         "BEQ" => self.beq(memory, address),
         "BIT" => self.bit(memory, address),
         "BRK" => self.brk(memory),
         "BMI" => self.bmi(memory, address),
         "BNE" => self.bne(memory, address),
         "BPL" => self.bpl(memory, address),
         "BVC" => self.bvc(memory, address),
         "BVS" => self.bvs(memory, address),
         "CLC" => self.clc(),
         "CLD" => self.cld(),
         "CLI" => self.cli(),
         "CLV" => self.clv(),
         "CMP" => self.cmp(memory, address),
         "CPX" => self.cpx(memory, address),
         "CPY" => self.cpy(memory, address),
         "DEC" => self.dec(memory, address),
         "DEX" => self.dex(),
         "DEY" => self.dey(),
         "EOR" => self.eor(memory, address),
         "INC" => self.inc(memory, address),
         "INX" => self.inx(),
         "INY" => self.iny(),
         "JMP" => self.jmp(address),
         "JSR" => self.jsr(memory, address),
         "LDA" => self.lda(memory, address),
         "LDX" => self.ldx(memory, address),
         "LDY" => self.ldy(memory, address),
         "LSR" => self.lsr(memory, address),
         "NOP" => self.nop(),
         "ORA" => self.ora(memory, address),
         "PHA" => self.pha(memory),
         "PHP" => self.php(memory),
         "PLA" => self.pla(memory),
         "PLP" => self.plp(memory),
         "ROL" => self.rol(memory, address),
         "ROR" => self.ror(memory, address),
         "RTI" => self.rti(memory),
         "RTS" => self.rts(memory),
         "SBC" => self.sbc(memory, address),
         "SEC" => self.sec(),
         "SED" => self.sed(),
         "SEI" => self.sei(),
         "STA" => self.sta(memory, address),
         "STX" => self.stx(memory, address),
         "STY" => self.sty(memory, address),
         "TAX" => self.tax(),
         "TAY" => self.tay(),
         "TSX" => self.tsx(),
         "TXA" => self.txa(),
         "TYA" => self.tya(),
         "TXS" => self.txs(),
         _     => 
            return 
               Err(
                  format!("shitty instrucction")
               )
      }

      Ok(())
   }

   fn adc(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.add_a(data);
   }
   fn and(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.a = self.a & data;
      self.set_zn(self.a);
   }
   fn asl(&mut self, memory: &mut dyn IOMem, address: u16) {
      if address == self.a as u16 {
         let data = self.a;

         if data >> 7 == 1 {
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }
   
         self.a = (data << 1) as u8;
         self.set_zn(self.a);
      } else {
         let mut data = memory.read_u8(address);

         // if 0b1000_0000 >> 7 == 0b0000_0001 
         if data >> 7 == 1 {
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }

         data = (data << 1) as u8;

         memory.write_u8(address, data);
         self.set_zn(data);
      }
   }
   fn bcc(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status != self.status | CARRY {
         self.pc = address;
      } 
   }
   fn bcs(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status == self.status | CARRY {
         self.pc = address;
      }     
   }
   fn beq(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status == self.status | ZERO {
         self.pc = address;
      } 
   }
   fn bit(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      if self.a & data == 0 {
         self.status |= ZERO;
      } else {
         self.status &= !ZERO;
      }

      if data & 0b1000_0000 > 0 {
         self.status |= NEGATIV;
      } else {
         self.status &= !NEGATIV;
      }
      
      if data & 0b0100_0000 > 0 {
         self.status |= OVERFLW;
      } else {
         self.status &= !OVERFLW;
      }
   }
   fn brk(&mut self, memory: &mut dyn IOMem) {
      self.pc += 1;
      let _pc = self.pc;
      self.push_stack_u16(memory, _pc);
      self.status |= BREAK;
      let _status = self.status;
      self.push_stack(memory, _status);
      self.status |= INTERRUPT;
      self.pc = memory.read_u16(0xFFFE);
   }
   fn bmi(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status == self.status | NEGATIV {
         self.pc = address;
      } 
   }
   fn bne(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status != self.status | ZERO {
         self.pc = address;
      } 
   }
   fn bpl(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status != self.status | NEGATIV {
         self.pc = address;
      } 
   }
   fn bvc(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status != self.status | OVERFLW {
         self.pc = address;
      } 
   }
   fn bvs(&mut self, memory: &mut dyn IOMem, address: u16) {
      if self.status == self.status | OVERFLW {
         self.pc = address;
      }
   }
   fn clc(&mut self) {
      self.status &= !CARRY;
   }
   fn cld(&mut self) {
      self.status &= !DECIMAL;
   }
   fn cli(&mut self) {
      self.status &= !INTERRUPT;
   }
   fn clv(&mut self) {
      self.status &= !OVERFLW;
   }
   fn cmp(&mut self, memory: &mut dyn IOMem, address: u16) {
      let with = memory.read_u8(address);
      if with <= self.a    {
         self.status |= CARRY;
      } else {
         self.status &= !CARRY;
      }
      
      self.set_zn(self.a.wrapping_sub(with));
   }
   fn cpx(&mut self, memory: &mut dyn IOMem, address: u16) {
      let with = memory.read_u8(address);
      if with <= self.x {
         self.status |= CARRY;
      } else {
         self.status &= !CARRY;
      }
      
      self.set_zn(self.x.wrapping_sub(with));
   }
   fn cpy(&mut self, memory: &mut dyn IOMem, address: u16) {
      let with = memory.read_u8(address);
      if with <= self.y {
         self.status |= CARRY;
      } else {
         self.status &= !CARRY;
      }
      
      self.set_zn(self.y.wrapping_sub(with));
   }  
   fn dec(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address).wrapping_sub(1);
      memory.write_u8(address, data);
      self.set_zn(data);
   }
   fn dex(&mut self) {
      self.x = self.x.wrapping_sub(1);
      self.set_zn(self.x);
   }
   fn dey(&mut self) {
      self.y = self.y.wrapping_sub(1);
      self.set_zn(self.y);
   }
   fn eor(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.a = data ^ self.a;
      self.set_zn(self.a);
   }
   fn inc(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address).wrapping_add(1);
      memory.write_u8(address, data);
      self.set_zn(data);
   }
   fn inx(&mut self) {
      self.x = self.x.wrapping_add(1);
      self.set_zn(self.x);
   }
   fn iny(&mut self) {
      self.y = self.y.wrapping_add(1);
      self.set_zn(self.y);
   }
   fn jmp(&mut self, address: u16) {
      self.pc = address;
   }
   fn jsr(&mut self, memory: &mut dyn IOMem, address: u16) {
      self.push_stack_u16(memory, self.pc + 1);
      let addr = address;
      self.pc = addr;
   }
   fn lda(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.a = data;
      self.set_zn(self.a); 
   }
   fn ldx(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.x = data;
      self.set_zn(self.x);
   }
   fn ldy(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.y = data;
      self.set_zn(self.y);
   }
   fn lsr(&mut self, memory: &mut dyn IOMem, address: u16) {
      if address == self.a as u16 {
         let data = self.a;

         if data & 1 == 1 {
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }
   
         self.a = (data >> 1) as u8;
         self.set_zn(self.a);
      } else {
         let mut data = memory.read_u8(address);

         // if 0b1000_0000 >> 7 == 0b0000_0001 
         if data >> 7 == 1 {
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }

         data = (data >> 1) as u8;

         memory.write_u8(address, data);
         self.set_zn(data);
      }
   }
   fn nop(&mut self) {
      // do nothing
   }
   fn ora(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.a = data | self.a;
      self.set_zn(self.a);
   }
   fn pha(&mut self, memory: &mut dyn IOMem) {
      self.push_stack(memory, self.a);   
   }
   fn php(&mut self, memory: &mut dyn IOMem) {
      self.push_stack(memory, self.status);
   }
   fn pla(&mut self, memory: &mut dyn IOMem) {
      self.a = self.pop_stack(memory);
      self.set_zn(self.a);
   }
   fn plp(&mut self, memory: &mut dyn IOMem) {
      self.status = self.pop_stack(memory);
   }
   fn rol(&mut self, memory: &mut dyn IOMem, address: u16) {
      if address == self.a as u16 {
         let mut data = self.a;
         let old_carry = if self.status == self.status | CARRY {
            true
         } else {
            false
         };
   
         if data >> 7 == 1 {
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }
   
         data = data << 1;
         if old_carry {
            data = data | 1;
         }
   
         self.a = data;
         self.set_zn(self.a);
      } else {
         let mut data = memory.read_u8(address);
         let old_carry = if self.status == self.status | CARRY {
            true
         } else {
            false
         };

         if data >> 7 == 1{
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }

         data = data << 1;
         if old_carry {
            data = data | 1;
         }
         
         memory.write_u8(address, data);

         if data >> 7 == 1{
            self.status |= NEGATIV;
         } else {
            self.status &= !NEGATIV;
         }
      }
   }
   fn ror(&mut self, memory: &mut dyn IOMem, address: u16) {
      if address == self.a as u16 {
         let mut data = self.a;
         let old_carry = if self.status == self.status | CARRY {
            true
         } else {
            false
         };

         if data & 1 == 1 {
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }

         data = data >> 1;
         if old_carry {
            data = data | 1;
         }

         self.a = data;
         self.set_zn(self.a);
      } else {
         let mut data = memory.read_u8(address);
         let old_carry = if self.status == self.status | CARRY {
            true
         } else {
            false
         };

         if data >> 7 == 1{
            self.status |= CARRY;
         } else {
            self.status &= !CARRY;
         }

         data = data >> 1;
         if old_carry {
            data = data | 1;
         }
         
         memory.write_u8(address, data);

         if data >> 7 == 1{
            self.status |= NEGATIV;
         } else {
            self.status &= !NEGATIV;
         }
      }
   }
   fn rti(&mut self, memory: &mut dyn IOMem) {
      self.status = self.pop_stack(memory);
      self.pc = self.pop_stack_u16(memory);
   }
   fn rts(&mut self, memory: &mut dyn IOMem) {
      self.pc = self.pop_stack_u16(memory) + 1;
   }
   fn sbc(&mut self, memory: &mut dyn IOMem, address: u16) {
      let data = memory.read_u8(address);
      self.add_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
   }
   fn sec(&mut self) {
      self.status |= CARRY;  
   }
   fn sed(&mut self) {
      self.status |= DECIMAL;
   }
   fn sei(&mut self) {
      self.status |= INTERRUPT;
   }
   fn sta(&mut self, memory: &mut dyn IOMem, address: u16) {
      memory.write_u8(address, self.a);
   }
   fn stx(&mut self, memory: &mut dyn IOMem, address: u16) {
      memory.write_u8(address, self.x);
   }
   fn sty(&mut self, memory: &mut dyn IOMem, address: u16) {
      memory.write_u8(address, self.y);
   }
   fn tax(&mut self) {
      self.x = self.a;
      self.set_zn(self.x);
   }
   fn tay(&mut self) {
      self.y = self.a;
      self.set_zn(self.y);
   }
   fn tsx(&mut self) {
      self.x = self.sp as u8;
      self.set_zn(self.x);
   }
   fn txa(&mut self) {
      self.a = self.x;
      self.set_zn(self.a);
   }
   fn txs(&mut self) {
      self.sp = self.x as u16;
   }
   fn tya(&mut self) {
      self.a = self.y;
      self.set_zn(self.a);
   }

   fn add_a(&mut self, value: u8) {
      let sum = self.a as u16 
         + value as u16
         + (if self.status == self.status | CARRY {
            1
         } else {
            0
         }) as u16;

      if sum > 0xFF {
         self.status |= CARRY;
      } else {
         self.status &= !CARRY;
      }

      if (value ^ sum as u8) & (sum as u8 ^ self.a) & 0x80 != 0 {
         self.status |= OVERFLW;
      } else {
         self.status &= !OVERFLW;
      }

      self.a = sum as u8;
      self.set_zn(self.a);
   }

   fn set_zn(&mut self, value: u8) {
      if value == 0 {   
         self.status |= ZERO;
      } else {
         self.status &= !ZERO;
      }

      if value & 0x80 != 0 {
         self.status |= NEGATIV;
      } else {
         self.status &= !NEGATIV;
      }
   }

   pub fn pop_stack(&mut self, memory: &mut dyn IOMem) -> u8 {
      self.sp += 1;
      memory.read_u8(STACK_POINTER + self.sp) 
   }

   pub fn push_stack(&mut self, memory: &mut dyn IOMem, value: u8) {
      memory.write_u8(STACK_POINTER + self.sp, value);
      self.sp -= 1;
   }

   pub fn pop_stack_u16(&mut self, memory: &mut dyn IOMem) -> u16 {
      let low = self.pop_stack(memory) as u16;
      let high = self.pop_stack(memory) as u16;
      high << 8 | low
   }

   pub fn push_stack_u16(&mut self, memory: &mut dyn IOMem, value: u16) {
      let high = (value >> 8) as u8;
      let low = (value & 0xFF) as u8;
      self.push_stack(memory, high);
      self.push_stack(memory, low);
   }

   pub fn get_mode(&self, memory: &mut dyn IOMem, opcode: &Opcode) -> u16 {
      match opcode.mode {
         OpMode::Implied     => 0,
         OpMode::Immediate   => self.address_immidiate(),   
         OpMode::Absolute    => self.address_absolute(memory),
         OpMode::AbsoluteX   => self.address_absolute_x(memory),   
         OpMode::AbsoluteY   => self.address_absolute_y(memory),   
         OpMode::Indirect    => self.address_indirect(memory),
         OpMode::IndirectX   => self.address_indirect_x(memory),   
         OpMode::IndirectY   => self.address_indirect_y(memory),   
         OpMode::ZeroPage    => self.address_zeropage(memory),
         OpMode::ZeroPageX   => self.address_zeropage_x(memory),   
         OpMode::ZeroPageY   => self.address_zeropage_y(memory),   
         OpMode::Accumulator => self.address_accumulator(),   
         OpMode::Relative    => self.address_relative(memory),
      }
   }

   fn address_immidiate(&self) -> u16 {
      self.pc as u16
   }

   fn address_absolute(&self, memory: &mut dyn IOMem) -> u16 {
      memory.read_u16(self.pc) as u16
   }

   fn address_absolute_x(&self, memory: &mut dyn IOMem) -> u16 {
      let first = memory.read_u16(self.pc);
      let second = first.wrapping_add(self.x as u16);
      second
   }

   fn address_absolute_y(&self, memory: &mut dyn IOMem) -> u16 {
      let first = memory.read_u16(self.pc);
      let second = first.wrapping_add(self.y as u16);
      second
   }

   fn address_indirect(&self, memory: &mut dyn IOMem) -> u16 {
      let pointer = memory.read_u8(self.pc);
      let low = memory.read_u8(pointer as u16);
      let high = memory.read_u8(pointer.wrapping_add(1) as u16);
      (high as u16) << 8 | (low as u16)
   }

   fn address_indirect_x(&self, memory: &mut dyn IOMem) -> u16 {
      let pointer = (memory.read_u8(self.pc) as u8).wrapping_add(self.x);
      let low = memory.read_u8(pointer as u16);
      let high = memory.read_u8(pointer.wrapping_add(1) as u16);
      (high as u16) << 8 | (low as u16)
   }

   fn address_indirect_y(&self, memory: &mut dyn IOMem) -> u16 {
      let first = memory.read_u8(self.pc);
      let low = memory.read_u8(first as u16);
      let high = memory.read_u8((first as u8).wrapping_add(1) as u16);
      let bit = (high as u16) << 8 | (low as u16);
      bit.wrapping_add(self.y as u16)
   }

   fn address_zeropage(&self, memory: &mut dyn IOMem) -> u16 {            
      memory.read_u8(self.pc) as u16
   }

   fn address_zeropage_x(&self, memory: &mut dyn IOMem) -> u16 {
      let first = memory.read_u8(self.pc);
      let second = first.wrapping_add(self.x) as u16;
      second
   }

   fn address_zeropage_y(&self, memory: &mut dyn IOMem) -> u16 {
      let first = memory.read_u8(self.pc);
      let second = first.wrapping_add(self.y) as u16;
      second
   }

   fn address_accumulator(&self) -> u16 {
      self.a as u16
   }

   fn address_relative(&self, memory: &mut dyn IOMem) -> u16 {
      let jump = memory.read_u8(self.pc) as i8;
      let addr = self.pc
         .wrapping_add(1)
         .wrapping_add(jump as u16);
      addr
   }
}
