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
    //println!("Location: {:04X}\tOpcode: 0x{:02X}  {:08b}\t\t{:x} ===== {:x}", registers.program_counter, opcode, opcode, registers.accumulator, registers.flags);


    if opcode == 0x00
    {
        no_operation(&mut system_data);
    }
    //inc
    else if (opcode & 0xC7) == 0x04
    {
        increment_8_bit_register(&mut system_data, &mut registers, opcode);
    }
    else if (opcode & 0xCF) == 0x03
    {
        increment_16_bit_register(&mut system_data, &mut registers, opcode);
    }
    //dec
    else if (opcode & 0xC7) == 0x05
    {
        decrement_8_bit_register(&mut system_data, &mut registers, opcode);
    }
    //add r or (hl)
    else if (opcode & 0xF8) == 0x80
    {
        add_8_bit(&mut system_data, &mut registers, opcode);
    }
    //compare
    else if opcode == 0xFE
    {
        compare_with_n(&mut system_data, &mut registers);
    }

    else if opcode == 0xBE
    {
        compare_with_hl_address(&mut system_data, &mut registers);
    }

    //8bit ld group
    else if (opcode & 0xC7) == 0x06
    {
        load_n_to_8bit_register(&mut system_data, &mut registers, opcode);
    }
    //0b01xxxxxx group    
    else if (opcode & 0xC0) == 0x40
    {
        //LD (HL), r
        if (opcode & 0xF8) == 0x70
        {
            load_hl_address_with_register(&mut system_data, &mut registers, opcode);
        }
        //ld r, r'
        else 
        {
            load_8_bit_register_to_register(&mut system_data, &mut registers, opcode);
        }
    }
    //ld (FF00+C), A
    else if opcode == 0xE2
    {
        load_accumulator_to_io_port_with_c_offset(&mut system_data, &mut registers);
    }
    //ld (FF00+n), A
    else if opcode == 0xE0
    {
        load_accumulator_to_io_port_with_n_offset(&mut system_data, &mut registers);
    }
    //ld A, (ff00 + n)
    else if opcode == 0xF0
    {
        load_accumulator_with_io_port_with_n_offset(&mut system_data, &mut registers);
    }
    //16 bit ld group
    else if (opcode & 0xCF) == 0x01
    {
        load_nn_to_16bit_register(&mut system_data, &mut registers, opcode);
    }
    else if opcode == 0x1A
    {
        load_accumulator_with_de_address(&mut system_data, &mut registers);
    }
    //sub r
    else if (opcode & 0xF8) == 0x90
    {
        subtract_8_bit(&mut system_data, &mut registers, opcode)
    }
    //xor
    else if (opcode & 0xF8) == 0xA8
    {
        xor_register(&mut system_data, &mut registers, opcode);
    }
    //jump nz, dis
    else if opcode == 0x20
    {
        jump_displacement_on_nonzero_flag(&mut system_data, &mut registers);
    }
    //jump z, dis
    else if opcode == 0x28
    {
        jump_displacement_on_zero_flag(&mut system_data, &mut registers);
    }
    //jump dis
    else if opcode == 0x18
    {
        jump_displacement(&mut system_data, &mut registers);
    }
    //LDD (HL), A
    else if opcode == 0x32
    {
        load_decrement_hl_register_location_with_accumulator(&mut system_data, &mut registers);        
    }
    //LDI (HL), A
    else if opcode == 0x22
    {
        load_increment_hl_register_location_with_accumulator(&mut system_data, &mut registers);        
    }
    //ld (nn), a
    else if opcode == 0xEA
    {
        load_nn_with_accumulator(&mut system_data, &mut registers);
    }
    //rla
    else if opcode == 0x17
    {
        rotate_accumulator_left_through_carry(&mut system_data, &mut registers);
    }
    //call nn
    else if opcode == 0xCD
    {
        call_nn(&mut system_data, &mut registers);
    }
    //ret
    else if opcode == 0xC9
    {
        return_from_call(&mut system_data, &mut registers);
    } 
    //push qq
    else if (opcode & 0xCF) == 0xC5
    {
        push_16_bit_register(&mut system_data, &mut registers, opcode);
    }

    //pop qq
    else if (opcode & 0xCF) == 0xC1
    {
        pop_16_bit_register(&mut system_data, &mut registers, opcode);
    }

    //cb codes
    else if opcode == 0xCB
    {
        cb_codes(&mut system_data, &mut registers);
    }
    else
    {
        println!("No Opcode Found");
    }

    system_data.cycles *= 4;
}

pub fn no_operation(system_data: &mut SystemData)
{
    system_data.cycles = 1;
}

pub fn increment_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
        system_data.cycles = 1;
        registers.flags = registers.flags & 0x10;
        let mut register_code = ((opcode & 0x38) >> 3) + 1;
        if register_code == 8 
        {
            register_code = 0;
        }
                
        if register_code == 7
        {
            system_data.cycles = 0;
            println!("No Opcode Found");
        }
        else
        {
            let mut current_register_value = registers.mapped_register_getter(register_code);
            if current_register_value == 0xFF{
                current_register_value = 0;
            }
            else {
                current_register_value += 1;
            }
            registers.mapped_register_setter(register_code, current_register_value);
            if registers.mapped_register_getter(register_code) == 0
            {
                registers.flags = registers.flags | 0x80;
            }
            else if registers.mapped_register_getter(register_code) &0x0F == 0x0
            {
                registers.flags = registers.flags | 0x20;
            }
            registers.program_counter += 1;
        }
}

pub fn increment_16_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.program_counter += 1;
    let register_code = ((opcode & 0x30) >> 4) + 1; 
    let mut current_register_value = registers.mapped_16_bit_register_getter(register_code);
    if current_register_value == 0xFFFF
    {
        current_register_value = 0;
    }    
    else
    {
        current_register_value += 1;
    }
    registers.mapped_16_bit_register_setter(register_code, current_register_value);
    system_data.cycles = 1;
}

pub fn decrement_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 1;
    registers.flags = registers.flags & 0x10;
    registers.flags = registers.flags | 0x40;
    let mut register_code = ((opcode & 0x38) >> 3) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }
            
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found");
    }
    else 
    {
        let mut current_register_value = registers.mapped_register_getter(register_code);
        if current_register_value == 0x00{
            current_register_value = 0xFF;
        }
        else {
            current_register_value -= 1;
        }
        registers.mapped_register_setter(register_code, current_register_value);
        if registers.mapped_register_getter(register_code) == 0
        {
            registers.flags = registers.flags | 0x80;
        }
        else if registers.mapped_register_getter(register_code) & 0x0F == 0x0F
        {
            registers.flags = registers.flags | 0x20;
        }
    }
    registers.program_counter += 1;
}

pub fn add_8_bit(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 1;
    registers.flags = 0x00;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8
    {
        register_code = 0;
    }

    let mut add_num;
    if register_code == 7
    {
        system_data.cycles += 1;
        add_num = system_data.mem_map[(((registers.h_register as u16) << 8)|(registers.l_register as u16)) as usize];
    }
    else
    {
        add_num = registers.mapped_register_getter(register_code as u8);
    }
    if (registers.accumulator & 0x0F) + (add_num & 0x0F) > 0x0F
    {
        registers.flags = registers.flags | 0x20;
    }
    if (registers.accumulator as u16 + add_num as u16) > 0xFF
    {
        registers.accumulator = ((registers.accumulator as u16 + add_num as u16) - 0x100) as u8;
        registers.flags = registers.flags | 0x10;
    }
    else {
        registers.accumulator += add_num;
    }
    if registers.accumulator == 0
    {
        registers.flags = registers.flags | 0x80;
    }
    registers.program_counter += 1;
}

pub fn load_n_to_8bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
        system_data.cycles = 2;
        let mut register_code = ((opcode & 0x38) >> 3) + 1;
        if register_code == 8 
        {
            register_code = 0;
        }

        if register_code == 7{
            system_data.cycles = 0;
            println!("No Opcode Found");
        }
        else
        {
            let mem_loc: usize = registers.program_counter as usize + 1;
            registers.mapped_register_setter(register_code, system_data.mem_map[mem_loc])
        }

        registers.program_counter += 2;
}

pub fn load_8_bit_register_to_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
        system_data.cycles = 1;
        //Load value from register
        let mut register_set_code = ((opcode & 0x38) >> 3) + 1;
        if register_set_code == 8 
        {
            register_set_code = 0;
        }
        let mut register_get_code = (opcode & 0x07) + 1;
        if register_get_code == 8 
        {
            register_get_code = 0;
        }

        if register_set_code == 7 || register_get_code == 7
        {
            system_data.cycles = 0;
            println!("No Opcode Found");
        }
        else {
            let register_value = registers.mapped_register_getter(register_get_code);
            registers.mapped_register_setter(register_set_code, register_value)
        }

        registers.program_counter += 1;
}

pub fn load_accumulator_to_io_port_with_c_offset(system_data: &mut SystemData, registers: &mut Registers)
{
        system_data.cycles = 2;
        system_data.mem_map[(0xFF00 + registers.c_register as u16) as usize] = registers.accumulator;
        registers.program_counter += 2;  
}

pub fn load_accumulator_to_io_port_with_n_offset(system_data: &mut SystemData, registers: &mut Registers)
{
        system_data.cycles = 3;
        let n = system_data.mem_map[(registers.program_counter + 1) as usize];
        system_data.mem_map[(0xFF00 + n as u16) as usize] = registers.accumulator;
        registers.program_counter += 2;  
}

pub fn load_accumulator_with_io_port_with_n_offset(system_data: &mut SystemData, registers: &mut Registers)
{
        system_data.cycles = 3;
        let n = system_data.mem_map[(registers.program_counter + 1) as usize];
        registers.accumulator = system_data.mem_map[(0xFF00 + n as u16) as usize];
        registers.program_counter += 2;  
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
            if registers.program_counter != 0x68{
                //println!("{:x}", registers.program_counter);
            }
            system_data.cycles = 3;
            let pc_dest: i8 = (system_data.mem_map[(registers.program_counter + 1) as usize]) as i8;
            registers.program_counter = ((registers.program_counter as i32 + pc_dest as i32) as u16) + 2;
        }
        else {
            system_data.cycles = 2;
            registers.program_counter += 2;
        }
}

pub fn jump_displacement_on_zero_flag(system_data: &mut SystemData, registers: &mut Registers)
{
    if (registers.flags & 0x80) == 0x80
    {
            system_data.cycles = 3;
            let pc_dest: i8 = (system_data.mem_map[(registers.program_counter + 1) as usize]) as i8;
            registers.program_counter = ((registers.program_counter as i32 + pc_dest as i32) as u16) + 2;   
    }
    else {
        system_data.cycles = 2;
        registers.program_counter += 2;
    }
}

pub fn jump_displacement(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 3;
    let pc_dest: i8 = (system_data.mem_map[(registers.program_counter + 1) as usize]) as i8;
    registers.program_counter = ((registers.program_counter as i32 + pc_dest as i32) as u16) + 2;
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

pub fn load_increment_hl_register_location_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    let mut mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    system_data.mem_map[mem_loc as usize] = registers.accumulator;
    mem_loc += 1;
    registers.l_register = (mem_loc & 0x00FF) as u8;
    registers.h_register = ((mem_loc & 0xFF00) >> 8) as u8;
    system_data.cycles = 2;
    registers.program_counter += 1;
}


pub fn load_hl_address_with_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    system_data.cycles = 2;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found");
    }
    else
    {
        system_data.mem_map[mem_loc as usize] = registers.mapped_register_getter(register_code);
    }


    registers.program_counter += 1;   
}

pub fn load_accumulator_with_de_address(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    let mem_loc: u16 = registers.e_register as u16 | (registers.d_register as u16) << 8;
    registers.accumulator = system_data.mem_map[mem_loc as usize];
    registers.program_counter += 1;
}

pub fn call_nn(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 6;
    let incremented_program_counter = registers.program_counter + 3;
    registers.stack_pointer -= 2;
    system_data.mem_map[registers.stack_pointer as usize + 1] = ((incremented_program_counter & 0xFF00) >> 8) as u8;
    system_data.mem_map[registers.stack_pointer as usize] = (incremented_program_counter & 0x00FF) as u8;
    registers.program_counter = (system_data.mem_map[registers.program_counter as usize + 1] as u16) | (system_data.mem_map[registers.program_counter as usize + 2] as u16) << 8;
}

pub fn return_from_call(system_data: &mut SystemData, registers: &mut Registers)
{
   system_data.cycles = 4;
   registers.program_counter = (system_data.mem_map[registers.stack_pointer as usize] as u16) | (system_data.mem_map[registers.stack_pointer as usize + 1] as u16) << 8;
   registers.stack_pointer += 2;
}

pub fn push_16_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 4;
    let mut register_code = ((opcode & 0x30) >> 4) + 1;
    if register_code == 4 
    {
        register_code = 0;
    }
    registers.stack_pointer -= 2;
    system_data.mem_map[registers.stack_pointer as usize + 1] = registers.mapped_register_getter_with_flags(register_code * 2);
    system_data.mem_map[registers.stack_pointer as usize] = registers.mapped_register_getter_with_flags((register_code * 2) + 1);
    registers.program_counter += 1;
}

pub fn pop_16_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 3;
    let mut register_code = ((opcode & 0x30) >> 4) + 1;
    if register_code == 4 
    {
        register_code = 0;
    }
    let stack_pointer = registers.stack_pointer;
    registers.mapped_register_setter_with_flags(register_code * 2, system_data.mem_map[stack_pointer as usize + 1]);
    registers.mapped_register_setter_with_flags((register_code * 2) + 1, system_data.mem_map[stack_pointer as usize]);
    registers.program_counter += 1;
    registers.stack_pointer += 2;
}

pub fn rotate_accumulator_left_through_carry(system_data: &mut SystemData, registers: &mut Registers)
{  
    let carry_bit = (registers.flags & 0x10) >> 4;
    registers.flags = 0x00;
    let mut val = registers.accumulator;
    if (val & 0x80) == 0x80
    {
            registers.flags = registers.flags | 0x10;
    }
    val = val << 1;
    val = val | carry_bit;
    registers.accumulator = val;
    registers.program_counter += 1;
    system_data.cycles = 1;
}

pub fn compare_with_n(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    registers.flags = 0x40;
    let n_value = system_data.mem_map[registers.program_counter as usize + 1];
    if registers.accumulator < n_value
    {
        registers.flags = registers.flags | 0x10;
    }
 
    if (n_value & 0x0F) > (registers.accumulator & 0x0F)
    {
        registers.flags = registers.flags | 0x20; 
    }

    if registers.accumulator == n_value
    {
        registers.flags = registers.flags | 0x80;
    }
    registers.program_counter += 2;
}

pub fn compare_with_hl_address(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    registers.flags = 0x40;
    let hl_value = system_data.mem_map[(((registers.h_register as u16) << 8) | (registers.l_register as u16)) as usize];
    if registers.accumulator < hl_value
    {
        registers.flags = registers.flags | 0x10;
    }
 
    if (hl_value & 0x0F) > (registers.accumulator & 0x0F)
    {
        registers.flags = registers.flags | 0x20; 
    }

    if registers.accumulator == hl_value
    {
        registers.flags = registers.flags | 0x80;
    }
    registers.program_counter += 1;
}

pub fn load_nn_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 4;
    let mem_loc = (system_data.mem_map[registers.program_counter as usize + 1] as u16) | (system_data.mem_map[registers.program_counter as usize + 2] as u16) << 8;
    system_data.mem_map[mem_loc as usize] =  registers.accumulator;
    registers.program_counter += 3;

}

pub fn subtract_8_bit(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 1;
    registers.flags = 0x40;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found");
    }
    else
    {
        let sub_register = registers.mapped_register_getter(register_code as u8);
        if sub_register & 0x0F > registers.accumulator & 0x0F
        {
            registers.flags = registers.flags | 0x20;
        }
        if sub_register > registers.accumulator
        {
            registers.flags = registers.flags | 0x10;
            registers.accumulator = ((0x100 + registers.accumulator as u16) - sub_register as u16) as u8
        }
        else {
            registers.accumulator -= sub_register
        }        
        if registers.accumulator == 0
        {
            registers.flags = registers.flags | 0x80;
        }
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
    else if (opcode & 0xF8) == 0x10
    {
        system_data.cycles = 2;
        rotate_left_through_carry(&mut system_data, &mut registers, opcode);
    }
    else 
    {
        println!("No Opcode Found");
        println!("Next Opcode: {:x}", system_data.mem_map[registers.program_counter as usize + 1]);
    }
}

pub fn bit_check_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8, test_bit: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }

    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found");
    }
    else
    {
        // if (registers.mapped_16_bit_register_getter(3) % 0x50 == 0)
        // {
        //     println!("{:x}", registers.mapped_16_bit_register_getter(3));
        // }
        registers.flags = registers.flags & 0x10;
        if (registers.mapped_register_getter(register_code) >> test_bit) & 0x01 == 0x00
        {   
            registers.flags = registers.flags | 0xA0;
        }
        else
        {
            registers.flags = registers.flags | 0x20;
        }
    }

        registers.program_counter += 2;
}

pub fn rotate_left_through_carry(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let carry_bit = (registers.flags & 0x10) >> 4;
    registers.flags = 0x00;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }
    if register_code == 7{
        system_data.cycles = 0;
        println!("No Opcode Found");
    }
    else {
        let mut val = registers.mapped_register_getter(register_code);
        if (val & 0x80) == 0x80{
            registers.flags = registers.flags | 0x10;
        }
        val = val << 1;
        val = val | carry_bit;
        if val == 0
        {
            registers.flags = registers.flags | 0x80;
        }
        registers.mapped_register_setter(register_code, val);
        registers.program_counter += 2;
    }
}
