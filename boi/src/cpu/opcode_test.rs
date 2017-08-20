#[cfg(test)]
mod opcode_test
{
    extern crate csv;
    extern crate hex;

    use self::hex::FromHex;
    use system::*;
    use cpu::opcode::*;
    use std::env;
    use std::error::Error;
    use std::ffi::OsString;
    use std::fs::File;
    use std::process;

    #[test]
    fn increments_propper_register()
    {   

        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        let opcodes: Vec<u8> = vec![0x3C, 0x04, 0x0C, 0x14, 0x1C, 0x24, 0x2C];

        for i in 0..7
        {
            increment(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i), 1);
            registers.program_counter -= 1;
        }
    }

    #[test]
    fn loads_n_to_correct_register_8_bit() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        let opcodes : Vec<u8> = vec![0x3E, 0x06, 0x0E, 0x16, 0x1E, 0x26, 0x2E];
        system_data.mem_map[1] = 1;

        for i in 0..7
        {
            load_n_to_8bit_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i), 1);
            registers.program_counter -= 2;   
        }
    }

    #[test]
    fn accumulator_io_load_with_c_offset() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.accumulator = 1;
        registers.c_register = 1;
        load_accumulator_to_io_port_with_c_offset(&mut system_data, &mut registers);
        assert_eq!(system_data.mem_map[0xFF01], 1);
    }

    #[test]
    fn loads_nn_to_correct_register_16_bit() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        system_data.mem_map[1] = 0x01;
        system_data.mem_map[2] = 0x02;
        //bc
        let opcode = 0x01;
        load_nn_to_16bit_register(&mut system_data, &mut registers, opcode);
        assert_eq!(registers.b_register, 2);
        assert_eq!(registers.c_register, 1);
        //de
        registers.program_counter -= 3;
        let opcode = 0x11;
        load_nn_to_16bit_register(&mut system_data, &mut registers, opcode);
        assert_eq!(registers.d_register, 2);
        assert_eq!(registers.e_register, 1);
        //hl
        registers.program_counter -= 3;
        let opcode = 0x21;
        load_nn_to_16bit_register(&mut system_data, &mut registers, opcode);
        assert_eq!(registers.h_register, 2);
        assert_eq!(registers.l_register, 1);
        //stack pointer
        registers.program_counter -= 3;
        let opcode = 0x31;
        load_nn_to_16bit_register(&mut system_data, &mut registers, opcode);
        assert_eq!(registers.stack_pointer, 0x0201);
        
    }
    
    #[test]
    fn xor_register_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        let opcode = 0xAF;
        //A
        registers.accumulator = 0xFF;
        xor_register(&mut system_data, &mut registers, opcode);
        assert_eq!(registers.accumulator, 0);
    }
        
    #[test]
    fn pc_jumps_displacement_on_nonzero_flag() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.program_counter = 100;
        //Positive Jump
        system_data.mem_map[101] = 10;
        registers.flags = 0x00;
        jump_displacement_on_nonzero_flag(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 112);
        //Negative Jump
        registers.program_counter = 100;
        system_data.mem_map[101] = 253;
        jump_displacement_on_nonzero_flag(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 99);
        //Zero Flag
        registers.program_counter = 100;
        system_data.mem_map[101] = 0xFF;
        registers.flags = 0xFF;
        jump_displacement_on_nonzero_flag(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 102)
    }
    
    #[test]
    fn load_decrement_hl_register_location_with_accumulator_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        //Normal increment
        registers.h_register = 0xFF;
        registers.l_register = 0xFF;
        registers.accumulator = 1;
        load_decrement_hl_register_location_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(system_data.mem_map[0xFFFF], 1);
        assert_eq!(registers.h_register, 0xFF);
        assert_eq!(registers.l_register, 0xFE);
    }

    #[test]
    fn bit_check_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        //H
        //Zero Flag
        let opcode = 0x7C;
        let test_bit = 7;
        registers.h_register = 0x00;
        bit_check_register(&mut system_data, &mut registers, opcode, test_bit);
        assert_eq!(registers.flags, 0xA0);
        //No Zero Flag
        registers.h_register = 0xFF;
        bit_check_register(&mut system_data, &mut registers, opcode, test_bit);
        assert_eq!(registers.flags, 0x20);
    }
    
    #[test]
    fn load_hl_address_with_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        let opcodes: Vec<u8> = vec![0x77, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75];
        let register_values: Vec<u8> = vec![1,2,3,4,5,0xFF,0xFF];

        for i in 0..7
        {
            registers.mapped_register_setter(i, register_values[i as usize]);
        }

        for i in 0..7
        {
            load_hl_address_with_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(system_data.mem_map[0xFFFF], register_values[i as usize]);
        }

    }

    #[test]
    fn load_accumulator_to_io_port_with_a_offset_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.accumulator = 1;
        system_data.mem_map[1] = 1;
        load_accumulator_to_io_port_with_n_offset(&mut system_data, &mut registers);
        assert_eq!(system_data.mem_map[0xFF01], 1);
    }

    #[test]
    fn load_8_bit_register_to_register_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        let test_file_raw = File::open("src/cpu/ld_r_r.csv").unwrap();
        let mut test_file = csv::ReaderBuilder::new().has_headers(false).from_reader(test_file_raw);
        let mut asserts: Vec<String> = Vec::new();
        let mut opcodes: Vec<String> = Vec::new();
        for record_raw in test_file.records()
        {
            let record = &(record_raw.unwrap());
            asserts.push(record[0].to_string());
            opcodes.push(record[1].to_string());
        }

        for index in 0..asserts.len()
        {
            if(index % 7 == 0){
                for i in 0..7 
                {
                    registers.mapped_register_setter(i, i + 1)
                }
           }
            let opcode = Vec::from_hex(&opcodes[index]).unwrap();
            load_8_bit_register_to_register(&mut system_data, &mut registers, opcode[0]);
            assert_eq!(asserts[index].parse::<u8>().unwrap(), registers.mapped_register_getter((index / 7) as u8));
        }
    }

    #[test]
    fn load_accumulator_with_de_address_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.d_register = 0xFF;
        registers.e_register = 0xEE;
        system_data.mem_map[0xFFEE] = 1;
        load_accumulator_with_de_address(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 1);
    }

    // #[test]
    // fn call_nn_test() {
    //     unimplemented!();
    // }
}