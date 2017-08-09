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
pub fn parse_opcode(system_data: &SystemData, registers: &mut Registers) -> u8
{
    let mut cycles: u8 = 0;
    let opcode: u8 = system_data.mem_map[registers.program_counter as usize];
    println!("{:x}", opcode);
    if (opcode & 0b11000111) == 0b00000100
    {
        if (opcode & 0b00111000) == 0b00111000 {
            registers.accumulator += 1;
            registers.program_counter += 1;
            cycles = 1;
        }
        else
        {
            println!("No Opcode Found");
        }
    }
    else if (opcode & 0b11001111) == 0b00000001
    {
        if (opcode & 0b00110000) == 0b00110000 
        {
            registers.stack_pointer = system_data.mem_map[(registers.program_counter + 1) as usize] as u16 | (system_data.mem_map[(registers.program_counter + 2) as usize] as u16) << 8;
            registers.program_counter += 3;
            cycles = 2;
        }
        else 
        {
            println!("No Opcode Found");
        }
    }
    else if (opcode & 0b11111000) == 0b10101000
    {
        if (opcode & 0b00000111) == 0b00000111 
        {
            ;
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