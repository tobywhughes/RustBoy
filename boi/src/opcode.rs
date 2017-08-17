use system::SystemData;
use system::Registers;

// Returns clock cycles passed during opcode
pub fn parse_opcode(system_data: &mut SystemData, registers: &mut Registers) -> u8
{
    let mut cycles: u8 = 0;
    let opcode: u8 = system_data.mem_map[registers.program_counter as usize];
    println!("{:x}", opcode);

    //inc
    if (opcode & 0xC7) == 0x04
    {
        cycles = 1;
        if (opcode & 0x38) == 0x38{
            registers.accumulator += 1;
            registers.program_counter += 1;
        }
        // else if (opcode & 0x38)== 
        // {

        // }
        else
        {
            cycles = 0;
            println!("No Opcode Found");
        }
    }
    //8bit ld group
    //ld r, n
    else if (opcode & 0xC7) == 0x06
    {
        cycles = 2;

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
            cycles = 0;
            println!("No Opcode Found");
        }
        registers.program_counter += 2;
    }
    //ld (FF00+C), A
    else if opcode == 0xE2
    {
        cycles = 2;
        system_data.mem_map[(0xFF00 + registers.c_register) as usize] = registers.accumulator;
        registers.program_counter += 1;  
    }
    //16 bit ld group
    else if (opcode & 0xCF) == 0x01
    {
        cycles = 2;

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
            cycles = 0;
            println!("No Opcode Found");
        }

        registers.program_counter += 3;
    }
    //xor
    else if (opcode & 0xF8) == 0xA8
    {
        if (opcode & 0x07) == 0x07 
        {
            registers.accumulator = registers.accumulator ^ registers.accumulator;
            registers.program_counter += 1;
            cycles = 1;
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
    else if (opcode == 0x20)
    {
        if (registers.flags & 0x80) != 0x80 {
            cycles = 3;
            let pc_dest: i8 = (system_data.mem_map[(registers.program_counter + 1) as usize] + 2) as i8;
            registers.program_counter = (registers.program_counter as i32 + pc_dest as i32) as u16;
        }
        else {
            cycles = 2;
            registers.program_counter += 2;
        }
    }

    //LDD (HL), A - 32
    else if (opcode == 0x32)
    {
        let mut mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
        system_data.mem_map[mem_loc as usize] = registers.accumulator;
        mem_loc -= 1;
        registers.l_register = (mem_loc & 0x0F) as u8;
        registers.h_register = ((mem_loc & 0xF0) >> 8) as u8;
        cycles = 2;
        registers.program_counter += 1;
    }
    //cb codes
    else if (opcode == 0xCB){
        let opcode_next :u8 = system_data.mem_map[(registers.program_counter + 1) as usize];
        //bit b, r
        if (opcode_next & 0xC0) == 0x40
        {
            cycles = 2;
            let test_bit: u8 = (opcode_next & 0x38) >> 3;
            if (opcode_next & 0x07) == 0x04
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
                cycles = 0;
                println!("No Opcode Found");
            }
            registers.program_counter += 2;
        }
        else 
        {
            println!("No Opcode Found");
        }
    }
    else
    {
        println!("No Opcode Found");
    }

    return cycles;
}