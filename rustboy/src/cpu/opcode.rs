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
    if registers.program_counter > 0x290 || registers.program_counter < 0x214
    {
        //println!("Location: {:04X}\tOpcode: 0x{:02X}  {:08b}\t\t{:x} ===== {:x}", registers.program_counter, opcode, opcode, registers.accumulator, registers.flags);
        //println!("AF {:04X} BC {:04X} DE {:04X} HL {:04X} SP {:04X}", registers.mapped_16_bit_register_getter(0), registers.mapped_16_bit_register_getter(1), registers.mapped_16_bit_register_getter(2), registers.mapped_16_bit_register_getter(3), registers.mapped_16_bit_register_getter(4)) ;
    }
    
    if opcode == 0xE0 || opcode == 0xE2 || opcode == 0xF0 || opcode == 0xF2
    {
        //println!("Location: {:04X}\tOpcode: 0x{:02X}  {:08b}\t\t{:x} ===== {:x}", registers.program_counter, opcode, opcode, registers.accumulator, registers.flags);
        //println!("C-register: {:02x} -- nextopcode: {:02x}", registers.c_register, system_data.mem_map[registers.program_counter as usize + 1]);
    }

    // if registers.program_counter > 0x8000
    // {
    //      while true {}()
    // }
    //println!("{:08b}", system_data.mem_map[0xFF40]);

    if registers.interrupt_master_enable_delay_flag
    {
        registers.interrupt_master_enable_delay_flag = false;
        registers.interrupt_master_enable_flag = true;
    }

    if opcode == 0x00
    {
        no_operation(&mut system_data, &mut registers);
    }
    //Disable Interupts
    else if opcode == 0xF3
    {
        disable_interupts(&mut system_data, &mut registers);
    }
    //Enable Interrupts
    else if opcode == 0xFB
    {
        let enable_delay = enable_interupts(&mut system_data, &mut registers);
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

    else if (opcode & 0xCF) == 0x0B
    {
        decrement_16_bit_register(&mut system_data, &mut registers, opcode);
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
        if opcode == 0x36 
        {
        load_n_to_hl_location(&mut system_data, &mut registers);
        }
        else
        {
        load_n_to_8bit_register(&mut system_data, &mut registers, opcode);
        }
    }
    //0b01xxxxxx group    
    else if (opcode & 0xC0) == 0x40
    {
        //LD (HL), r
        if (opcode & 0xF8) == 0x70
        {
            load_hl_address_with_register(&mut system_data, &mut registers, opcode);
        }
        //LD r, (HL)
        else if (opcode & 0xC7) == 0x46
        {
            load_register_with_hl_location(&mut system_data, &mut registers, opcode);
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
        xor_8_bit_register(&mut system_data, &mut registers, opcode);
    }
    //or
    else if (opcode & 0xF8) == 0xB0
    {
        xor_8_bit_register(&mut system_data, &mut registers, opcode);
    }
    //and
    else if (opcode & 0xF8) == 0xA0
    {
        xor_8_bit_register(&mut system_data, &mut registers, opcode);
    }
    //jump dis
    else if opcode == 0x18
    {
        jump_displacement(&mut system_data, &mut registers);
    }
    //jump nn
    else if opcode == 0xC3
    {
        jump_address(&mut system_data, &mut registers);
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
    //LDI A, (HL)
    else if opcode == 0x2A
    {
        load_accumulator_with_hl_then_increment(&mut system_data, &mut registers);
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

    //cpl - ones complement accumulator
    else if opcode == 0x2F
    {
        ones_complement(&mut system_data, &mut registers);
    }

    else if opcode == 0xE6
    {
        and_nn_with_accumulator(&mut system_data, &mut registers);
    }

    else if (opcode & 0xC7) == 0xC7
    {
        rst_jump(&mut system_data, &mut registers, opcode);
    }
    //add hl, ss
    else if (opcode & 0xCF) == 0x09
    {
        add_16_bit_register_to_hl(&mut system_data, &mut registers, opcode);
    }
    //jp (HL)
    else if opcode == 0xE9
    {
        jump_to_hl(&mut system_data, &mut registers);
    }
    else if opcode == 0xF6
    {
        or_n(&mut system_data, &mut registers);
    }
    //sbc a, r
    else if (opcode & 0xF8) == 0x98
    {
        if opcode == 0x9E
        {
            subtract_hl_location_and_carry_from_accumulator(&mut system_data, &mut registers);
        }
        else
        {
            subtract_register_and_carry_from_accumulator(&mut system_data, &mut registers, opcode);
        }
    }
    else if (opcode & 0xC7) == 0xC4
    {
        call_function_nn_on_conditional(&mut system_data, &mut registers, opcode);
    }

    else if opcode == 0xFA
    {
        load_accumulator_with_nn_address(&mut system_data, &mut registers);
    }

    else if opcode == 0xC6
    {
        add_8_bit_to_accumulator(&mut system_data, &mut registers);
    }

    else if opcode == 0xD6
    {
        subtraction_n_from_accumulator(&mut system_data, &mut registers);
    }

    else if opcode == 0x0A
    {
        load_accumulator_with_bc_address(&mut system_data, &mut registers);
    }

    else if (opcode & 0xE7) == 0x20
    {
        jump_displacement_on_flag(&mut system_data, &mut registers, opcode);
    }
    else if opcode == 0x1F
    {
        rotate_accumulator_right_through_carry(&mut system_data, &mut registers);
    }
    else if opcode == 0xCE
    {
        add_8_bit_to_accumulator_with_carry(&mut system_data, &mut registers);
    }
    else if (opcode & 0xE7) == 0xC0
    {
        return_from_call_conditional(&mut system_data, &mut registers, opcode);
    }

    else if (opcode & 0xE7) == 0xC2
    {
        jump_address_with_conditional(&mut system_data, &mut registers, opcode);
    }

    //cb codes
    else if opcode == 0xCB
    {
        cb_codes(&mut system_data, &mut registers);
    }

    else
    {
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }

    if opcode == 0xF0 || opcode == 0xF2
    {
        //println!("{:02x}", registers.accumulator);
    }
    system_data.cycles *= 4;


}

pub fn no_operation(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 1;
    registers.program_counter += 1;
}

pub fn disable_interupts(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 1;
    registers.program_counter += 1;
    registers.interrupt_master_enable_flag = false;
}

pub fn enable_interupts(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 1;
    registers.program_counter += 1;
    registers.interrupt_master_enable_delay_flag = true;
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
            println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
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
    system_data.cycles = 2;
}

pub fn decrement_16_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.program_counter += 1;
    let register_code = ((opcode & 0x30) >> 4) + 1; 
    let mut current_register_value = registers.mapped_16_bit_register_getter(register_code);
    if current_register_value == 0x0000
    {
        current_register_value = 0xFFFF;
    }    
    else
    {
        current_register_value -= 1;
    }
    registers.mapped_16_bit_register_setter(register_code, current_register_value);
    system_data.cycles = 2;
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
        println!("No Opcode Found--ad 0x{:04x}--op 0x{:x}", registers.program_counter, opcode);
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
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else
    {
        let mem_loc: usize = registers.program_counter as usize + 1;
        registers.mapped_register_setter(register_code, system_data.mem_map[mem_loc])
    }

    registers.program_counter += 2;
}

pub fn load_n_to_hl_location(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 3;
    system_data.mem_map[registers.mapped_16_bit_register_getter(3) as usize] = system_data.mem_map[registers.program_counter as usize + 1];
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
        println!("No Opcode Found--ad 0x{:04x}--op 0x{:x}", registers.program_counter, opcode);
    }
    else {
        let register_value = registers.mapped_register_getter(register_get_code);
        registers.mapped_register_setter(register_set_code, register_value);
        registers.program_counter += 1;
    }

    
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
        println!("No Opcode Found--ad 0x{:04x}--op 0x{:x}", registers.program_counter, opcode);
    }

    registers.program_counter += 3;
}

pub fn xor_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7{
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else {
        if register_code == 8
        {
            register_code = 0;
        } 
        registers.accumulator = registers.accumulator ^ registers.mapped_register_getter(register_code);
        registers.flags = 0;
        if registers.accumulator == 0
        {
            registers.flags = 0x80;
        }

        system_data.cycles = 1;
        registers.program_counter += 1;
    }
}

pub fn or_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7{
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else {
        if register_code == 8
        {
            register_code = 0;
        } 
        registers.accumulator = registers.accumulator | registers.mapped_register_getter(register_code);
        registers.flags = 0;
        if registers.accumulator == 0
        {
            registers.flags = 0x80;
        }

        system_data.cycles = 1;
        registers.program_counter += 1;
    }
}

pub fn and_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7{
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else {
        if register_code == 8
        {
            register_code = 0;
        } 
        registers.accumulator = registers.accumulator & registers.mapped_register_getter(register_code);
        registers.flags = 0x20;
        if registers.accumulator == 0
        {
            registers.flags |= 0x80;
        }

        system_data.cycles = 1;
        registers.program_counter += 1;
    }
}

pub fn jump_displacement(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 3;
    let pc_dest: i8 = (system_data.mem_map[(registers.program_counter + 1) as usize]) as i8;
    registers.program_counter = ((registers.program_counter as i32 + pc_dest as i32) as u16) + 2;
}

pub fn jump_address(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 4;
    let upper: u16 = system_data.mem_map[registers.program_counter as usize + 2] as u16;
    let lower: u16 = system_data.mem_map[registers.program_counter as usize + 1] as u16;
    registers.program_counter = (upper << 8) | lower;
}

pub fn load_decrement_hl_register_location_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    let mut mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    system_data.mem_map[mem_loc as usize] = registers.accumulator;
    if mem_loc == 0
    {
        mem_loc = 0xFFFF;
    }
    else {
        mem_loc -= 1;
    }
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
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
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
    let value = registers.mapped_16_bit_register_getter(register_code);
    let upper = ((value & 0xFF00) >> 8) as u8;
    let lower = (value & 0x00FF) as u8;
    registers.stack_pointer -= 1;
    system_data.mem_map[registers.stack_pointer as usize] = upper;
    registers.stack_pointer -= 1;
    system_data.mem_map[registers.stack_pointer as usize] = lower;

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
    
    let lower = system_data.mem_map[registers.stack_pointer as usize] as u16;
    registers.stack_pointer += 1;
    let upper = system_data.mem_map[registers.stack_pointer as usize] as u16;
    registers.stack_pointer += 1;
    let full_value = (upper << 8) | lower;
    registers.mapped_16_bit_register_setter(register_code, full_value);
    registers.program_counter += 1;
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
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
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

pub fn load_accumulator_with_hl_then_increment(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    let mut address = registers.mapped_16_bit_register_getter(3);
    registers.accumulator = system_data.mem_map[address as usize];
    if address == 0xFFFF
    {
        registers.mapped_16_bit_register_setter(3, 0);
    }
    else {
        registers.mapped_16_bit_register_setter(3, address + 1);
    }
    
    registers.program_counter += 1;
}

pub fn ones_complement(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 1;
    registers.program_counter += 1;
    registers.accumulator ^= 0xFF;
}

pub fn and_nn_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    registers.program_counter += 2;
    let nn = system_data.mem_map[registers.program_counter as usize + 1];
    registers.accumulator &= nn;
    registers.flags = 0x20;
    if registers.accumulator == 0
    {
        registers.flags |= 0x80;
    }
}

pub fn rst_jump(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 4;
    registers.stack_pointer -= 1;
    system_data.mem_map[registers.stack_pointer as usize] = ((registers.program_counter & 0xFF00) >> 8) as u8;
    registers.stack_pointer -= 1;
    system_data.mem_map[registers.stack_pointer as usize] = ((registers.program_counter & 0x00FF)) as u8;
    let locations: Vec<u16> = vec![0x0000, 0x0008, 0x0010, 0x0018, 0x0020, 0x0028, 0x0030, 0x0038];
    let location_index = (opcode & 0x38) >> 3;
    registers.program_counter = locations[location_index as usize];
}

pub fn add_16_bit_register_to_hl(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.flags &= 0x8F;
    system_data.cycles = 2;
    registers.program_counter += 1;
    let mut register_code = (opcode & 0x30) >> 4;
    register_code += 1;
    let add_register = registers.mapped_16_bit_register_getter(register_code);
    let hl_register_temp = registers.mapped_16_bit_register_getter(3);
    let temp_32_bit_add = add_register as u32 + hl_register_temp as u32;
    if  temp_32_bit_add >= 0x00010000
    {
        registers.mapped_16_bit_register_setter(3, (temp_32_bit_add & 0x0000FFFF) as u16);
        registers.flags |= 0x10;
    }
    else{
        registers.mapped_16_bit_register_setter(3, temp_32_bit_add as u16);
    }
    if (add_register & 0x0FFF) + (hl_register_temp & 0x0FFF) >= 0x1000
    {
        registers.flags |= 0x20;
    }
}

pub fn jump_to_hl(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 1;
    registers.program_counter = registers.mapped_16_bit_register_getter(3);
}

pub fn load_register_with_hl_location(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 2;
    let mut register_code = ((opcode & 0x38) >> 3) + 1;
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else 
    {
        if register_code == 8
        {
            register_code = 0;
        }
        let location_value = system_data.mem_map[registers.mapped_16_bit_register_getter(3) as usize];
        registers.mapped_register_setter(register_code, location_value);
        registers.program_counter += 1;
    }
}

pub fn or_n(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    registers.accumulator |= system_data.mem_map[registers.program_counter as usize + 1];
    registers.flags = 0x00;
    if registers.accumulator == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}

pub fn subtract_register_and_carry_from_accumulator(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    system_data.cycles = 1;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else
    {
        if register_code == 8
        {
            register_code = 0;
        }
        let mut accumulator_value = registers.accumulator as u16;
        let register_value = registers.mapped_register_getter(register_code) as u16;
        let carry_bit= ((registers.flags & 0x10) >> 4) as u16;
        let subtraction_value = register_value + carry_bit;

        registers.flags = 0x40; 

        //Half
        if (subtraction_value & 0x0F) > (accumulator_value & 0x0F)
        {
            registers.flags |= 0x20;
        }

        //Carry
        if subtraction_value > accumulator_value
        {
            registers.flags |= 0x10; 
            accumulator_value += 0x0100;
        }

        registers.accumulator = (accumulator_value - subtraction_value) as u8;
        if registers.accumulator == 0
        {
            registers.flags |= 0x80;
        }

        registers.program_counter += 1;
    }
}

pub fn subtract_hl_location_and_carry_from_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 1;

    let mut accumulator_value = registers.accumulator as u16;
    let location_value = system_data.mem_map[registers.mapped_16_bit_register_getter(3) as usize] as u16;
    let carry_bit= ((registers.flags & 0x10) >> 4) as u16;
    let subtraction_value = location_value + carry_bit;

    registers.flags = 0x40; 

    //Half
    if (subtraction_value & 0x0F) > (accumulator_value & 0x0F)
    {
        registers.flags |= 0x20;
    }

    //Carry
    if subtraction_value > accumulator_value
    {
        registers.flags |= 0x10; 
        accumulator_value += 0x0100;
    }

    registers.accumulator = (accumulator_value - subtraction_value) as u8;
    if registers.accumulator == 0
    {
        registers.flags |= 0x80;
    }

    registers.program_counter += 1;
}

pub fn load_accumulator_with_nn_address(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 4;
    let lower = system_data.mem_map[registers.program_counter as usize + 1] as u16;
    let upper = system_data.mem_map[registers.program_counter as usize + 2] as u16;
    let retrieved_value = system_data.mem_map[(lower | (upper << 8)) as usize];
    registers.accumulator = retrieved_value;
    registers.program_counter += 3;
}

pub fn call_function_nn_on_conditional(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let condition_code = (opcode & 0x38) >> 3;
    if condition_code > 3
    {
        system_data.cycles = 0;
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
        println!("Opcode Does Not Exist On Opcode Table");
    }
    else
    {
        let mut call_flag = false;
        if condition_code == 0
        {
            if (registers.flags & 0x80) == 0
            {
                call_flag = true;
            }
        }
        else if condition_code == 1
        {
            if (registers.flags & 0x80) != 0
            {
                call_flag = true;
            }
        }
        else if condition_code == 2
        {
            if (registers.flags & 0x10) == 0
            {
                call_flag = true;
            }
        }
        else
        {
            if (registers.flags & 0x10) != 0
            {
                call_flag = true;
            }
        }

        if call_flag
        {
            system_data.cycles = 6;
            registers.stack_pointer -= 1;
            system_data.mem_map[registers.stack_pointer as usize] = ((registers.program_counter & 0xFF00) >> 8) as u8;
            registers.stack_pointer -= 1;
            system_data.mem_map[registers.stack_pointer as usize] = (registers.program_counter & 0x00FF) as u8;

            let lower = system_data.mem_map[registers.program_counter as usize + 1] as u16;
            let upper = system_data.mem_map[registers.program_counter as usize + 2] as u16;

            registers.program_counter = lower | (upper << 8);
        }
        else
        {
            system_data.cycles = 3;
            registers.program_counter += 3;
        }
    }
}

pub fn add_8_bit_to_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    let n = system_data.mem_map[registers.program_counter as usize + 1] as u16;
    let accumulator_value = registers.accumulator as u16;
    registers.flags = 0x00;
    //Half
    if (n & 0x0F) + (accumulator_value & 0x0F) >= 0x10
    {
        registers.flags |= 0x20;
    }

    if n + accumulator_value >= 0x100
    {
        registers.flags |= 0x10;
        registers.accumulator = ((n + accumulator_value) - 0x100) as u8;
    }
    else
    {
        registers.accumulator += n as u8;
    }

    if registers.accumulator == 0x00
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}

pub fn subtraction_n_from_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    let n = system_data.mem_map[registers.program_counter as usize + 1] as u16;
    let accumulator_value = registers.accumulator as u16;
    registers.flags = 0x40;
    if (n & 0x0F) > (accumulator_value & 0x0F)
    {
        registers.flags |= 0x20;
    }

    if n > accumulator_value
    {
        registers.flags |= 0x10;
        registers.accumulator = ((accumulator_value + 0x100) - n) as u8;
    }
    else
    {
        registers.accumulator -= n as u8;
    }

    if registers.accumulator == 0x00
    {
        registers.flags |= 0x80;
    }

    registers.program_counter += 2;
}

pub fn load_accumulator_with_bc_address(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    let bc_address: usize = registers.mapped_16_bit_register_getter(1) as usize;
    registers.accumulator = system_data.mem_map[bc_address];
    registers.program_counter += 1;
}

pub fn jump_displacement_on_flag(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let condition_code = (opcode & 0x18) >> 3;
    let mut call_flag: bool = false;
    match condition_code
    {
        0x00 => if (registers.flags & 0x80) != 0x80 {call_flag = true},
        0x01 => if (registers.flags & 0x80) == 0x80 {call_flag = true},
        0x02 => if (registers.flags & 0x10) != 0x10 {call_flag = true},
        0x03 => if (registers.flags & 0x10) == 0x10 {call_flag = true},
        _ =>(),
    }

    if call_flag != true
    {
        system_data.cycles = 2;
        registers.program_counter += 2;
    }

    else 
    {
        system_data.cycles = 3;
        let jump_value: i8 = system_data.mem_map[registers.program_counter as usize + 1] as i8;
        registers.program_counter = ((registers.program_counter as i32) + (jump_value as i32)) as u16;
        registers.program_counter += 2;
   }
}

pub fn rotate_accumulator_right_through_carry(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 1;
    let carry_bit = (registers.flags & 0x10) << 3;
    let carry_set_bit = (registers.accumulator & 0x01) << 4;
    registers.flags = 0x00;
    registers.accumulator = registers.accumulator >> 1;
    registers.accumulator |= carry_bit;
    registers.flags |= carry_set_bit;
    registers.program_counter += 1;
}

pub fn add_8_bit_to_accumulator_with_carry(system_data: &mut SystemData, registers: &mut Registers)
{
    system_data.cycles = 2;
    let carry_bit = (registers.flags & 0x10) >> 4;
    let n_value = system_data.mem_map[registers.program_counter as usize + 1];
    let add_value = (n_value as u16) + (carry_bit as u16);
    let accumulator_value = registers.accumulator as u16;
    registers.flags = 0x00;
    if (add_value & 0x0F) + (accumulator_value & 0x0F) >= 0x10
    {
        registers.flags |= 0x20;
    }

    if add_value + accumulator_value >= 0x100
    {
        registers.flags |= 0x10;
        registers.accumulator = ((add_value + accumulator_value) - 0x100) as u8;
    }
    else
    {
        registers.accumulator = (add_value + accumulator_value) as u8;
    }

    if registers.accumulator == 0x00
    {
        registers.flags |= 0x80;
    }

    registers.program_counter += 2;
}

pub fn return_from_call_conditional(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    //5 if pass, 2 if fail
    let condition_code = (opcode & 0x18) >> 3;
    let mut call_flag = false;
    match condition_code
    {
        0 => if (registers.flags & 0x80) != 0x80 {call_flag = true},
        1 => if (registers.flags & 0x80) == 0x80 {call_flag = true},
        2 => if (registers.flags & 0x10) != 0x10 {call_flag = true},
        3 => if (registers.flags & 0x10) == 0x10 {call_flag = true},
        _ => (),
    }

    if call_flag  == false
    {
        system_data.cycles = 2;
        registers.program_counter += 1;
    }
    else
    {
        let lower = system_data.mem_map[registers.stack_pointer as usize] as u16;
        registers.stack_pointer += 1;
        let upper = system_data.mem_map[registers.stack_pointer as usize] as u16;
        registers.stack_pointer += 1;
        registers.program_counter = lower | (upper << 8);
    }
}

pub fn jump_address_with_conditional(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let condition_code = (opcode & 0x18) >> 3;
    let mut call_flag = false;
    match condition_code
    {
        0 => if registers.flags & 0x80 != 0x80 {call_flag = true},
        1 => if registers.flags & 0x80 == 0x80 {call_flag = true},
        2 => if registers.flags & 0x10 != 0x10 {call_flag = true},
        3 => if registers.flags & 0x10 == 0x10 {call_flag = true},
        _ => (),
    }
    if call_flag == false
    {
        system_data.cycles = 3;
        registers.program_counter += 3;
    }
    else
    {
        let lower = system_data.mem_map[registers.program_counter as usize + 1] as u16;
        let upper = system_data.mem_map[registers.program_counter as usize + 2] as u16;
        registers.program_counter = lower | (upper << 8);
        system_data.cycles = 4;
    }
}

//##########################################################################
//##########################################################################
//##########################################################################
//##########################################################################
//##########################################################################
//################################    CB   #################################
//##########################################################################
//##########################################################################
//##########################################################################
//##########################################################################




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
    else if (opcode & 0xF8) == 0x30
    {
        system_data.cycles = 2;
        swap_nibbles(&mut system_data, &mut registers, opcode);
    }
    else if (opcode & 0xC0) == 0xC0
    {
        if opcode & 0x07 == 0x06
        {
            system_data.cycles = 4;
            set_bit_of_hl_location(&mut system_data, &mut registers, opcode);
        }
        else 
        {
            system_data.cycles = 2;
            set_bit_in_register(&mut system_data, &mut registers, opcode);
        }
    }
    else if (opcode & 0xF8) == 0x38
    {
        if opcode == 0x3E
        {
            shift_hl_location_right_logical(&mut system_data, &mut registers);
        }
        else{
            shift_right_register_logical(&mut system_data, &mut registers, opcode);
        }
    }
    else 
    {
        println!("Unimplemented CB code");
        println!("Next Opcode: 0x{:x}", system_data.mem_map[registers.program_counter as usize + 1]);
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
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
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
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
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

pub fn swap_nibbles(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else 
    {
        if register_code == 8
        {
            register_code = 0;
        }
        let previous_value = registers.mapped_register_getter(register_code);
        let high_nibble = previous_value & 0x80;
        let low_nibble = previous_value & 0x01;
        let new_value = (previous_value & 0x7E) | (high_nibble >> 7) | (low_nibble << 7);
        registers.mapped_register_setter(register_code, new_value);
        registers.flags = 0x00;
        if new_value == 0
        {
            registers.flags |= 0x80;
        }
        registers.program_counter += 2;
    }
}

pub fn set_bit_in_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else {
        if register_code == 8
        {
            register_code = 0;
        }
        let bit_shift = (opcode & 0x38) >> 3;
        let start_value = registers.mapped_register_getter(register_code);
        registers.mapped_register_setter(register_code, start_value | (0x01 << bit_shift));
        registers.program_counter += 2;
    }
}

pub fn set_bit_of_hl_location(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let bit_shift = (opcode & 0x38) >> 3;
    system_data.mem_map[registers.mapped_16_bit_register_getter(3) as usize] |= (0x01 << bit_shift);
    registers.program_counter += 2;
}

pub fn shift_right_register_logical(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7
    {
        system_data.cycles = 0;
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else 
    {
        if register_code == 8
        {
            register_code = 0;
        }
        registers.flags = 0x00;
        let current_register_value = registers.mapped_register_getter(register_code);
        let set_register_value = current_register_value >> 1;
        if current_register_value & 0x01 == 0x01
        {
            registers.flags |= 0x10;
        }
        if set_register_value == 0x00
        {
            registers.flags |= 0x80;
        }

        registers.mapped_register_setter(register_code, set_register_value);

        registers.program_counter += 2;
        system_data.cycles = 2;
    }

}

pub fn shift_hl_location_right_logical(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.flags = 0x00;
    let current_location_value = system_data.mem_map[registers.mapped_16_bit_register_getter(3) as usize];
    let set_location_value = current_location_value >> 1;
    if current_location_value & 0x01 == 0x01
    {
        registers.flags |= 0x10;
    }
    if set_location_value == 0x00
    {
        registers.flags |= 0x80;
    }

    system_data.mem_map[registers.mapped_16_bit_register_getter(3) as usize] = set_location_value;

    registers.program_counter += 2;
    system_data.cycles = 4;
}