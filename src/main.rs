extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;

use std::fs::File;
use std::io::Read;
use std::time::Duration;


use cpu::*;
use c64mem::*;
use memory::*;
use vic::*;

mod cpu;
mod opcode;
mod memory;
mod c64mem;
mod vic;

pub struct Emu {
    cpu: Cpu,
    mem: C64Mem
}

impl Emu {
    pub fn new() -> Self {
        let cpu = Cpu::new();
        let mem = C64Mem::new();
    
        Self {
            cpu: cpu,
            mem: mem
        }
    }

    pub fn start(&mut self, pc_addr: u16) {
        let (basic, chargen, kernal) = (
            self.load_file("rom/basic.bin"),
            self.load_file("rom/characters.bin"),
            self.load_file("rom/kernal.bin")
        );

        let test_rom = vec! [
            0xa9, 0x64, 0xe9, 0x01, 0xc9, 0x00, 0xd0, 0xfa, 0x00
        ];

        self.mem.load_rom(&test_rom, 0xE000);
        self.cpu.set_pc(pc_addr);
        self.cpu.set_sp(0xFF);

        let mut vic = Vic::new();
        vic.init(&mut self.mem);
    }

    pub fn load_file(&mut self, file_name: &str) -> Vec<u8> {
        let mut file = File::open(file_name).unwrap();
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data).expect("File Not Exsist");
        data
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Malmmodore 64", 800, 600)
        .position_centered()
        .build()
        .unwrap();
        
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
        
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut rng = rand::thread_rng();

    let mut emu = Emu::new();
    emu.start(0xE000);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let _ = emu.cpu.execute_instructions(&mut emu.mem);
        ::std::thread::sleep(Duration::new(0, 70_000));
    }

    println!("Hello, world!");
}
