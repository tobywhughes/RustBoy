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

use piston_window::*;
use image::ImageBuffer;
use image::{RgbaImage, Rgba};
use opengl_graphics::Texture;
use piston::event_loop::*;
use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::fs::File;
use std::io::prelude::*;
use std::env;
use cpu::cpu::*;
use gpu::gpu::*;
use gpu::gpu_registers::*;
use mmu::*;
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
    system_data.mmu.mem_map = read_gb_file(file_name);
    let mut registers: Registers = Registers::new();
    let mut gpu_registers: GPU_Registers = GPU_Registers::new();
    init_emulator_state(&mut system_data, &mut registers);

    //Initialize Screen
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("RustBoy", [system_data.width as u32, system_data.height as u32])
                                        .opengl(opengl)
                                        .exit_on_esc(true)
                                        .build()
                                        .unwrap();
    //window.set_max_fps(60);
    let mut app = App
    {
        gl: GlGraphics::new(opengl),
    };
    let mut events = Events::new(EventSettings::new().max_fps(60));
    //events.set_max_fps(60);
    let mut background_tile_map: TileMap = TileMap::new();
    let mut window_tile_map: TileMap = TileMap::new();
  
    //Operation loop
    let mut emulator_loop = true;

    while let Some(e) = events.next(&mut window)
    {
        if let Some(r) = e.update_args()
        {
            while !gpu_registers.v_blank_draw_flag
            {   //Default until joypad implementation
                system_data.mmu.set_to_memory(0xFF00, 0xFF, false); //system_data.mem_map[0xFF00] | 0x0F;
                //^^^^ Delete when joypad implemented ^^^^^^
                let opcode = system_data.mmu.get_from_memory(registers.program_counter as usize, false);
                let address = registers.program_counter;
                cpu_continue(&mut system_data, &mut registers);
                update_gpu(&mut system_data, &mut registers, &mut gpu_registers);     
            }
        }

        if let Some(r) = e.render_args(){
            gpu_registers.v_blank_draw_flag = false;
            background_tile_map.populate_tile_map(&mut system_data, gpu_registers.lcdc_register.tile_data, gpu_registers.lcdc_register.background_display_select);  
            window_tile_map.populate_tile_map(&mut system_data, gpu_registers.lcdc_register.tile_data, gpu_registers.lcdc_register.window_display_select);
            //let background_image: RgbaImage = create_background_img(&background_tile_map);
            let background_image: RgbaImage = create_background_img(&window_tile_map);
            app.render(&background_image, &r);
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
    fn render(&mut self, img: &RgbaImage, args: &RenderArgs)
    {
            use graphics::*;
            let BLANK: types::Color = color::hex("9CBD0F");
            let tile = Texture::from_image(&img, &TextureSettings::new());
            
            self.gl.draw(args.viewport(), |c, gl| 
            {
                clear(color::BLACK, gl);
                let transform = c.transform.trans(0.0,0.0);
                image(&tile, transform, gl);
            });
    }
}

fn read_gb_file(file_name: &str) -> Vec<u8>
{
    let mut buffer : Vec<u8> = vec![0; 0x10000];
    let file = File::open(file_name);
    if file.is_ok()
    {
        if file.unwrap().read(&mut buffer).is_ok()
        {
            return buffer;
        }        
    }
    return buffer;
}

fn create_background_img(background_tile_map: &TileMap) -> RgbaImage
{

    let mut buffer = ImageBuffer::new(256, 256);
    for tile_y in 0..32
    {
        for tile_x in 0..32
        {
            for pixel_y in 0..8
            {
                for pixel_x in 0..8
                {
                    let tile = background_tile_map.map[(tile_y * 32) + tile_x];
                    let pixel_data = background_tile_map.tiles[tile as usize].data[(pixel_y * 8) + pixel_x];
                    let pixel = pixel_color_map(pixel_data);
                    buffer.put_pixel(((tile_x * 8) + pixel_x) as u32, ((tile_y * 8) + pixel_y) as u32, pixel);
                }
            }
        }
    }
    return buffer;
}

fn pixel_color_map(pixel_data: u8) -> Rgba<u8>
{
    match pixel_data 
    {
        0 => return Rgba([156,189,15, 0xFF]),
        1 => return Rgba([140,173,15, 0xFF]),
        2 => return Rgba([48,98,48, 0xFF]),
        3 => return Rgba([15, 56, 15, 0xFF]),
        _ => return Rgba([0, 0, 0, 0xFF]),
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


#[cfg(test)]
mod main_tests
{
    use read_gb_file;

    #[test]
    fn passing_bad_filename_to_read_gb_file_return_empty_vec()
    {
        let return_vector : Vec<u8> = read_gb_file("");
        assert_eq!(return_vector, vec![0;0x10000]);
    }
}
