use system::SystemData;
use system::Registers;
use cpu::opcode_helpers::*;

use std::io;


// Returns clock system_data.cycle passed during opcode
pub fn parse_opcode(system_data_original: &mut SystemData, registers_original: &mut Registers)
{
    //Borrow splitting
    let mut system_data = system_data_original;
    let mut registers = registers_original;
    let mut bonus_cycles = 0;

    if registers.halt_flag
    {
        if (system_data.mmu.get_from_memory(0xFF0F, false) & 0x1F) != 0x00
        {
            registers.halt_flag = false;
            bonus_cycles += 4;
        }
        else{
            system_data.cycles = 4;
            return;
        }
    }
//42A6
    system_data.cycles = 0;
    let mut opcode: u8 = system_data.mmu.get_from_memory(registers.program_counter as usize, false);
    // if (registers.program_counter >= 0x312  && registers.program_counter < 0xC320) || registers.program_counter < 0x100
    // //  {
    //   if registers.program_counter >= 0x209E && registers.program_counter < 0x2100
    // {

    //      println!("Location: {:04X}\tOpcode: 0x{:02X}  {:08b}\t\t{:x} ===== {:x}", registers.program_counter, opcode, opcode, registers.accumulator, registers.flags);
    //      println!("AF {:04X} BC {:04X} DE {:04X} HL {:04X} SP {:04X} LY {} IE {:02X} IF {:02X} TIMER {:02X} DIV {:02X} DIV MEMORY {:02X} FULL CLOCK {:04X}", registers.mapped_16_bit_register_getter(0), registers.mapped_16_bit_register_getter(1), 
    //                                                                                                 registers.mapped_16_bit_register_getter(2), registers.mapped_16_bit_register_getter(3), 
    //                                                                                                 registers.mapped_16_bit_register_getter(4), system_data.mmu.get_from_memory(0xFF44, false)
    //                                                                                                 , system_data.mmu.get_from_memory(0xFFFF, false), system_data.mmu.get_from_memory(0xFF0F, false),
    //                                                                                                 system_data.mmu.get_from_memory(0xFF05, false), system_data.timer.divider_register, system_data.mmu.get_from_memory(0xFF04, false),
    //                                                                                                 system_data.timer.cycle_register);
    //         io::stdin().read_line(&mut String::new());
    //         system_data.debug_flag1 = true;
    //       }
    //  }
    
    if opcode == 0xE0 || opcode == 0xE2 || opcode == 0xF0 || opcode == 0xF2
    {
        //println!("Location: {:04X}\tOpcode: 0x{:02X}  {:08b}\t\t{:x} ===== {:x}", registers.program_counter, opcode, opcode, registers.accumulator, registers.flags);
        //println!("C-register: {:02x} -- nextopcode: {:02x}", registers.c_register, system_data.mem_map[registers.program_counter as usize + 1]);
    }

    //if registers.program_counter == 0x100
    //{
        //println!("LOCATION CATCH");
     if registers.program_counter == 0xC1B9
    {
        //io::stdin().read_line(&mut String::new());
    }
    //     //while true {}()
    // }
    //println!("{:08b}", system_data.mem_map[0xFF40]);

    //Interrupt Handling
    if registers.interrupt_master_enable_flag
    {
        let enabled_interrupts = system_data.mmu.get_from_memory(0xFFFF, false);
        let requested_interrupts = system_data.mmu.get_from_memory(0xFF0F, false);
        let runnable_interrupts = enabled_interrupts & requested_interrupts;
        if requested_interrupts != 0 {
            //println!("{} -- {} -- {}", enabled_interrupts, requested_interrupts, runnable_interrupts);
        }
        if runnable_interrupts > 0x00 && runnable_interrupts <= 0x1F
        {
            let mut interrupt_vector = 0;
            if runnable_interrupts & 0x01 == 0x01 {interrupt_vector = 0x40; system_data.mmu.set_to_memory(0xFF0F,requested_interrupts & 0x1E, false);}
            else if runnable_interrupts & 0x02 == 0x02 {interrupt_vector = 0x48; system_data.mmu.set_to_memory(0xFF0F, requested_interrupts & 0x1D, false);}
            else if runnable_interrupts & 0x04 == 0x04 {interrupt_vector = 0x50; system_data.mmu.set_to_memory(0xFF0F, requested_interrupts & 0x1B, false);}
            else if runnable_interrupts & 0x08 == 0x08 {interrupt_vector = 0x58; system_data.mmu.set_to_memory(0xFF0F, requested_interrupts & 0x17, false);}
            else if runnable_interrupts & 0x10 == 0x10 {interrupt_vector = 0x60; system_data.mmu.set_to_memory(0xFF0F, requested_interrupts & 0x0F, false);}
            registers.stack_pointer -= 2;
            system_data.mmu.set_to_memory(registers.stack_pointer as usize + 1, ((registers.program_counter & 0xFF00) >> 8) as u8, true);
            system_data.mmu.set_to_memory(registers.stack_pointer as usize, (registers.program_counter & 0x00FF) as u8, true);
            registers.program_counter = interrupt_vector as u16;
            opcode = system_data.mmu.get_from_memory(registers.program_counter as usize, false);
            registers.interrupt_master_enable_flag = !registers.interrupt_master_enable_flag;
            bonus_cycles += 20;
        }
        
    }

    if registers.interrupt_master_enable_delay_flag
    {
        registers.interrupt_master_enable_delay_flag = false;
        registers.interrupt_master_enable_flag = true;
    }

    system_data.cycles = cycle_parse(opcode);

    match opcode
    {
0x00 => no_operation(&mut system_data, &mut registers),
0x01 => load_nn_to_16bit_register(&mut system_data, &mut registers, opcode),
0x02 => load_accumulator_to_address_at_bc(&mut system_data, &mut registers),
0x03 => increment_16_bit_register(&mut system_data, &mut registers, opcode),
0x04 => increment_8_bit_register(&mut system_data, &mut registers, opcode),
0x05 => decrement_8_bit_register(&mut system_data, &mut registers, opcode),
0x06 => load_n_to_8bit_register(&mut system_data, &mut registers, opcode),
0x07 => rlca(&mut system_data, &mut registers),
0x08 => load_stack_pointer_to_nn_address(&mut system_data, &mut registers),
0x09 => add_16_bit_register_to_hl(&mut system_data, &mut registers, opcode),
0x0A => load_accumulator_with_bc_address(&mut system_data, &mut registers),
0x0B => decrement_16_bit_register(&mut system_data, &mut registers, opcode),
0x0C => increment_8_bit_register(&mut system_data, &mut registers, opcode),
0x0D => decrement_8_bit_register(&mut system_data, &mut registers, opcode),
0x0E => load_n_to_8bit_register(&mut system_data, &mut registers, opcode),
0x0F => rrca(&mut system_data, &mut registers),

0x10 => stop(&mut system_data, &mut registers), //TEMP FOR DEBUGGING. CHANGE WHEN IMPLEMENTING INTERUPTS
0x11 => load_nn_to_16bit_register(&mut system_data, &mut registers, opcode),
0x12 => load_de_location_with_accumulator(&mut system_data, &mut registers),
0x13 => increment_16_bit_register(&mut system_data, &mut registers, opcode),
0x14 => increment_8_bit_register(&mut system_data, &mut registers, opcode),
0x15 => decrement_8_bit_register(&mut system_data, &mut registers, opcode),
0x16 => load_n_to_8bit_register(&mut system_data, &mut registers, opcode),
0x17 => rotate_accumulator_left_through_carry(&mut system_data, &mut registers),
0x18 => jump_displacement(&mut system_data, &mut registers),
0x19 => add_16_bit_register_to_hl(&mut system_data, &mut registers, opcode),
0x1A => load_accumulator_with_de_address(&mut system_data, &mut registers),
0x1B => decrement_16_bit_register(&mut system_data, &mut registers, opcode),
0x1C => increment_8_bit_register(&mut system_data, &mut registers, opcode),
0x1D => decrement_8_bit_register(&mut system_data, &mut registers, opcode),
0x1E => load_n_to_8bit_register(&mut system_data, &mut registers, opcode),
0x1F => rotate_accumulator_right_through_carry(&mut system_data, &mut registers),

0x20 => jump_displacement_on_flag(&mut system_data, &mut registers, opcode),
0x21 => load_nn_to_16bit_register(&mut system_data, &mut registers, opcode),
0x22 => load_increment_hl_register_location_with_accumulator(&mut system_data, &mut registers), 
0x23 => increment_16_bit_register(&mut system_data, &mut registers, opcode),
0x24 => increment_8_bit_register(&mut system_data, &mut registers, opcode),
0x25 => decrement_8_bit_register(&mut system_data, &mut registers, opcode),
0x26 => load_n_to_8bit_register(&mut system_data, &mut registers, opcode),
0x27 => bcd_adjust(&mut system_data, &mut registers),
0x28 => jump_displacement_on_flag(&mut system_data, &mut registers, opcode),
0x29 => add_16_bit_register_to_hl(&mut system_data, &mut registers, opcode),
0x2A => load_accumulator_with_hl_then_increment(&mut system_data, &mut registers),
0x2B => decrement_16_bit_register(&mut system_data, &mut registers, opcode),
0x2C => increment_8_bit_register(&mut system_data, &mut registers, opcode),
0x2D => decrement_8_bit_register(&mut system_data, &mut registers, opcode),
0x2E => load_n_to_8bit_register(&mut system_data, &mut registers, opcode),
0x2F => ones_complement(&mut system_data, &mut registers),

0x30 => jump_displacement_on_flag(&mut system_data, &mut registers, opcode),
0x31 => load_nn_to_16bit_register(&mut system_data, &mut registers, opcode),
0x32 => load_decrement_hl_register_location_with_accumulator(&mut system_data, &mut registers),
0x33 => increment_16_bit_register(&mut system_data, &mut registers, opcode),
0x34 => increment_hl_location(&mut system_data, &mut registers),
0x35 => decrement_hl_location(&mut system_data, &mut registers),
0x36 => load_n_to_hl_location(&mut system_data, &mut registers),
0x37 => set_carry_flag(&mut system_data, &mut registers),
0x38 => jump_displacement_on_flag(&mut system_data, &mut registers, opcode),
0x39 => add_16_bit_register_to_hl(&mut system_data, &mut registers, opcode),
0x3A => load_accumulator_with_hl_then_decrement(&mut system_data, &mut registers),
0x3B => decrement_16_bit_register(&mut system_data, &mut registers, opcode),
0x3C => increment_8_bit_register(&mut system_data, &mut registers, opcode),
0x3D => decrement_8_bit_register(&mut system_data, &mut registers, opcode),
0x3E => load_n_to_8bit_register(&mut system_data, &mut registers, opcode),
0x3F => flip_carry_flag(&mut system_data, &mut registers),

0x40...0x45 | 0x47...0x4D | 0x4F...0x55 | 0x57...0x5D | 
0x5F...0x65 | 0x67...0x6D | 0x6F| 0x78...0x7D | 0x7F => load_8_bit_register_to_register(&mut system_data, &mut registers, opcode),
0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E  | 0x7E => load_register_with_hl_location(&mut system_data, &mut registers, opcode),
0x70...0x75 | 0x77=> load_hl_address_with_register(&mut system_data, &mut registers, opcode),
0x76 => halt(&mut system_data, &mut registers),
0x80...0x87 => add_8_bit(&mut system_data, &mut registers, opcode),
0x88...0x8F => add_registers_to_accumulator_with_carry(&mut system_data, &mut registers, opcode), 
0x90...0x97 => subtract_8_bit(&mut system_data, &mut registers, opcode),
0x98...0x9D | 0x9F => subtract_register_and_carry_from_accumulator(&mut system_data, &mut registers, opcode),
0x9E => subtract_hl_location_and_carry_from_accumulator(&mut system_data, &mut registers),
0xA0...0xA7 => and_8_bit_register(&mut system_data, &mut registers, opcode),
0xA8...0xAD | 0xAF => xor_8_bit_register(&mut system_data, &mut registers, opcode),
0xAE => xor_hl_location(&mut system_data, &mut registers),
0xB0...0xB5 | 0xB7 => or_8_bit_register(&mut system_data, &mut registers, opcode),
0xB6 => or_hl_location(&mut system_data, &mut registers),
0xB8...0xBD | 0xBF => compare_register_to_accumulator(&mut system_data, &mut registers, opcode),
0xBE => compare_with_hl_address(&mut system_data, &mut registers),

0xC0 => return_from_call_conditional(&mut system_data, &mut registers, opcode),
0xC1 => pop_16_bit_register(&mut system_data, &mut registers, opcode),
0xC2 => jump_address_with_conditional(&mut system_data, &mut registers, opcode),
0xC3 => jump_address(&mut system_data, &mut registers),
0xC4 => call_function_nn_on_conditional(&mut system_data, &mut registers, opcode),
0xC5 => push_16_bit_register(&mut system_data, &mut registers, opcode),
0xC6 => add_8_bit_to_accumulator(&mut system_data, &mut registers),
0xC7 => rst_jump(&mut system_data, &mut registers, opcode),
0xC8 => return_from_call_conditional(&mut system_data, &mut registers, opcode),
0xC9 => return_from_call(&mut system_data, &mut registers),
0xCA => jump_address_with_conditional(&mut system_data, &mut registers, opcode),
0xCB => cb_codes(&mut system_data, &mut registers),
0xCC => call_function_nn_on_conditional(&mut system_data, &mut registers, opcode),
0xCD => call_nn(&mut system_data, &mut registers),
0xCE => add_8_bit_to_accumulator_with_carry(&mut system_data, &mut registers),
0xCF => rst_jump(&mut system_data, &mut registers, opcode),

0xD0 => return_from_call_conditional(&mut system_data, &mut registers, opcode),
0xD1 => pop_16_bit_register(&mut system_data, &mut registers, opcode),
0xD2 => jump_address_with_conditional(&mut system_data, &mut registers, opcode),
0xD3 => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xD4 => call_function_nn_on_conditional(&mut system_data, &mut registers, opcode),
0xD5 => push_16_bit_register(&mut system_data, &mut registers, opcode),
0xD6 => subtraction_n_from_accumulator(&mut system_data, &mut registers),
0xD7 => rst_jump(&mut system_data, &mut registers, opcode),
0xD8 => return_from_call_conditional(&mut system_data, &mut registers, opcode),
0xD9 => return_from_call_ei(&mut system_data, &mut registers),
0xDA => jump_address_with_conditional(&mut system_data, &mut registers, opcode),
0xDB => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xDC => call_function_nn_on_conditional(&mut system_data, &mut registers, opcode),
0xDD => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xDE => subtract_8_bit_from_accumulator_with_carry(&mut system_data, &mut registers),
0xDF => rst_jump(&mut system_data, &mut registers, opcode),

0xE0 => load_accumulator_to_io_port_with_n_offset(&mut system_data, &mut registers),
0xE1 => pop_16_bit_register(&mut system_data, &mut registers, opcode),
0xE2 => load_accumulator_to_io_port_with_c_offset(&mut system_data, &mut registers),
0xE3 => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xE4 => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xE5 => push_16_bit_register(&mut system_data, &mut registers, opcode),
0xE6 => and_nn_with_accumulator(&mut system_data, &mut registers),
0xE7 => rst_jump(&mut system_data, &mut registers, opcode),
0xE8 => add_signed_8_bit_to_stack_pointer(&mut system_data, &mut registers),
0xE9 => jump_to_hl(&mut system_data, &mut registers),
0xEA => load_nn_with_accumulator(&mut system_data, &mut registers),
0xEB => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xEC => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xED => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xEE => xor_accumulator_with_n(&mut system_data, &mut registers),
0xEF => rst_jump(&mut system_data, &mut registers, opcode),

0xF0 => load_accumulator_with_io_port_with_n_offset(&mut system_data, &mut registers),
0xF1 => pop_16_bit_register(&mut system_data, &mut registers, opcode),
0xF2 => read_io_port_with_c_offset_to_accumulator(&mut system_data, &mut registers),
0xF3 => disable_interupts(&mut system_data, &mut registers),
0xF4 => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xF5 => push_16_bit_register(&mut system_data, &mut registers, opcode),
0xF6 => or_n(&mut system_data, &mut registers),
0xF7 => rst_jump(&mut system_data, &mut registers, opcode),
0xF8 => load_hl_with_stack_pointer_plus_n(&mut system_data, &mut registers),
0xF9 => load_hl_to_stack_pointer(&mut system_data, &mut registers),
0xFA => load_accumulator_with_nn_address(&mut system_data, &mut registers),
0xFB => enable_interupts(&mut system_data, &mut registers),
0xFC => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xFD => println!("Illegal Opcode - 0x{:X} --- 0x{:X}", registers.program_counter, opcode), // Illegal
0xFE => compare_with_n(&mut system_data, &mut registers),
0xFF => rst_jump(&mut system_data, &mut registers, opcode),
   _ => println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode),
    }
          ;
    if opcode == 0xF0 || opcode == 0xF2
    {
        //println!("{:02x}", registers.accumulator);
    }
    system_data.cycles *= 4;
    system_data.cycles += bonus_cycles;

    if system_data.cycles == 0
    {
        while true {;}
    }


}

pub fn no_operation(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
}

pub fn disable_interupts(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.interrupt_master_enable_flag = false;
}

pub fn enable_interupts(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.interrupt_master_enable_delay_flag = true;
}

pub fn stop(system_data: &mut SystemData, registers: &mut Registers)
{
    //Temporary debugging
    registers.program_counter += 2;
}

pub fn halt(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.halt_flag = true;
}

pub fn increment_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
        registers.flags = registers.flags & 0x10;
        let mut register_code = (((opcode & 0x38) >> 3) + 1) % 8;
        let mut current_register_value = registers.mapped_register_getter(register_code);
        
        if current_register_value == 0xFF{
            current_register_value = 0;
        }
        else {
            current_register_value += 1;
        }
        registers.mapped_register_setter(register_code, current_register_value);
        if current_register_value == 0
        {
            registers.flags |= 0x80;
        }
        if (current_register_value & 0x0F) == 0
        {
            registers.flags |= 0x20;
        }
        registers.program_counter += 1;
}

pub fn increment_hl_location(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.flags = registers.flags & 0x10;

    let mut current_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    
    if current_value == 0xFF{
        current_value = 0;
    }
    else {
        current_value += 1;
    }
    if current_value == 0
    {
        registers.flags = registers.flags | 0x80;
    }
    if current_value &0x0F == 0x0
    {
        registers.flags = registers.flags | 0x20;
    }

    system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, current_value, true);
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
}

pub fn decrement_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.flags = registers.flags & 0x10;
    registers.flags = registers.flags | 0x40;
    let mut register_code = ((opcode & 0x38) >> 3) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }
            
    if register_code == 7
    {
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
    registers.flags = 0x00;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8
    {
        register_code = 0;
    }

    let mut add_num;
    if register_code == 7
    {
        add_num = system_data.mmu.get_from_memory((((registers.h_register as u16) << 8)|(registers.l_register as u16)) as usize, true);
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
    let mut register_code = ((opcode & 0x38) >> 3) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }

    if register_code == 7{
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else
    {
        let mem_loc: usize = registers.program_counter as usize + 1;
        registers.mapped_register_setter(register_code, system_data.mmu.get_from_memory(mem_loc, false))
    }

    registers.program_counter += 2;
}

pub fn load_n_to_hl_location(system_data: &mut SystemData, registers: &mut Registers)
{
    let n_value = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, n_value, true);
    registers.program_counter += 2;
}

pub fn load_8_bit_register_to_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
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
    system_data.mmu.set_to_memory((0xFF00 + registers.c_register as u16) as usize ,registers.accumulator, true);
    registers.program_counter += 1;  
}

pub fn read_io_port_with_c_offset_to_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.accumulator = system_data.mmu.get_from_memory((0xFF00 + registers.c_register as u16) as usize, false);
    registers.program_counter += 1;  
}

pub fn load_accumulator_to_io_port_with_n_offset(system_data: &mut SystemData, registers: &mut Registers)
{
    let n = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    system_data.mmu.set_to_memory((0xFF00 + n as u16) as usize ,registers.accumulator, true);
    registers.program_counter += 2;  
}

pub fn load_accumulator_with_io_port_with_n_offset(system_data: &mut SystemData, registers: &mut Registers)
{
    let n = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    registers.accumulator = system_data.mmu.get_from_memory((0xFF00 + n as u16) as usize, false);
    registers.program_counter += 2;  
} 

pub fn load_nn_to_16bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8){

    let lower = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
    let upper = system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16;
    let set_value = lower | (upper << 8);
    match opcode
    {
        0x01 => registers.mapped_16_bit_register_setter(1, set_value),
        0x11 => registers.mapped_16_bit_register_setter(2, set_value),
        0x21 => registers.mapped_16_bit_register_setter(3, set_value),
        0x31 => registers.mapped_16_bit_register_setter(4, set_value),
        _ => {
            println!("ERROR- INVALID REGISTER CODE --ad 0x{:04x}--op 0x{:x}", registers.program_counter, opcode);
            return;
        }
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
        registers.program_counter += 1;
    }
}

pub fn and_8_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8
    {
        register_code = 0;
    } 
    let mut compare_value = 0;
    if register_code == 7
    {
        compare_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else {
        compare_value = registers.mapped_register_getter(register_code);
    }
    registers.accumulator = registers.accumulator & compare_value;
    registers.flags = 0x20;
    if registers.accumulator == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 1;
}

pub fn jump_displacement(system_data: &mut SystemData, registers: &mut Registers)
{
    let pc_dest: i8 = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as i8;
    registers.program_counter = ((registers.program_counter as i32 + pc_dest as i32) as u16) + 2;
}

pub fn jump_address(system_data: &mut SystemData, registers: &mut Registers)
{
    let upper: u16 = system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16;
    let lower: u16 = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
    registers.program_counter = (upper << 8) | lower;
}

pub fn load_decrement_hl_register_location_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    let mut mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    system_data.mmu.set_to_memory(mem_loc as usize, registers.accumulator, true);
    if mem_loc == 0
    {
        mem_loc = 0xFFFF;
    }
    else {
        mem_loc -= 1;
    }
    registers.l_register = (mem_loc & 0x00FF) as u8;
    registers.h_register = ((mem_loc & 0xFF00) >> 8) as u8;
    registers.program_counter += 1;
}

pub fn load_increment_hl_register_location_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    let mut mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    system_data.mmu.set_to_memory(mem_loc as usize, registers.accumulator, true);
    if mem_loc == 0xFFFF
    {
        mem_loc = 0x0000;
    }
    else 
    {
        mem_loc += 1;
    }
    registers.l_register = (mem_loc & 0x00FF) as u8;
    registers.h_register = ((mem_loc & 0xFF00) >> 8) as u8;
    registers.program_counter += 1;
}


pub fn load_hl_address_with_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mem_loc: u16 = registers.l_register as u16 | (registers.h_register as u16) << 8;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }
    if register_code == 7
    {
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else
    {
        system_data.mmu.set_to_memory(mem_loc as usize, registers.mapped_register_getter(register_code), true);
    }


    registers.program_counter += 1;   
}

pub fn load_accumulator_with_de_address(system_data: &mut SystemData, registers: &mut Registers)
{
    let mem_loc: u16 = registers.e_register as u16 | (registers.d_register as u16) << 8;
    registers.accumulator = system_data.mmu.get_from_memory(mem_loc as usize, true);
    registers.program_counter += 1;
}

pub fn call_nn(system_data: &mut SystemData, registers: &mut Registers)
{
    let incremented_program_counter = registers.program_counter + 3;
    registers.stack_pointer -= 2;
    system_data.mmu.set_to_memory(registers.stack_pointer as usize + 1, ((incremented_program_counter & 0xFF00) >> 8) as u8, true);
    system_data.mmu.set_to_memory(registers.stack_pointer as usize, (incremented_program_counter & 0x00FF) as u8, true);
    registers.program_counter = (system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16) | (system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16) << 8;
}

pub fn return_from_call(system_data: &mut SystemData, registers: &mut Registers)
{
   registers.program_counter = (system_data.mmu.get_from_memory(registers.stack_pointer as usize, true) as u16) | (system_data.mmu.get_from_memory(registers.stack_pointer as usize + 1, true) as u16) << 8;
   registers.stack_pointer += 2;
}

pub fn return_from_call_ei(system_data: &mut SystemData, registers: &mut Registers)
{
   registers.program_counter = (system_data.mmu.get_from_memory(registers.stack_pointer as usize, true) as u16) | (system_data.mmu.get_from_memory(registers.stack_pointer as usize + 1, true) as u16) << 8;
   registers.stack_pointer += 2;
   registers.interrupt_master_enable_delay_flag = true;
}

pub fn push_16_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = ((opcode & 0x30) >> 4) + 1;
    if register_code == 4 
    {
        register_code = 0;
    }
    let value = registers.mapped_16_bit_register_getter(register_code);
    let upper = ((value & 0xFF00) >> 8) as u8;
    let lower = (value & 0x00FF) as u8;
    registers.stack_pointer -= 1;
    system_data.mmu.set_to_memory(registers.stack_pointer as usize, upper, true);
    registers.stack_pointer -= 1;
    system_data.mmu.set_to_memory(registers.stack_pointer as usize, lower, true);

    registers.program_counter += 1;
}

pub fn pop_16_bit_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = 0;
    match opcode
    {
        0xF1 => register_code = 0,
        0xC1 => register_code = 1,
        0xD1 => register_code = 2,
        0xE1 => register_code = 3,
        _ => (),
    }
    
    let lower = system_data.mmu.get_from_memory(registers.stack_pointer as usize, true) as u16;
    registers.stack_pointer += 1;
    let upper = system_data.mmu.get_from_memory(registers.stack_pointer as usize, true) as u16;
    registers.stack_pointer += 1;
    let mut full_value = (upper << 8) | lower;
    if register_code == 0
    {
        full_value &= 0xFFF0;
    }
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
}

pub fn compare_with_n(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.flags = 0x40;
    let n_value = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
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
    registers.flags = 0x40;
    let hl_value = system_data.mmu.get_from_memory((((registers.h_register as u16) << 8) | (registers.l_register as u16)) as usize, true);
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
    let mem_loc = (system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16) | (system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16) << 8;
    system_data.mmu.set_to_memory(mem_loc as usize, registers.accumulator, true);
    registers.program_counter += 3;

}

pub fn subtract_8_bit(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.flags = 0x40;
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8 
    {
        register_code = 0;
    }
    let mut sub_register = 0;
    if register_code == 7
    {
        sub_register = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
        sub_register = registers.mapped_register_getter(register_code as u8);    
    }
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
    registers.program_counter += 1;
}

pub fn load_accumulator_with_hl_then_increment(system_data: &mut SystemData, registers: &mut Registers)
{
    let mut address = registers.mapped_16_bit_register_getter(3);
    registers.accumulator = system_data.mmu.get_from_memory(address as usize, true);
    if address == 0xFFFF
    {
        registers.mapped_16_bit_register_setter(3, 0);
    }
    else {
        registers.mapped_16_bit_register_setter(3, address + 1);
    }
    
    registers.program_counter += 1;
}

pub fn load_accumulator_with_hl_then_decrement(system_data: &mut SystemData, registers: &mut Registers)
{
    let mut address = registers.mapped_16_bit_register_getter(3);
    registers.accumulator = system_data.mmu.get_from_memory(address as usize, true);
    registers.accumulator = system_data.mmu.get_from_memory(address as usize, true);
    if address == 0x0000
    {
        registers.mapped_16_bit_register_setter(3, 0xFFFF);
    }
    else {
        registers.mapped_16_bit_register_setter(3, address - 1);
    }

    registers.program_counter += 1;
}

pub fn ones_complement(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.accumulator ^= 0xFF;
    registers.flags |= 0x60;
}

pub fn and_nn_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    let nn = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    registers.accumulator &= nn;
    registers.flags = 0x20;
    if registers.accumulator == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}

pub fn rst_jump(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.program_counter += 1;
    registers.stack_pointer -= 1;
    system_data.mmu.set_to_memory(registers.stack_pointer as usize, ((registers.program_counter & 0xFF00) >> 8) as u8, true);
    registers.stack_pointer -= 1;
    system_data.mmu.set_to_memory(registers.stack_pointer as usize, ((registers.program_counter & 0x00FF)) as u8, true);
    let locations: Vec<u16> = vec![0x0000, 0x0008, 0x0010, 0x0018, 0x0020, 0x0028, 0x0030, 0x0038];
    let location_index = (opcode & 0x38) >> 3;
    registers.program_counter = locations[location_index as usize];
}

pub fn add_16_bit_register_to_hl(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.flags &= 0x8F;
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
    registers.program_counter = registers.mapped_16_bit_register_getter(3);
}

pub fn load_register_with_hl_location(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = ((opcode & 0x38) >> 3) + 1;
    if register_code == 7
    {
        println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
    }
    else 
    {
        if register_code == 8
        {
            register_code = 0;
        }
        let location_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
        registers.mapped_register_setter(register_code, location_value);
        registers.program_counter += 1;
    }
}

pub fn or_n(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.accumulator |= system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    registers.flags = 0x00;
    if registers.accumulator == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}

pub fn subtract_register_and_carry_from_accumulator(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7
    {
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
        if (register_value & 0x0F) + carry_bit > (accumulator_value & 0x0F)
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
    let mut accumulator_value = registers.accumulator as u16;
    let location_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true) as u16;
    let carry_bit= ((registers.flags & 0x10) >> 4) as u16;
    let subtraction_value = location_value + carry_bit;

    registers.flags = 0x40; 

    //Half
    if (location_value & 0x0F) + carry_bit > (accumulator_value & 0x0F)
    {
        registers.flags |= 0x20;
    }

    //Carry
    if (location_value + carry_bit) > accumulator_value
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
    let lower = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
    let upper = system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16;
    let retrieved_value = system_data.mmu.get_from_memory((lower | (upper << 8)) as usize, true);
    registers.accumulator = retrieved_value;
    registers.program_counter += 3;
}

pub fn load_stack_pointer_to_nn_address(system_data: &mut SystemData, registers: &mut Registers)
{
    let lower = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
    let upper = system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16;
    let address = lower | (upper << 8);
    system_data.mmu.set_to_memory(address as usize, (registers.stack_pointer & 0xFF) as u8, true);
    system_data.mmu.set_to_memory(address as usize + 1, ((registers.stack_pointer & 0xFF00) >> 8) as u8, true);
    registers.program_counter += 3
}

pub fn call_function_nn_on_conditional(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let condition_code = (opcode & 0x38) >> 3;
    if condition_code > 3
    {
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
            let lower = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
            let upper = system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16;

            registers.program_counter += 3;

            registers.stack_pointer -= 1;
            system_data.mmu.set_to_memory(registers.stack_pointer as usize, ((registers.program_counter & 0xFF00) >> 8) as u8, true);
            registers.stack_pointer -= 1;
            system_data.mmu.set_to_memory(registers.stack_pointer as usize, (registers.program_counter & 0x00FF) as u8, true);


            registers.program_counter = lower | (upper << 8);
        }
        else
        {
            registers.program_counter += 3;
        }
    }
}

pub fn add_8_bit_to_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    let n = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
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
    let n = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
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
    let bc_address: usize = registers.mapped_16_bit_register_getter(1) as usize;
    registers.accumulator = system_data.mmu.get_from_memory(bc_address, true);
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
        registers.program_counter += 2;
    }

    else 
    {
        system_data.cycles = 3;
        let jump_value: i8 = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as i8;
        registers.program_counter = ((registers.program_counter as i32) + (jump_value as i32)) as u16;
        registers.program_counter += 2;
   }
}

pub fn rotate_accumulator_right_through_carry(system_data: &mut SystemData, registers: &mut Registers)
{
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
    let carry_bit = (registers.flags & 0x10) >> 4;
    let n_value = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    let add_value = (n_value as u16) + (carry_bit as u16);
    let accumulator_value = registers.accumulator as u16;
    registers.flags = 0x00;
    if (n_value & 0x0F) + (accumulator_value as u8 & 0x0F) + carry_bit >= 0x10
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

pub fn subtract_8_bit_from_accumulator_with_carry(system_data: &mut SystemData, registers: &mut Registers)
{
    let carry_bit = (registers.flags & 0x10) >> 4;
    let n_value = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    let sub_value = (n_value as u16) + (carry_bit as u16);
    let accumulator_value = registers.accumulator as u16;
    registers.flags = 0x40;
    if ((n_value & 0x0F) + carry_bit) > (accumulator_value as u8 & 0x0F)
    {
        registers.flags |= 0x20;
    }

    if sub_value > accumulator_value
    {
        registers.flags |= 0x10;
        registers.accumulator = ((accumulator_value + 0x100) - sub_value) as u8;
    }
    else
    {
        registers.accumulator = (accumulator_value - sub_value) as u8;
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
        registers.program_counter += 1;
    }
    else
    {
        system_data.cycles = 5;
        let lower = system_data.mmu.get_from_memory(registers.stack_pointer as usize, true) as u16;
        registers.stack_pointer += 1;
        let upper = system_data.mmu.get_from_memory(registers.stack_pointer as usize, true) as u16;
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
        registers.program_counter += 3;
    }
    else
    {
        let lower = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
        let upper = system_data.mmu.get_from_memory(registers.program_counter as usize + 2, false) as u16;
        registers.program_counter = lower | (upper << 8);
        system_data.cycles = 4;
    }
}

pub fn xor_hl_location(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.flags = 0x00;
    registers.accumulator ^= system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    if registers.accumulator == 0x00
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 1;
}

pub fn xor_accumulator_with_n(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.flags = 0x00;
    let n_value = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false);
    registers.accumulator ^= n_value;
    if registers.accumulator == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}

pub fn add_registers_to_accumulator_with_carry(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = 0;
    match opcode
    {
        0x8F => register_code = 0,
        0x88 => register_code = 1,
        0x89 => register_code = 2,
        0x8A => register_code = 3,
        0x8B => register_code = 4,
        0x8C => register_code = 5,
        0x8D => register_code = 6,
        0x8E => register_code = 7,
        _ => {
            println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
            return;
            },
    }
    let carry_bit = ((registers.flags & 0x10) >> 4) as u16;
    let accumulator_value = registers.accumulator as u16;
    let mut register_value = 0;
    if register_code != 7 
    {
        register_value = registers.mapped_register_getter(register_code) as u16;
        system_data.cycles = 1;
    }
    else 
    {
        register_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true) as u16;
        system_data.cycles = 2;
    }
    let add_value =  register_value + carry_bit;
    registers.flags = 0x00;
    if (accumulator_value & 0x0F) + ((register_value & 0x0F) + carry_bit) >= 0x10
    {
        registers.flags |= 0x20;
    }

    let mut new_value = accumulator_value + add_value;
    if new_value >= 0x100
    {
        registers.flags |= 0x10;
        new_value -= 0x100;
    }

    if new_value == 0x00
    {
        registers.flags |= 0x80;
    }
    registers.accumulator = new_value as u8;

    
    registers.program_counter += 1;
}

pub fn or_hl_location(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    let hl_location_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    let new_value = registers.accumulator | hl_location_value;
    registers.flags = 0x00;
    if new_value == 0x00{
        registers.flags |= 0x80;
    }
    registers.accumulator = new_value;
}

pub fn decrement_hl_location(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    let hl_location_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    let mut new_value = 0;
    if hl_location_value == 0
    {
        new_value = 0xFF
    }
    else{
        new_value = hl_location_value - 1;
    }

    registers.flags &= 0x10;
    registers.flags |= 0x40;
    system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
    if new_value & 0x0F == 0x0F
    {
        registers.flags |= 0x20;
    }
    if new_value == 0x00
    {
        registers.flags |= 0x80;
    }
}

pub fn load_de_location_with_accumulator(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    let hl_location = registers.mapped_16_bit_register_getter(2);
    system_data.mmu.set_to_memory(hl_location as usize, registers.accumulator, true);
}

pub fn load_hl_with_stack_pointer_plus_n(system_data: &mut SystemData, registers: &mut Registers)
{
    let unsigned_n_value = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
    let n_value: i32 = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as i8 as i32;
    let stack_pointer = registers.stack_pointer as i32;
    let mut new_value = stack_pointer + n_value;
    registers.flags = 0x00;

    if (unsigned_n_value & 0x0F) + (stack_pointer as u16 & 0x0F) >= 0x10
    {
        registers.flags |= 0x20;
    }
    if (unsigned_n_value & 0xFF) + (stack_pointer as u16 & 0xFF) >= 0x100
    {
        registers.flags |= 0x10;
    }
    if new_value >= 0x10000
    {
        new_value -= 0x10000;
    }
    else if new_value < 0
    {
        new_value += 0x10000;
    }
    registers.mapped_16_bit_register_setter(3, new_value as u16);
    registers.program_counter += 2;
}

pub fn compare_register_to_accumulator(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    registers.flags = 0x40;
    let mut register_code = 0;
    match opcode
    {
        0xBF => register_code = 0,
        0xB8 => register_code = 1,
        0xB9 => register_code = 2,
        0xBA => register_code = 3,
        0xBB => register_code = 4,
        0xBC => register_code = 5,
        0xBD => register_code = 6,
        _ => 
        {
            println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
            return;
        },
    }
    registers.program_counter +=1; 
    let register_value = registers.mapped_register_getter(register_code);
    if registers.accumulator == register_value
    {
        registers.flags |= 0x80;
    }
    if register_value > registers.accumulator
    {
        registers.flags |= 0x10;
    }
    if (register_value & 0x0F) > (registers.accumulator & 0x0F)
    {
        registers.flags |= 0x20;
    }
}

pub fn load_hl_to_stack_pointer(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.stack_pointer = registers.mapped_16_bit_register_getter(3);
}

pub fn bcd_adjust(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    let mut add_value = 0;
    let mut carry_set = 0x00;
    let n_flag = (registers.flags & 0x40) >> 6;
    let h_flag = (registers.flags & 0x20) >> 5;
    let c_flag = (registers.flags & 0x10) >> 4;
    let upper = (registers.accumulator & 0xF0) >> 4;
    let lower = (registers.accumulator & 0x0F);

    if n_flag == 0
    {
        if (c_flag == 1) || (registers.accumulator > 0x99)
        {
            add_value += 0x60;
            carry_set = 0x10;
        }
        if (h_flag == 1) || ((registers.accumulator & 0x0F) > 0x09)
        {
            add_value += 0x06;
        }
    }
    else 
    {
        if c_flag == 1 && h_flag == 1
        {
           add_value = 0x9A;
           carry_set = 0x10;
        }
        
        else if c_flag == 1
        {
            add_value = 0xA0;
            carry_set = 0x10;
        }

        else if h_flag == 1
        {
            add_value = 0xFA;
        }
    }
    registers.flags &= 0x40;
    let mut new_value = registers.accumulator as u16 + add_value as u16;
    if (new_value & 0x00FF) == 0x00
    {
        registers.flags |= 0x80;
    }
    registers.flags |= carry_set;
    registers.accumulator = (new_value & 0x00FF) as u8;
}

pub fn rlca(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.flags = 0;
    let set_bit = (registers.accumulator & 0x80) >> 7;
    registers.accumulator = registers.accumulator << 1;
    registers.accumulator |= set_bit;
    registers.flags |= (set_bit << 4);
}

pub fn rrca(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.flags = 0;
    let set_bit = (registers.accumulator & 0x01);
    registers.accumulator = registers.accumulator >> 1;
    registers.accumulator |= set_bit << 7;
    registers.flags |= (set_bit << 4);
}

pub fn set_carry_flag(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    registers.flags &= 0x80;
    registers.flags |= 0x10;
}

pub fn flip_carry_flag(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    let carry_set = (registers.flags ^ 0xFF) & 0x10;
    registers.flags &= 0x80;
    registers.flags |= carry_set;
}

pub fn load_accumulator_to_address_at_bc(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.program_counter += 1;
    system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(1) as usize, registers.accumulator, true);
}

pub fn add_signed_8_bit_to_stack_pointer(system_data: &mut SystemData, registers: &mut Registers)
{
    registers.flags = 0x00;
    let unsigned_flag_calc = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as u16;
    let add_value = system_data.mmu.get_from_memory(registers.program_counter as usize + 1, false) as i8 as i32;
    let stack_ponter_value = registers.stack_pointer as i32;
    // if (add_value & 0x0F) + (stack_ponter_value & 0x0F) >= 0x10 && add_value > 0
    if (unsigned_flag_calc & 0x0F) + (stack_ponter_value as u16 & 0x0F) >= 0x10
    {
        registers.flags |= 0x20;
    }
    //if (add_value & 0xF0) + (stack_ponter_value & 0xF0) >= 0x100 && add_value > 0
    if (unsigned_flag_calc & 0xFF) + (stack_ponter_value as u16 & 0xFF) >= 0x100
    {
        registers.flags |= 0x10;
    }
    if add_value + stack_ponter_value >= 0x10000
    {
        registers.stack_pointer = (add_value + stack_ponter_value - 0x10000) as u16;
        
    }
    else if add_value + stack_ponter_value < 0
    {
        registers.stack_pointer = (add_value + stack_ponter_value + 0x10000) as u16;
    }
    else 
    {
        registers.stack_pointer = (add_value + stack_ponter_value) as u16;
    }
    registers.program_counter += 2;
}

//##########################################################################
//##########################################################################
//##########################################################################
//#########################################################################
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
    let opcode :u8 = system_data.mmu.get_from_memory((registers.program_counter + 1) as usize, false);
    
    system_data.cycles = cb_cycle_parse(opcode);

    match opcode
    {
        0x00...0x07 => rotate_register_left_carry_set(&mut system_data, &mut registers, opcode),
        0x08...0x0F => rotate_register_right_carry_set(&mut system_data, &mut registers, opcode),
        0x10...0x17 => rotate_left_through_carry(&mut system_data, &mut registers, opcode),
        0x18...0x1F=> rotate_right_through_carry(&mut system_data, &mut registers, opcode),
        0x20...0x27 => shift_left_load_carry(&mut system_data, &mut registers, opcode),
        0x28...0x2F => shift_right_load_carry(&mut system_data, &mut registers, opcode),
        0x30...0x37 => swap_nibbles(&mut system_data, &mut registers, opcode),
        0x38...0x3F => shift_right_register_logical(&mut system_data, &mut registers, opcode),
        0x40...0x7F => bit_check_register(&mut system_data, &mut registers, opcode, (opcode & 0x38) >> 3),
        0x80...0xBF => reset_bit_in_register(&mut system_data, &mut registers, opcode), 
        0xC0...0xFF => set_bit_in_register(&mut system_data, &mut registers, opcode),
        _ => (),
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
        registers.flags = registers.flags & 0x10;
        if (system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize ,true) >> test_bit) & 0x01 == 0x00
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
    let mut val = 0;
    if register_code == 7
    {
        val = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
        val = registers.mapped_register_getter(register_code); 
    }
    if (val & 0x80) == 0x80{
        registers.flags = registers.flags | 0x10;
    }
    val = val << 1;
    val = val | carry_bit;
    if val == 0
    {
        registers.flags = registers.flags | 0x80;
    }
    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, val, true);
    }
    else 
    {
        registers.mapped_register_setter(register_code, val);
    }
    registers.program_counter += 2;
}

pub fn swap_nibbles(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 8
    {
        register_code = 0;
    }
    let mut previous_value = 0;
    if register_code == 7
    {
        previous_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
        previous_value = registers.mapped_register_getter(register_code); 
    }
    let high_nibble = previous_value & 0xF0;
    let low_nibble = previous_value & 0x0F;
    let new_value = (high_nibble >> 4) | (low_nibble << 4);
    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
    }
    else 
    {
        registers.mapped_register_setter(register_code, new_value);
    }
    registers.flags = 0x00;
    if new_value == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}

pub fn set_bit_in_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = (opcode & 0x07) + 1;
    if register_code == 7
    {
        let bit_shift = (opcode & 0x38) >> 3;
        let new_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize,true) | (0x01 << bit_shift);
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
        registers.program_counter += 2;
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

pub fn shift_right_register_logical(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = ((opcode & 0x07) + 1) % 8;
    registers.flags = 0x00;

    let mut current_value = 0;
    if register_code == 7
    {
        current_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
        current_value = registers.mapped_register_getter(register_code);
    }
    
    let set_value = current_value >> 1;
    if current_value & 0x01 == 0x01
    {
        registers.flags |= 0x10;
    }
    if set_value == 0x00
    {
        registers.flags |= 0x80;
    }

    registers.program_counter += 2;
    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, set_value, true);
    }
    else
    {
        registers.mapped_register_setter(register_code, set_value);
    }
}


pub fn rotate_right_through_carry(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = ((opcode & 0x07) + 1) % 8;

    registers.program_counter += 2;
    let carry_bit = (registers.flags & 0x10) << 3;
    let mut new_value = 0;
    if register_code == 7
    {
        new_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
        new_value = registers.mapped_register_getter(register_code);
    }
    
    let carry_set_bit = (new_value & 0x01) << 4;
    registers.flags = 0x00;
    registers.flags |= carry_set_bit;
    new_value = (new_value >> 1) | carry_bit;
    if new_value == 0
    {
        registers.flags |= 0x80;
    }

    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
    }
    else
    {
        registers.mapped_register_setter(register_code, new_value);
    }
}

pub fn reset_bit_in_register(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = 0;
    match opcode
    {
        0x87 | 0x8F | 0x97 | 0x9F | 0xA7 | 0xAF | 0xB7 | 0xBF => register_code = 0,
        0x80 | 0x88 | 0x90 | 0x98 | 0xA0 | 0xA8 | 0xB0 | 0xB8 => register_code = 1,
        0x81 | 0x89 | 0x91 | 0x99 | 0xA1 | 0xA9 | 0xB1 | 0xB9 => register_code = 2, 
        0x82 | 0x8A | 0x92 | 0x9A | 0xA2 | 0xAA | 0xB2 | 0xBA => register_code = 3, 
        0x83 | 0x8B | 0x93 | 0x9B | 0xA3 | 0xAB | 0xB3 | 0xBB => register_code = 4, 
        0x84 | 0x8C | 0x94 | 0x9C | 0xA4 | 0xAC | 0xB4 | 0xBC => register_code = 5, 
        0x85 | 0x8D | 0x95 | 0x9D | 0xA5 | 0xAD | 0xB5 | 0xBD => register_code = 6, 
        _ => {
            let bit_shift = (opcode & 0x38) >> 3;
            let start_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
            let bit_removal = 0xFF ^ (0x01 << bit_shift);
            system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, start_value & bit_removal, true);
            registers.program_counter += 2;
            return;
        }, 
    }

    let bit_shift = (opcode & 0x38) >> 3;
    let start_value = registers.mapped_register_getter(register_code);
    let bit_removal = 0xFF ^ (0x01 << bit_shift);
    registers.mapped_register_setter(register_code, start_value & bit_removal);
    registers.program_counter += 2;
}

pub fn rotate_register_left_carry_set(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = 0;
    match opcode
    {
        0x07 => register_code = 0,
        0x00 => register_code = 1,
        0x01 => register_code = 2,
        0x02 => register_code = 3,
        0x03 => register_code = 4,
        0x04 => register_code = 5,
        0x05 => register_code = 6,
        0x06 => register_code = 7,
        _ => {
            println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
            return;
        },
    }

    registers.flags = 0x00;
    let mut current_value = 0;
    if register_code == 7
    {
        current_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
       current_value = registers.mapped_register_getter(register_code); 
    }
    let carry_bit = (current_value >> 7);
    let new_value = (current_value << 1) | carry_bit;
    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
    }
    else 
    {
        registers.mapped_register_setter(register_code, new_value);
    }
    if new_value == 0
    {
        registers.flags |= 0x80;
    }
    registers.flags |= (carry_bit << 4);
    registers.program_counter += 2;
}

pub fn rotate_register_right_carry_set(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = 0;
    match opcode
    {
        0x0F => register_code = 0,
        0x08 => register_code = 1,
        0x09 => register_code = 2,
        0x0A => register_code = 3,
        0x0B => register_code = 4,
        0x0C => register_code = 5,
        0x0D => register_code = 6,
        0x0E => register_code = 7,
        _ => {
            println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
            return;
        },
    }

    registers.flags = 0x00;
    let mut current_value = 0;
    if register_code == 7
    {
        current_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
       current_value = registers.mapped_register_getter(register_code); 
    }
    let carry_bit = current_value << 7;
    let new_value = (current_value >> 1) | carry_bit;
    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
    }
    else 
    {
        registers.mapped_register_setter(register_code, new_value);
    }
    if new_value == 0
    {
        registers.flags |= 0x80;
    }
    registers.flags |= (carry_bit >> 3);
    registers.program_counter += 2;
}

pub fn shift_left_load_carry(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = 0;
    match opcode
    {
        0x27 => register_code = 0,
        0x20 => register_code = 1,
        0x21 => register_code = 2,
        0x22 => register_code = 3,
        0x23 => register_code = 4,
        0x24 => register_code = 5,
        0x25 => register_code = 6,
        0x26 => register_code = 7,
        _ => {
            println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
            return;
        },
    }
    registers.flags = 0x00;
    let mut current_value = 0;
    if register_code == 7
    {
        current_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else {
        current_value = registers.mapped_register_getter(register_code);
    }
    let carry_bit = (current_value & 0x80) >> 3;
    registers.flags |= carry_bit;
    let new_value = current_value << 1;
    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
    }
    else 
    {
        registers.mapped_register_setter(register_code, new_value);
    }
    if new_value == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}

pub fn shift_right_load_carry(system_data: &mut SystemData, registers: &mut Registers, opcode: u8)
{
    let mut register_code = 0;
    match opcode
    {
        0x2F => register_code = 0,
        0x28 => register_code = 1,
        0x29 => register_code = 2,
        0x2A => register_code = 3,
        0x2B => register_code = 4,
        0x2C => register_code = 5,
        0x2D => register_code = 6,
        0x2E => register_code = 7,
        _ => {
            println!("No Opcode Found - 0x{:X} --- 0x{:X}", registers.program_counter, opcode);
            return;
        },
    }
    registers.flags = 0x00;
    let mut current_value = 0;
    if register_code == 7
    {
        current_value = system_data.mmu.get_from_memory(registers.mapped_16_bit_register_getter(3) as usize, true);
    }
    else 
    {
        current_value = registers.mapped_register_getter(register_code);
    }
    let carry_bit = (current_value & 0x01) << 4;
    registers.flags |= carry_bit;
    let keep_bit = current_value & 0x80;
    let new_value = (current_value >> 1) | keep_bit;
    if register_code == 7
    {
        system_data.mmu.set_to_memory(registers.mapped_16_bit_register_getter(3) as usize, new_value, true);
    }
    else 
    {
        registers.mapped_register_setter(register_code, new_value);
    }
    if new_value == 0
    {
        registers.flags |= 0x80;
    }
    registers.program_counter += 2;
}