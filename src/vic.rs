const SPRITE_LOC_PTR          : u16 = 0xD000;      
const X_MSBS                  : u16 = 0xD010;
const Y_SCROLL                : u16 = 0xD011;
const RASTER_COUNTER          : u16 = 0xD012;      
const LIGHT_PEN_X             : u16 = 0xD013;   
const LIGHT_PEN_Y             : u16 = 0xD014;   
const SPRITE_ENABLE           : u16 = 0xD015;      
const X_SCROLL                : u16 = 0xD016;
const SPRITE_Y_EXPANSION      : u16 = 0xD017;            
const MEM_PTR                 : u16 = 0xD018;
const INTERRUPT_REG           : u16 = 0xD019;      
const INTERRUPT_ENABLE        : u16 = 0xD01A;         
const SPRITE_DATA_PRIORITY    : u16 = 0xD01B;            
const SPRITE_MULTY_COL        : u16 = 0xD01C;         
const SPRITE_X_EXPANSION      : u16 = 0xD01D;            
const SPRITE_SPRITE_COLLISION : u16 = 0xD01E;               
const SPRITE_DATA_COLLISION   : u16 = 0xD01F;               
const BORDER_COLOR            : u16 = 0xD020;      
const BACKGROUND_COLOR_0      : u16 = 0xD021;            
const BACKGROUND_COLOR_1      : u16 = 0xD022;            
const BACKGROUND_COLOR_2      : u16 = 0xD023;            
const BACKGROUND_COLOR_3      : u16 = 0xD024;            
const SPRITE_MULTY_COL0       : u16 = 0xD025;         
const SPRITE_MULTY_COL1       : u16 = 0xD026;         
const COLOR_SPRITE_PTR        : u16 = 0xD027;         

pub struct Vic {
   vram: Vec<u8>,
   palette: [u32; 16]
}

impl Vic {
   pub fn new() -> Self {
      Self {
         vram: vec![0; 0x4000],
         palette: [0u32; 16]
      }
   }

   pub fn init(&mut self) {
      let mut n = 0;
      for i in &[
         0x000000,
         0xffffff,
         0x68372b,
         0x70a4b2,
         0x6f3d86,
         0x588d43,
         0x352879,
         0xb8c76f,
         0x6f4f25,
         0x433900,
         0x9a6759,
         0x444444,
         0x6c6c6c,
         0x9ad284,
         0x6c5eb5,
         0x959595
      ] {
         self.palette[n] = *i;
         n += 1;
      }
   }

   fn get_rgb(&self, index: u8) -> (u8, u8, u8) {
      match index {
         0..=15 => (
            (self.palette[index as usize] >> 16) as u8 & 0xff, 
            (self.palette[index as usize] >>  8) as u8 & 0xff, 
            (self.palette[index as usize] >>  0) as u8 & 0xff, 
         ),
         _ => panic!("Color Palette Out-Of-Range: {}", index)
      }
   }

   
}