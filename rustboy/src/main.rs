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
    system_data.mem_map = read_gb_file(file_name);
    let mut registers: Registers = Registers::new();
    let mut gpu_registers: GPU_Registers = GPU_Registers::new();
  
    //Initialize Screen
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("RustBoy", [system_data.width as u32, system_data.height as u32]).opengl(opengl).exit_on_esc(true).build().unwrap();
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

        if let Some(r) = e.render_args(){
            gpu_registers.v_blank_draw_flag = false;
            background_tile_map.populate_tile_map(&mut system_data, gpu_registers.lcdc_register.tile_data, gpu_registers.lcdc_register.background_display_select);  
            window_tile_map.populate_tile_map(&mut system_data, gpu_registers.lcdc_register.tile_data, gpu_registers.lcdc_register.window_display_select);
            let background_image: RgbaImage = create_background_img(&background_tile_map);
            app.render(&background_image, &r);
            // break;
        }

        if let Some(r) = e.update_args()
        {
            let mut frame_cycles = 0;
            while gpu_registers.v_blank_draw_flag
            {
                let opcode = system_data.mem_map[registers.program_counter as usize];
                let address = registers.program_counter;
                cpu_continue(&mut system_data, &mut registers);
                update_gpu(&mut system_data, &mut registers, &mut gpu_registers);     
            }
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
            let square = rectangle::square(0.0, 0.0, 50.0);

            let tile = Texture::from_image(img, &TextureSettings::new());
            
            self.gl.draw(args.viewport(), |c, gl| 
            {
                clear(BLANK, gl);
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
    return vec![0;0];
}

fn create_tile_img(tile_data: &TileData) -> RgbaImage
{
    let mut buffer = ImageBuffer::new(8,8);
    for pixel_y in 0..8
    {
        for pixel_x in 0..8 
        {
            let pixel_data = tile_data.data[(pixel_y * 8) + pixel_x];
            let mut pixel : Rgba<u8>;
            pixel = pixel_color_map(pixel_data);
            buffer.put_pixel(pixel_x as u32, pixel_y as u32, pixel);
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
                    buffer.put_pixel(((tile_x *32 ) + pixel_x) as u32, ((tile_y) + pixel_y) as u32, pixel);
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


fn output_mem_selection(mem_map: &Vec<u8>, start: u16, end:u16)
{
    for index in start .. end
    {
        print!("{}-{:x}\t", index, mem_map[index as usize]);
    }
    print!("\n");
}


#[cfg(test)]
mod main_tests
{
    use read_gb_file;

    #[test]
    fn passing_bad_filename_to_read_gb_file_return_empty_vec()
    {
        let return_vector : Vec<u8> = read_gb_file("");
        assert_eq!(return_vector, vec![0;0]);
    }
}
