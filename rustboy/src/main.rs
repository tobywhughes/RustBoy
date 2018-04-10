#![allow(dead_code)]

extern crate csv;
extern crate hex;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate piston_window;

mod cpu;
mod gpu;
mod system;
mod mmu;
mod timer;

use piston_window::*;
use image::ImageBuffer;
use image::{RgbaImage, Rgba};
use opengl_graphics::Texture;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::env;
use cpu::cpu::*;
use gpu::gpu::*;
use gpu::gpu_registers::*;
use mmu::*;
use timer::*;
use system::*;
use self::hex::FromHex;

static MAX_SPRITE: u8 = 40;


fn main()
 {
    //Initialize Emulator
    let args: Vec<String> = env::args().collect();
    let emulator_type: String = String::from("CLASSIC");
    let file_name: &String = &args[1];
    let mut system_data : SystemData = get_system_data(&emulator_type);
    system_data.mmu.initialize_cartridge(file_name);
    let mut registers: Registers = Registers::new();
    let mut gpu_registers: GPU_Registers = GPU_Registers::new();
    init_emulator_state(&mut system_data, &mut registers);

    //Initialize Screen
    let opengl = OpenGL::V3_2;
    let scale_factor = 1.5;
    let mut window: PistonWindow = WindowSettings::new("RustBoy", [(system_data.width as f64 * scale_factor) as u32, (system_data.height as f64 * scale_factor) as u32])
                                        .opengl(opengl)
                                        .exit_on_esc(true)
                                        .build()
                                        .unwrap();
    //window.set_max_fps(60);
    let mut app = App
    {
        gl: GlGraphics::new(opengl),
    };
    let mut events = Events::new(EventSettings::new());
    //events.set_max_fps(60);
    let mut background_tile_map: TileMap = TileMap::new();
    let mut window_tile_map: TileMap = TileMap::new();
    let mut oam_tile_map: TileMap = TileMap::new();
    let mut oam_table = OAM_Table::new();
  
    //Operation loop
    let mut emulator_loop = true;
    gpu_registers.v_blank_draw_flag = false;

    let mut space_flag = false;

    while let Some(e) = events.next(&mut window)
    {
        if let Some(Button::Keyboard(key)) = e.press_args()
        {
            match key
            {
                Key::A => system_data.input.left = true,
                Key::D => system_data.input.right = true,
                Key::W => system_data.input.up = true,
                Key::S => system_data.input.down = true,
                Key::NumPad1 => system_data.input.a_button = true,
                Key::NumPad2 => system_data.input.b_button = true,
                Key::Return => system_data.input.start = true,
                Key::Space => system_data.input.select = true,
                _ => (),
            }
        } 
        
        if let Some(Button::Keyboard(key)) = e.release_args()
        {
            match key
            {
                Key::A => system_data.input.left = false,
                Key::D => system_data.input.right = false,
                Key::W => system_data.input.up = false,
                Key::S => system_data.input.down = false,
                Key::NumPad1 => system_data.input.a_button = false,
                Key::NumPad2 => system_data.input.b_button = false,
                Key::Return => system_data.input.start = false,
                Key::Space => system_data.input.select = false,
                _ => (),
            }
        }    


       if let Some(r) = e.update_args()
       {
            while !gpu_registers.v_blank_draw_flag
            {
                let joypad_input = system_data.input.update_input(&system_data);
                if (joypad_input & 0x0F) != 0x0F
                {
                    let current_if = system_data.mmu.get_from_memory(0xFF0F, false);
                    system_data.mmu.set_to_memory(0xFF0F, current_if | 0x10, false);
                }
                system_data.mmu.set_to_memory(0xFF00, joypad_input, false);
                let opcode = system_data.mmu.get_from_memory(registers.program_counter as usize, false);
                let address = registers.program_counter;
                cpu_continue(&mut system_data, &mut registers);
                update_gpu(&mut system_data, &mut registers, &mut gpu_registers);
                system_data.timer_tick();
            }
       }


        if let Some(r) = e.render_args(){
                gpu_registers.v_blank_draw_flag = false;
                background_tile_map.populate_tile_map(&mut system_data, gpu_registers.lcdc_register.tile_data, gpu_registers.lcdc_register.background_display_select);  
                window_tile_map.populate_tile_map(&mut system_data, gpu_registers.lcdc_register.tile_data, gpu_registers.lcdc_register.window_display_select);
                oam_tile_map.populate_tile_map(&mut system_data, true, true);
                oam_table.populate_oam_table(&system_data);
                let image: RgbaImage = create_background_img(&background_tile_map, &gpu_registers, &system_data, &oam_table, &oam_tile_map);
                app.render(&image, &r, scale_factor);
                // break;
        }
    }
        // if system_data.cycles == 0  //|| registers.program_counter == 0x6d 
        // {
        //     emulator_loop = false;
        //     println!("Location: {:04X}\tOpcode: 0x{:02X}  {:08b}", address, opcode, opcode);
        // }
    //Cleanup?
}


pub struct App
{
    gl: GlGraphics
}

impl App
{
    fn render(&mut self, img: &RgbaImage, args: &RenderArgs, scale_factor: f64)
    {
            use graphics::*;
            let BLANK: types::Color = color::hex("9CBD0F");
            let tile = Texture::from_image(&img, &TextureSettings::new());
            
            self.gl.draw(args.viewport(), |c, gl| 
            {
                clear(color::BLACK, gl);
                let transform = c.transform.trans(0.0,0.0).zoom(scale_factor);
                image(&tile, transform, gl);
            });
    }
}

fn init_emulator_state(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter = 0x100;

    let states: Vec<u8> = vec![0x00, 0x00, 0x00, 0x80, 0xBF, 0xF3, 0xBF, 0x3F, 
                               0x00, 0xBF, 0x7F, 0xFF, 0x9F, 0xBF, 0xFF, 0x00,
                               0x00, 0xBF, 0x77, 0xF3, 0xF1, 0x91, 0x00, 0x00, 
                               0x00, 0xFC, 0xFF, 0xFF, 0x00, 0x00, 0x00];

    let mem_locations: Vec<usize> = vec![0xFF05, 0xFF06, 0xFF07, 0xFF10, 0xFF11, 0xFF12, 
                                       0xFF14, 0xFF16, 0xFF17, 0xFF19, 0xFF1A, 0xFF1B, 
                                       0xFF1C, 0xFF1E, 0xFF20, 0xFF21, 0xFF22, 0xFF23, 
                                       0xFF24, 0xFF25, 0xFF26, 0xFF40, 0xFF42, 0xFF43, 
                                       0xFF45, 0xFF47, 0xFF48, 0xFF49, 0xFF4A, 0xFF4B, 0xFFFF];

    for i in 0..states.len()
    {
        system_data.mmu.set_to_memory(mem_locations[i], states[i] ,false);
    }

    let register_states: Vec<u16> = vec![0x01B0, 0x0013, 0x00D8, 0x014D, 0xFFFE];
    for i in 0..register_states.len()
    {
        registers.mapped_16_bit_register_setter(i as u8, register_states[i]);
    }
}