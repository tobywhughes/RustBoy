use system::SystemData;
use system::Registers;



// Returns clock system_data.cycle passed during opcode
pub fn parse_opcode(system_data_original: &mut SystemData, registers_original: &mut Registers)
{
    //Borrow splitting
    let mut system_data = system_data_original;
    let mut registers = registers_original;

    system_data.cycles = 0;
    let opcode: u8 = system_data.mem_map[registers.program_counter as usize];
    println!("{:x}: {:x}", registers.program_counter, opcode);

    //inc
    if (opcode & 0xC7) == 0x04
    {
        increment(&mut system_data, &mut registers, opcode);
    }
    //8bit ld group
    //ld r, n
    else if (opcode & 0xC7) == 0x06
    {
        load_n_to_8bit_register(&mut system_data, &mut registers, opcode);
    }
    //ld (FF00+C), A
    else if opcode == 0xE2
    {
        load_accumulator_to_io_port_with_c_offset(&mut system_data, &mut registers);
    }
    //16 bit ld group
    else if (opcode & 0xCF) == 0x01
    {
        load_nn_to_16bit_register(&mut system_data, &mut registers, opcode);
    }
    //xor
    else if (opcode & 0xF8) == 0xA8
    {
        xor_register(&mut system_data, &mut registers, opcode);
    }
    //jump nz, dis
    else if (opcode == 0x20)
    {
        jump_displacement_on_nonzero_flag(&mut system_data, &mut registers);
    }
    //LD (HL), r
    else if (opcode & 0xF8) == 0x70
    {
        load_hl_address_with_register(&mut system_data, &mut registers, opcode);
    }
    //LDD (HL), A
    else if (opcode == 0x32)
    {
        load_decrement_hl_register_location_with_accumulator(&mut system_data, &mut registers);        
    }
    //cb codes
    else if (opcode == 0xCB){
        cb_codes(&mut system_data, &mut registers);
    }
    else
    {
        println!("No Opcode Found");
    }
}

pub fn increment(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
        system_data.cycles = 1;
        if (opcode & 0x38) == 0x38{
            registers.accumulator += 1;
            registers.program_counter += 1;
        }
        else if (opcode & 0x38) == 0x00{
            registers.b_register += 1;
            registers.program_counter += 1;
        }
        else if (opcode & 0x38) == 0x08{
            registers.c_register += 1;
            registers.program_counter += 1;
        }
        else if (opcode & 0x38) == 0x10{
            registers.d_register += 1;
            registers.program_counter += 1;
        }
        else if (opcode & 0x38) == 0x18{
            registers.e_register += 1;
            registers.program_counter += 1;
        }
        else if (opcode & 0x38) == 0x20{
            registers.h_register += 1;
            registers.program_counter += 1;
        }
        else if (opcode & 0x38) == 0x28{
            registers.l_register += 1;
            registers.program_counter += 1;
        }
        else
        {
            system_data.cycles = 0;
            println!("No Opcode Found");
        }
}

pub fn load_n_to_8bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
        system_data.cycles = 2;
        if(opcode & 0x38) == 0x38
        {
            registers.accumulator = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if(opcode & 0x38) == 0x00
        {
            registers.b_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if(opcode & 0x38) == 0x08
        {
            registers.c_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if(opcode & 0x38) == 0x10
        {
            registers.d_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if(opcode & 0x38) == 0x18
        {
            registers.e_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if(opcode & 0x38) == 0x20
        {
            registers.h_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if(opcode & 0x38) == 0x28
        {
            registers.l_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else 
        {
            system_data.cycles = 0;
            println!("No Opcode Found");
        }
        registers.program_counter += 2;
}

pub fn load_accumulator_to_io_port_with_c_offset(system_data: &mut SystemData, registers: &mut Registers)
{
        system_data.cycles = 2;
        system_data.mem_map[(0xFF00 + registers.c_register as u16) as usize] = registers.accumulator;
        registers.program_counter += 1;  
}

pub fn load_nn_to_16bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8){
    system_data.cycles = 2;

        if (opcode & 0x30) == 0x30 
        {
            registers.stack_pointer = system_data.mem_map[(registers.program_counter + 1) as usize] as u16 | (system_data.mem_map[(registers.program_counter + 2) as usize] as u16) << 8;
        }
        else if (opcode & 0x30) == 0x20
        {
            registers.h_register = system_data.mem_map[(registers.program_counter + 2) as usize];
            registers.l_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if (opcode & 0x30) == 0x10
        {
            registers.d_register = system_data.mem_map[(registers.program_counter + 2) as usize];
            registers.e_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else if (opcode & 0x30) == 0x00
        {
            registers.b_register = system_data.mem_map[(registers.program_counter + 2) as usize];
            registers.c_register = system_data.mem_map[(registers.program_counter + 1) as usize];
        }
        else 
        {
            system_data.cycles = 0;
            println!("No Opcode Found");
        }

        registers.program_counter += 3;
}

pub fn xor_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
        if (opcode & 0x07) == 0x07 
        {
            registers.accumulator = registers.accumulator ^ registers.accumulator;
            registers.program_counter += 1;
            system_data.cycles = 1;
            registers.flags = 0x00;
            if  registers.accumulator ==  0 {
                registers.flags = registers.flags | 0x80;
            }
        }
        else 
        {
            println!("No Opcode Found");
        }
}

pub fn jump_displacement_on_nonzero_flag(system_data: &mut SystemData, registers: &mut Registers)
{
        if (registers.flags & 0x80) != 0x80 {
            system_data.cycles = 3;
            let pc_dest: i8 = (system_data.mem_map[(registers.program_counter + 1) as usize] + 2) as i8;
            registers.program_counter = (registers.program_counter as i32 + pc_dest as i32) as u16;
        }
        else {
            system_data.cycles = 2;
            registers.program_counter += 2;
        }
}

pub fn load_decrement_hl_register_location_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    let mut mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    system_data.mem_map[mem_loc as usize] = registers.accumulator;
    mem_loc -= 1;
    registers.l_register = (mem_loc & 0x00FF) as u8;
    registers.h_register = ((mem_loc & 0xFF00) >> 8) as u8;
    system_data.cycles = 2;
    registers.program_counter += 1;
}


pub fn load_hl_address_with_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    system_data.cycles = 2;
    if (opcode & 0x07) == 0x07 
    {
        system_data.mem_map[mem_loc as usize] = registers.accumulator;
    }
    else if (opcode & 0x07) == 0x00 
    {
        system_data.mem_map[mem_loc as usize] = registers.b_register;
    }
    else if (opcode & 0x07) == 0x01 
    {
        system_data.mem_map[mem_loc as usize] = registers.c_register;
    }
    else if (opcode & 0x07) == 0x02 
    { 
        system_data.mem_map[mem_loc as usize] = registers.d_register;
    }
    else if (opcode & 0x07) == 0x03 
    {
        system_data.mem_map[mem_loc as usize] = registers.e_register;
    }
    else if (opcode & 0x07) == 0x04 
    {
        system_data.mem_map[mem_loc as usize] = registers.h_register;
    }
    else if (opcode & 0x07) == 0x05 
    {
        system_data.mem_map[mem_loc as usize] = registers.l_register;
    }
    else 
    {
        system_data.cycles = 0;
        println!("No Opcode Found");
    }
    registers.program_counter += 1;   
}

///////////////////
//CB
///////////////////

pub fn cb_codes(system_data_original: &mut SystemData, registers_original: &mut Registers)
{
    //Borrow splitting
    let mut system_data = system_data_original;
    let mut registers = registers_original;
    let opcode :u8 = system_data.mem_map[(registers.program_counter + 1) as usize];
    //bit b, r
    if (opcode & 0xC0) == 0x40
    {
        system_data.cycles = 2;
        let test_bit: u8 = (opcode & 0x38) >> 3;
        bit_check_register(&mut system_data, &mut registers, opcode, test_bit)
    }
    else 
    {
        println!("No Opcode Found");
    }
}

pub fn bit_check_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8, test_bit: u8)
{
        if (opcode & 0x07) == 0x04
        {
            registers.flags = registers.flags & 0x10;
            
            if ((registers.h_register >> test_bit) & 0x01) == 0x00
            {
                registers.flags = registers.flags | 0xA0;
            }
            else
            {
                registers.flags = registers.flags | 0x20;
            }
        }
        else
        {
            system_data.cycles = 0;
            println!("No Opcode Found");
        }
        registers.program_counter += 2;
}