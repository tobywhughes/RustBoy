use system::SystemData;

pub struct Registers 
{
    pub accumulator:u8,
    pub flags:u8,  
    pub b_register:u8,  
    pub c_register:u8,  
    pub d_register:u8,
    pub e_register:u8,
    pub h_register:u8,
    pub l_register:u8,
    pub stack_pointer: u16,
    pub program_counter: u16, 
}

// Returns clock cycles passed during opcode
pub fn parse_opcode(system_data: &mut SystemData, registers: &mut Registers) -> u8
{
    let mut cycles: u8 = 0;
    let opcode: u8 = system_data.mem_map[registers.program_counter as usize];
    println!("{:x}", opcode);

    //inc
    if (opcode & 0xC7) == 0x04
    {
        if (opcode & 0x38) == 0x38{
            registers.accumulator += 1;
            registers.program_counter += 1;
            cycles = 1;
        }
        else
        {
            println!("No Opcode Found");
        }
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

pub fn init_registers() -> Registers
{
    return Registers 
    {
        accumulator: 0,
        flags: 0,  
        b_register:0,  
        c_register:0,  
        d_register:0,
        e_register:0,
        h_register:0,
        l_register:0,
        stack_pointer: 0,
        program_counter: 0,  
    };
}