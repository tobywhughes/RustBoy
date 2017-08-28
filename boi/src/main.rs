#![allow(dead_code)]

mod cpu;
mod gpu;
mod system;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use cpu::cpu::*;
use gpu::gpu::*;
use system::*;

static MAX_SPRITE: u8 = 40;


fn main()
 {
    //Initialize Emulator
    let args: Vec<String> = env::args().collect();
    let emulator_type: String = String::from("CLASSIC");
    let file_name: &String = &args[1];
    let mut system_data : SystemData = get_system_data(&emulator_type);
    system_data.mem_map = read_gb_file(file_name);
    let mut registers: Registers = init_registers();
    let mut gpu_registers: GPU_Registers = GPU_Registers::new();
    //Operation loop
    let mut emulator_loop = true;
    while emulator_loop
    {
        cpu_continue(&mut system_data, &mut registers);
        update_gpu(&mut system_data, &mut registers, &mut gpu_registers);
        if system_data.cycles == 0 
        {
            emulator_loop = false;
        }
    }
    //Cleanup?
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
