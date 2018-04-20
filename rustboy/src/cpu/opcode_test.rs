#[cfg(test)]
mod opcode_test
{
    extern crate csv;
    extern crate hex;

    use self::hex::FromHex;
    use system::*;
    use cpu::opcode::*;
    use std::fs::File;

    #[test]
    fn increments_8_bit_register_test()
    {   

        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x3C, 0x04, 0x0C, 0x14, 0x1C, 0x24, 0x2C];

        //Normal flag
        for i in 0..7
        {
            increment_8_bit_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i), 1);
            assert_eq!(registers.flags, 0x00);
        }
        //Half flag
        for i in 0..7
        {
            registers.mapped_register_setter(i as u8, 0x0F);
            increment_8_bit_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x10);
            assert_eq!(registers.flags, 0x20);
        }

        //Zero flag
        for i in 0..7
        {
            registers.mapped_register_setter(i as u8, 0xFF);
            increment_8_bit_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x00);
            assert_eq!(registers.flags, 0xA0);
        }
    }

    #[test]
    fn increment_hl_location_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();

        registers.mapped_16_bit_register_setter(3, 0x1234);

        //Normal flag
        increment_hl_location(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0x1234], 1);
        assert_eq!(registers.flags, 0x00);
        //Half flag
        system_data.mmu.mem_map[0x1234] = 0x0F;
        increment_hl_location(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x10);
        assert_eq!(registers.flags, 0x20);

        //Zero flag
        system_data.mmu.mem_map[0x1234] = 0xFF;
        increment_hl_location(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x00);
        assert_eq!(registers.flags, 0xA0);
    }


    #[test]
    fn increment_16_bit_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x03, 0x13, 0x23, 0x33];
        //TODO: ADD FLAG TESTS
        for i in 1..5
        {
            registers.mapped_16_bit_register_setter(i as u8, 0x00FE);
            increment_16_bit_register(&mut system_data, &mut registers, opcodes[i - 1]);
            assert_eq!(registers.mapped_16_bit_register_getter(i as u8), 0x00FF);
            registers.mapped_16_bit_register_setter(i as u8, 0xFFFF);
            increment_16_bit_register(&mut system_data, &mut registers, opcodes[i - 1]);
            assert_eq!(registers.mapped_16_bit_register_getter(i as u8), 0x0000);
        }
    }


    #[test]
    fn decrement_8_bit_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x3D, 0x05, 0x0D, 0x15, 0x1D, 0x25, 0x2D];
        
        //Normal flag
        for i in 0..7
        {
            registers.mapped_register_setter(i as u8, 2);
            decrement_8_bit_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i), 1);
            assert_eq!(registers.flags, 0x40);
        }

        //Half Flag
        for i in 0..7
        {
            registers.mapped_register_setter(i as u8, 0x10);
            decrement_8_bit_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i), 0x0F);
            assert_eq!(registers.flags, 0x60);
        }

        //Zero Flag
        for i in 0..7
        {
            registers.mapped_register_setter(i as u8, 1);
            decrement_8_bit_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.mapped_register_getter(i), 0);
            assert_eq!(registers.flags, 0xC0);
        }
    }

    #[test]
    fn decrement_16_bit_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x0B, 0x1B, 0x2B, 0x3B];
        for i in 1..5
        {
            registers.mapped_16_bit_register_setter(i as u8, 0x0100);
            decrement_16_bit_register(&mut system_data, &mut registers, opcodes[i - 1]);
            assert_eq!(registers.mapped_16_bit_register_getter(i as u8), 0x00FF);
            registers.mapped_16_bit_register_setter(i as u8, 0x0000);
            decrement_16_bit_register(&mut system_data, &mut registers, opcodes[i - 1]);
            assert_eq!(registers.mapped_16_bit_register_getter(i as u8), 0xFFFF);
        }
    }

    #[test]
    fn loads_n_to_correct_register_8_bit() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes : Vec<u8> = vec![0x3E, 0x06, 0x0E, 0x16, 0x1E, 0x26, 0x2E];
        system_data.mmu.mem_map[1] = 1;

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
        let mut registers : Registers = Registers::new();
        registers.accumulator = 1;
        registers.c_register = 1;
        load_accumulator_to_io_port_with_c_offset(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0xFF01], 1);
    }

    #[test]
    fn read_io_port_with_c_offset_to_accumulator_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.c_register = 1;
        system_data.mmu.mem_map[0xFF01] = 0xFF;
        read_io_port_with_c_offset_to_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0xFF);
    }

    #[test]
    fn loads_nn_to_correct_register_16_bit() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[1] = 0x01;
        system_data.mmu.mem_map[2] = 0x02;
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
    fn xor_8_bit_register_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAF];
        for register in 0..7
        {
            registers.mapped_register_setter(register, 0xFF);
        }

        for opcode in 0..opcodes.len()
        {
            registers.accumulator = 0xFF;
            xor_8_bit_register(&mut system_data, &mut registers, opcodes[opcode]);
            assert_eq!(registers.accumulator, 0);
            assert_eq!(registers.flags, 0x80);
        }
    }

    #[test]
    fn xor_hl_location_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //Zero
        registers.accumulator = 0xFF;
        registers.mapped_16_bit_register_setter(3, 0x1234);
        system_data.mmu.mem_map[0x1234] = 0xFF;
        xor_hl_location(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x80);
        assert_eq!(registers.accumulator, 0x00);
        
        registers.accumulator = 0x00;
        registers.mapped_16_bit_register_setter(3, 0x1234);
        system_data.mmu.mem_map[0x1234] = 0xFF;
        xor_hl_location(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x00);
        assert_eq!(registers.accumulator, 0xFF);


    }

    #[test]
    fn or_8_bit_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5];
        for register in 0..7
        {
            registers.mapped_register_setter(register, 0x0F);
        }

        for opcode in 0..opcodes.len()
        {
            registers.accumulator = 0xF0;
            or_8_bit_register(&mut system_data, &mut registers, opcodes[opcode]);
            assert_eq!(registers.accumulator, 0xFF);
        }

        //Zero flag
        registers.accumulator = 0x00;
        or_8_bit_register(&mut system_data, &mut registers, 0xB7);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0x80);
    }

    #[test]
    fn and_8_bit_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5];
        for register in 0..7
        {
            registers.mapped_register_setter(register, 0x00);
        }

        for opcode in 0..opcodes.len()
        {
            registers.accumulator = 0xFF;
            and_8_bit_register(&mut system_data, &mut registers, opcodes[opcode]);
            assert_eq!(registers.accumulator, 0);
            assert_eq!(registers.flags, 0xA0);
        }

        //NZ Flag
        registers.accumulator = 0xFF;
        and_8_bit_register(&mut system_data, &mut registers, 0xA7);
        assert_eq!(registers.accumulator, 0xFF);
        assert_eq!(registers.flags, 0x20);
    }
        
    
    #[test]
    fn load_decrement_hl_register_location_with_accumulator_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //Normal increment
        registers.h_register = 0xFF;
        registers.l_register = 0xFF;
        registers.accumulator = 1;
        load_decrement_hl_register_location_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0xFFFF], 1);
        assert_eq!(registers.h_register, 0xFF);
        assert_eq!(registers.l_register, 0xFE);
    }

    #[test]
    fn load_increment_hl_register_location_with_accumulator_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.h_register = 0xFF;
        registers.l_register = 0xFE;
        registers.accumulator = 1;
        load_increment_hl_register_location_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0xFFFE], 1);
        assert_eq!(registers.h_register, 0xFF);
        assert_eq!(registers.l_register, 0xFF);
    }

    //TODO: Write a better test for this
    #[test]
    fn bit_check_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let test_file_raw = File::open("src/cpu/test_csvs/bit_check.csv").unwrap();
        let mut test_file = csv::ReaderBuilder::new().has_headers(false).from_reader(test_file_raw);
        let mut opcodes: Vec<String> = Vec::new();
        for record_raw in test_file.records()
        {
            let record = &(record_raw.unwrap());
            opcodes.push(record[0].to_string());
        }

        for i in 0..8
        {
            for j in 0..7
            {
                let opcode = Vec::from_hex(&opcodes[(i * 7) + j]).unwrap();
                registers.mapped_register_setter(j as u8, 0x00);
                registers.flags = 0x00;
                bit_check_register(&mut system_data, &mut registers, opcode[0], j as u8);
                assert_eq!(registers.flags, 0xA0);
                registers.mapped_register_setter(j as u8, 0xFF);
                registers.flags = 0x00;
                bit_check_register(&mut system_data, &mut registers, opcode[0], j as u8);
                assert_eq!(registers.flags, 0x20)
            }
        }
    }
    
    #[test]
    fn load_hl_address_with_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x77, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75];
        let register_values: Vec<u8> = vec![1,2,3,4,5,0xFF,0xFF];

        for i in 0..7
        {
            registers.mapped_register_setter(i, register_values[i as usize]);
        }

        for i in 0..7
        {
            load_hl_address_with_register(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(system_data.mmu.mem_map[0xFFFF], register_values[i as usize]);
        }

    }

    #[test]
    fn load_accumulator_to_io_port_with_a_offset_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.accumulator = 1;
        system_data.mmu.mem_map[1] = 1;
        load_accumulator_to_io_port_with_n_offset(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0xFF01], 1);
    }

    #[test]
    fn load_8_bit_register_to_register_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let test_file_raw = File::open("src/cpu/test_csvs/ld_r_r.csv").unwrap();
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
            if index % 7 == 0 {
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
        let mut registers : Registers = Registers::new();
        registers.d_register = 0xFF;
        registers.e_register = 0xEE;
        system_data.mmu.mem_map[0xFFEE] = 1;
        load_accumulator_with_de_address(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 1);
    }

    #[test]
    fn load_accumulator_with_bc_address_test()
    {
        //Todo - combine 0x0A with 0x1A
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.b_register = 0xFF;
        registers.c_register = 0xEE;
        system_data.mmu.mem_map[0xFFEE] = 1;
        load_accumulator_with_bc_address(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 1);
    }

    #[test]
    fn call_nn_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.program_counter = 1;
        system_data.mmu.mem_map[2] = 0xCD;
        system_data.mmu.mem_map[3] = 0xAB;
        registers.stack_pointer = 0xFFFE;
        call_nn(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0xABCD);
        assert_eq!(registers.stack_pointer, 0xFFFC);
        assert_eq!(system_data.mmu.mem_map[registers.stack_pointer as usize], 4);
    }

    #[test]
    fn return_from_calltest() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x2000] = 0xFF;
        system_data.mmu.mem_map[0x2001] = 0xEE;
        registers.stack_pointer = 0x2000;
        return_from_call(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0xEEFF);
        assert_eq!(registers.stack_pointer, 0x2002);
    }

    #[test]
    fn push_16_bit_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xF5, 0xC5, 0xD5, 0xE5];
        registers.stack_pointer = 0xFFFE;
        for i in 0..8
        {
            registers.mapped_register_setter_with_flags(i, i);
        }
        for i in 0..4
        {
            push_16_bit_register(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(system_data.mmu.mem_map[registers.stack_pointer as usize + 1], i as u8 * 2);
            assert_eq!(system_data.mmu.mem_map[registers.stack_pointer as usize], (i as u8 * 2) + 1);
        }
    }

    #[test]
    fn pop_16_bit_register_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xF1, 0xC1, 0xD1, 0xE1];
        registers.stack_pointer = 0xFFF0;
        for i in 0..8
        {
            system_data.mmu.mem_map[registers.stack_pointer as usize + i] = i as u8; 
        }
        for i in 0..4
        {
            pop_16_bit_register(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter_with_flags(i as u8 * 2), (i as u8 * 2) + 1);
            assert_eq!(registers.mapped_register_getter_with_flags((i as u8 * 2) + 1), i as u8 * 2);
        }

    }

    #[test]
    fn rotate_left_through_carry_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes : Vec<u8> = vec![0x17, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15];
        for i in 0..7
        {
            registers.mapped_register_setter(i as u8, 0xBF);
            registers.flags = 0x00;
            //Carry
            rotate_left_through_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.flags, 0x10);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x7E);
            //No Carry
            rotate_left_through_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.flags, 0x00);
            assert_eq!(registers.mapped_register_getter(i as u8), 0xFD);
        }
    }

    #[test]
    fn rotate_accumulator_left_through_carry_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //Carry
        registers.accumulator = 0x80;
        rotate_accumulator_left_through_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0);
        //No Carry
        rotate_accumulator_left_through_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 1);
    }

    #[test]
    fn compare_with_n_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.accumulator = 0xF1;
        //Normal
        system_data.mmu.mem_map[1] = 0x01;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x40);
        //Carry
        system_data.mmu.mem_map[3] = 0x0F2;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x70);
        //Half Carry
        system_data.mmu.mem_map[5] = 0x02;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x60);
        //Zero
        system_data.mmu.mem_map[7] = 0xF1;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0xC0);
    }

    #[test]
    fn load_nn_with_accumulator_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.accumulator = 1;
        system_data.mmu.mem_map[1] = 0xEE;
        system_data.mmu.mem_map[2] = 0xFF;
        load_nn_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0xFFEE], 1);
    }
    #[test]
    fn load_accumulator_with_io_port_with_n_offset_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0xFF0F] = 1;
        system_data.mmu.mem_map[1] = 0x0F;
        load_accumulator_with_io_port_with_n_offset(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 1);
    }
    
    #[test]
    fn jump_displacement_on_flag_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x20, 0x28, 0x30, 0x38];
        let pass_flags: Vec<u8> = vec![0x00, 0x80, 0x00, 0x10];
        let fail_flags: Vec<u8> = vec![0x80, 0x00, 0x10, 0x00];
        let locations: Vec<u16> = vec![0x104, 0xFE];
        let location_jump_values = vec![0x02, 0xFC];
        for location_index in 0..locations.len()
        {
            //Pass
            for i in 0..pass_flags.len()
            {
                //println!("#####{}", i);
                registers.program_counter = 0x100;
                system_data.mmu.mem_map[0x101] = location_jump_values[location_index];
                registers.flags = pass_flags[i];
                jump_displacement_on_flag(&mut system_data, &mut registers, opcodes[i]);
                assert_eq!(registers.program_counter, locations[location_index]);               
            }
            //Fail
            for i in 0..fail_flags.len()
            {
                registers.program_counter = 0x100;
                system_data.mmu.mem_map[0x101] = location_jump_values[location_index];
                registers.flags = fail_flags[i];
                jump_displacement_on_flag(&mut system_data, &mut registers, opcodes[i]);
                assert_eq!(registers.program_counter, 0x102); 
            }
        }
    }

    #[test]
    fn jump_displacement_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[1] = 0xE;
        jump_displacement(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0x10);
        system_data.mmu.mem_map[0x11] = 0xFA;
        jump_displacement(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0xC);
        
    }

    #[test]
    fn jump_address_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x0000] = 0xC3;
        system_data.mmu.mem_map[0x0001] = 0x34;
        system_data.mmu.mem_map[0x0002] = 0x12;
        jump_address(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0x1234);
        assert_eq!(system_data.cycles, 4);
    }

    #[test]
    fn subract_8_bit_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x97, 0x90, 0x91,0x92,0x93,0x94,0x95];
        //normal
        for i in 1..7
        {
            registers.accumulator = 7;
            registers.mapped_register_setter(i as u8, i);        
            subtract_8_bit(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.accumulator, 7 - i);
            assert_eq!(registers.flags, 0x40);
        }
        //zero flag
        registers.accumulator = 1;
        subtract_8_bit(&mut system_data, &mut registers, opcodes[0]);
        assert_eq!(registers.accumulator, 0);
        assert_eq!(registers.flags, 0xC0);
        //Half carry
        registers.accumulator = 0x10;
        registers.b_register = 0x01;
        subtract_8_bit(&mut system_data, &mut registers, opcodes[1]);
        assert_eq!(registers.flags, 0x60);
        //Carry
        registers.accumulator = 0x00;
        registers.b_register = 0x01;
        subtract_8_bit(&mut system_data, &mut registers, opcodes[1]);
        assert_eq!(registers.accumulator, 0xFF);
        assert_eq!(registers.flags, 0x70);
    }


    #[test]
    fn compare_with_hl_address_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.accumulator = 0x19;
        registers.h_register = 0x12;
        registers.l_register = 0x34;
        system_data.mmu.mem_map[0x1234] = 0x04;
        //Normal
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x40);
        //Carry
        system_data.mmu.mem_map[0x1234] = 0xFF;
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x70);
        //Half Carry
        system_data.mmu.mem_map[0x1234] = 0x0F;
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x60);
        //Zero
        system_data.mmu.mem_map[0x1234] = 0x19;
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0xC0);
    }

    #[test]
    fn add_8_bit_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x87, 0x80, 0x81,0x82,0x83,0x84,0x85,0x86];
        for i in 1..7
        {
            registers.mapped_register_setter(i as u8, 1);
        }

        //normal
        for i in 0..7
        {
            registers.accumulator = 1;
            add_8_bit(&mut system_data, &mut registers, opcodes[i as usize]);
            assert_eq!(registers.accumulator, 2);
            assert_eq!(registers.flags, 0x00);
        }

        registers.h_register = 0xFF;
        registers.l_register = 0xEE;
        system_data.mmu.mem_map[0xFFEE] = 1;
        registers.accumulator = 1;
        add_8_bit(&mut system_data, &mut registers, opcodes[7]);
        assert_eq!(registers.accumulator, 2);
        assert_eq!(registers.flags, 0x00);

        //zero
        registers.accumulator = 0;
        add_8_bit(&mut system_data, &mut registers, opcodes[0]);
        assert_eq!(registers.flags, 0x80);

        //half
        registers.accumulator = 0x0F;
        add_8_bit(&mut system_data, &mut registers, opcodes[0]);
        assert_eq!(registers.flags, 0x20);

        //carry
        registers.accumulator = 0xFF;
        add_8_bit(&mut system_data, &mut registers, opcodes[0]);
        assert_eq!(registers.flags, 0x30);
    }

    #[test]
    fn load_accumulator_with_hl_then_increment_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);
        system_data.mmu.mem_map[0x1234] = 0xFF;
        load_accumulator_with_hl_then_increment(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0xFF);
        assert_eq!(registers.mapped_16_bit_register_getter(3), 0x1235);
    }

    #[test]
    fn load_accumulator_with_hl_then_decrement_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);
        system_data.mmu.mem_map[0x1234] = 0xFF;
        load_accumulator_with_hl_then_decrement(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0xFF);
        assert_eq!(registers.mapped_16_bit_register_getter(3), 0x1233);
    }

    #[test]
    fn ones_complement_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.accumulator = 0b01010101;
        ones_complement(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0b10101010);
    }

    #[test]
    fn and_nn_with_accumulator_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x01] = 0x00;
        registers.accumulator = 0xFF;
        and_nn_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0xA0);
    }

    #[test]
    fn swap_nibbles_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x37, 0x30, 0x31,0x32,0x33,0x34,0x35];
        for i in 0..opcodes.len()
        {
            registers.mapped_register_setter(i as u8, 0x80);
            swap_nibbles(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x08);
            assert_eq!(registers.flags, 0x00);
        }

        registers.accumulator = 0;
        swap_nibbles(&mut system_data, &mut registers, 0x37);
        assert_eq!(registers.accumulator, 0);
        assert_eq!(registers.flags, 0x80);
    }

    #[test]
    fn rst_jump_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xC7, 0xCF, 0xD7, 0xDF, 0xE7, 0xEF, 0xF7, 0xFF];
        let locations: Vec<u16> = vec![0x0000, 0x0008, 0x0010, 0x0018, 0x0020, 0x0028, 0x0030, 0x0038];
        for i in 0..opcodes.len()
        {
            registers.program_counter = 0x1234;
            registers.stack_pointer = 0x0002;
            rst_jump(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(system_data.mmu.mem_map[0x0000], 0x35);
            assert_eq!(system_data.mmu.mem_map[0x0001], 0x12);
            assert_eq!(registers.stack_pointer, 0x0000);
            assert_eq!(registers.program_counter, locations[i]);

        }
    }


    #[test]
    fn add_16_bit_register_to_hl_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x09, 0x19, 0x29, 0x39];
        for i in 0..opcodes.len()
        {
            registers.mapped_16_bit_register_setter(3, 0x0001);
            registers.mapped_16_bit_register_setter(i as u8 + 1, 0x0001);
            add_16_bit_register_to_hl(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_16_bit_register_getter(3), 0x00002);
            assert_eq!(registers.flags, 0x00);
        }
        registers.mapped_16_bit_register_setter(3, 0xFFFFF);
        add_16_bit_register_to_hl(&mut system_data, &mut registers, 0x29);
        assert_eq!(registers.flags, 0x30);
    }

    #[test]
    fn jump_to_hl_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);
        jump_to_hl(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0x1234);
    }

    #[test]
    fn load_register_with_hl_location_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x1234] = 0xFF;
        let opcodes: Vec<u8> = vec![0x7E, 0x46, 0x4E, 0x56, 0x5E, 0x66, 0x6E];
        for i in 0..opcodes.len()
        {
            registers.mapped_16_bit_register_setter(3, 0x1234);
            load_register_with_hl_location(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0xFF);
        }
    }

    #[test]
    fn load_n_to_hl_location_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x0001] = 0xFF;
        registers.mapped_16_bit_register_setter(3, 0x1234);
        load_n_to_hl_location(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0xFF);
    }

    #[test]
    fn or_n_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x0001] = 0x00;
        system_data.mmu.mem_map[0x0003] = 0xFF;
        or_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x80);
        assert_eq!(registers.accumulator, 0x00);
        or_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x00);
        assert_eq!(registers.accumulator, 0xFF);
    }

    #[test]
    fn set_bit_in_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let test_file_raw = File::open("src/cpu/test_csvs/set_bit.csv").unwrap();
        let mut test_file = csv::ReaderBuilder::new().has_headers(false).from_reader(test_file_raw);
        let mut opcodes: Vec<String> = Vec::new();
        for record_raw in test_file.records()
        {
            let record = &(record_raw.unwrap());
            opcodes.push(record[0].to_string());
        }
        for i in 0..8
        {
            for j in 0..7
            {
                let opcode = Vec::from_hex(&opcodes[(i * 7) + j]).unwrap();
                registers.mapped_register_setter(j as u8, 0x00);
                set_bit_in_register(&mut system_data, &mut registers, opcode[0]);
                assert_eq!((registers.mapped_register_getter(j as u8) >> i) & 0x01, 0x01);
            }
        }
    }

    #[test]
    fn set_bit_of_hl_location_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xC6, 0xCE, 0xD6, 0xDE, 0xE6, 0xEE, 0xF6, 0xFE];
        registers.mapped_16_bit_register_setter(3, 0x1234);
        for i in 0..opcodes.len()
        {
            system_data.mmu.mem_map[0x1234] = 0;
            set_bit_in_register(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!((system_data.mmu.mem_map[0x1234] >> i) & 0x01, 0x01);
        }
    }

    #[test]
    fn subtract_register_and_carry_from_accumulator_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x9F, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D];
        //Without carry flag
        //Sets zero flag
        for i in 0..opcodes.len()
        {
            registers.accumulator = 0x02;
            registers.mapped_register_setter(i as u8, 0x02);
            subtract_register_and_carry_from_accumulator(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.accumulator, 0x00);
            assert_eq!(registers.flags, 0xC0);
            
        }

        //With carry flag
        //Sets carry flag        
        for i in 1..opcodes.len()
        {
            registers.flags = 0x10; 
            registers.accumulator = 0x11;
            registers.mapped_register_setter(i as u8, 0x20);
            subtract_register_and_carry_from_accumulator(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.accumulator, 0xF0);
            assert_eq!(registers.flags, 0x50);
        }

        //Half carry flag
        registers.flags = 0x10;
        registers.accumulator = 0x10;
        registers.b_register = 0x00;
        subtract_register_and_carry_from_accumulator(&mut system_data, &mut registers, 0x98);
        assert_eq!(registers.accumulator, 0x0F);
        assert_eq!(registers.flags, 0x60);
    }

    #[test]
    fn subtract_hl_location_and_carry_from_accumulator_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);
        //without carry
        //Sets carry
        registers.flags = 0x00;
        registers.accumulator = 0x10;
        system_data.mmu.mem_map[0x1234] = 0x20;
        subtract_hl_location_and_carry_from_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0xF0);
        assert_eq!(registers.flags, 0x50);

        //with carry
        //half carry
        registers.flags = 0x10;
        registers.accumulator = 0x20;
        system_data.mmu.mem_map[0x1234] = 0x00;
        subtract_hl_location_and_carry_from_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x1F);
        assert_eq!(registers.flags, 0x60);

        //zero flag
        registers.flags = 0x10;
        registers.accumulator = 0x02;
        system_data.mmu.mem_map[0x1234] = 0x01;
        subtract_hl_location_and_carry_from_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0xC0);
    }

    #[test]
    fn load_accumulator_with_nn_address_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x1234] = 0xFF;
        system_data.mmu.mem_map[0x0001] = 0x34;
        system_data.mmu.mem_map[0x0002] = 0x12;
        registers.accumulator = 0x00;
        load_accumulator_with_nn_address(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0xFF);
    }

    #[test]
    fn call_function_nn_on_conditional_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let flags: Vec<u8> = vec![0x00, 0x80, 0x00, 0x10];
        let fail_flags: Vec<u8> = vec![0x80, 0x00, 0x10, 0x00];
        let program_counters: Vec<usize> = vec![0x0703, 0x0501, 0x02FF, 0x00FD];    
        let opcodes: Vec<u8> = vec![0xC4, 0xCC, 0xD4, 0xDC];
        
        registers.stack_pointer = 0x08;

        for i in 0..flags.len()
        {
            registers.flags = flags[i];
            registers.program_counter = program_counters[i] as u16;   
            system_data.mmu.mem_map[program_counters[i] + 1]  = 0x34;
            system_data.mmu.mem_map[program_counters[i] + 2]  = 0x12;
            call_function_nn_on_conditional(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.program_counter, 0x1234);
        }

        for i in 0..flags.len()
        {
            registers.flags = fail_flags[i];
            registers.program_counter = 0x0000;   
            call_function_nn_on_conditional(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.program_counter, 0x0003);
        }

        for i in 0..8
        {
            assert_eq!(system_data.mmu.mem_map[i], i as u8);
        }

    }

    #[test]
    fn add_8_bit_to_accumulator_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //Zero flag
        registers.flags = 0x00;
        registers.accumulator = 0x00;
        system_data.mmu.mem_map[0x01] = 0x00;
        add_8_bit_to_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x80);
        assert_eq!(registers.accumulator, 0x00);

        //Carry flag
        registers.flags = 0x00;
        registers.accumulator = 0xF0;
        system_data.mmu.mem_map[0x03] = 0x11;
        add_8_bit_to_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x10);
        assert_eq!(registers.accumulator, 0x01);

        //Half Carry flag
        registers.flags = 0x00;
        registers.accumulator = 0x08;
        system_data.mmu.mem_map[0x05] = 0x08;
        add_8_bit_to_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x20);
        assert_eq!(registers.accumulator, 0x10);
    }

    #[test]
    fn subtract_n_from_accumulator_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();

        //Zero flag
        registers.flags = 0x00;
        registers.accumulator = 0x00;
        system_data.mmu.mem_map[0x01] = 0x00;
        subtraction_n_from_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0xC0);
        assert_eq!(registers.accumulator, 0x00);

        //Carry flag
        registers.flags = 0x00;
        registers.accumulator = 0x00;
        system_data.mmu.mem_map[0x03] = 0x10;
        subtraction_n_from_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x50);
        assert_eq!(registers.accumulator, 0xF0);

        //Half Carry flag
        registers.flags = 0x00;
        registers.accumulator = 0x10;
        system_data.mmu.mem_map[0x05] = 0x01;
        subtraction_n_from_accumulator(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x60);
        assert_eq!(registers.accumulator, 0x0F);
    }

    #[test]
    fn rotate_accumulator_right_through_carry_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //With Carry
        //Set
        registers.flags = 0x10;
        registers.accumulator = 0xFF;
        rotate_accumulator_right_through_carry(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x10);
        assert_eq!(registers.accumulator, 0xFF);
        //No Set
        registers.flags = 0x10;
        registers.accumulator = 0x00;
        rotate_accumulator_right_through_carry(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x00);
        assert_eq!(registers.accumulator, 0x80);

        //No Carry
        registers.flags = 0x00;
        registers.accumulator = 0x02;
        rotate_accumulator_right_through_carry(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x00);
        assert_eq!(registers.accumulator, 0x01);
    }

    #[test]
    fn add_8_bit_to_accumulator_with_carry_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //No Carry Set & Zero
        registers.flags = 0x00;
        registers.accumulator = 0x00;
        system_data.mmu.mem_map[0x01] = 0x00;
        add_8_bit_to_accumulator_with_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0x80);

        //Half Carry
        registers.flags = 0x10;
        registers.accumulator = 0x0F;
        system_data.mmu.mem_map[0x03] = 0x00;
        add_8_bit_to_accumulator_with_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x10);
        assert_eq!(registers.flags, 0x20);

        //Carry
        registers.flags = 0x10;
        registers.accumulator = 0xF0;
        system_data.mmu.mem_map[0x05] = 0x20;
        add_8_bit_to_accumulator_with_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x11);
        assert_eq!(registers.flags, 0x10);
    }

    #[test]
    fn subtract_8_bit_from_accumulator_with_carry_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //No Carry Set & Zero
        registers.flags = 0x00;
        registers.accumulator = 0x01;
        system_data.mmu.mem_map[0x01] = 0x01;
        subtract_8_bit_from_accumulator_with_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0xC0);

        //Half Carry
        registers.flags = 0x10;
        registers.accumulator = 0x10;
        system_data.mmu.mem_map[0x03] = 0x00;
        subtract_8_bit_from_accumulator_with_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x0F);
        assert_eq!(registers.flags, 0x60);
        
        //Carry
        registers.flags = 0x10;
        registers.accumulator = 0x01;
        system_data.mmu.mem_map[0x05] = 0x10;
        subtract_8_bit_from_accumulator_with_carry(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0xF0);
        assert_eq!(registers.flags, 0x50);
    }

    #[test]
    fn return_from_call_conditional_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xC0, 0xC8, 0xD0, 0xD8];
        let pass_flags: Vec<u8> = vec![0x00, 0x80, 0x00, 0x10];
        let fail_flags: Vec<u8> = vec![0x80, 0x00, 0x10, 0x00];
        let flags: Vec<Vec<u8>> = vec![pass_flags, fail_flags];
        let locations: Vec<u16> = vec![0x1234, 0x0001];
        system_data.mmu.mem_map[0x100] = 0x34;
        system_data.mmu.mem_map[0x101] = 0x12;

        for i in 0..opcodes.len()
        {
            for j in 0..flags.len()
            {
                registers.program_counter = 0x00;
                registers.stack_pointer = 0x100;
                registers.flags = flags[j][i];
                return_from_call_conditional(&mut system_data, &mut registers, opcodes[i]);
                assert_eq!(registers.program_counter, locations[j]);
            }
        }
    }

    #[test]
    fn shift_right_register_logical_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x3F, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D];

        //Zero flag test
        registers.accumulator = 0x00;
        shift_right_register_logical(&mut system_data, &mut registers, 0x3F);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0x80);

        for i in 0..opcodes.len()
        {
            registers.mapped_register_setter(i as u8, 0xFF);
            shift_right_register_logical(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.flags, 0x10);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x7F)
        }
    }

    #[test]
    fn shift_hl_location_right_logical_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);

        //Zero flag test
        system_data.mmu.mem_map[0x1234] = 0x00;
        shift_right_register_logical(&mut system_data, &mut registers, 0x3E);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x00);
        assert_eq!(registers.flags, 0x80);

        system_data.mmu.mem_map[0x1234] = 0xFF;
        shift_right_register_logical(&mut system_data, &mut registers, 0x3E);
        assert_eq!(registers.flags, 0x10);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x7F);
    }

    #[test]
    fn jump_address_with_conditional_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xC2, 0xCA, 0xD2, 0xDA];
        let pass_flags: Vec<u8> = vec![0x00, 0x80, 0x00, 0x10];
        let fail_flags: Vec<u8> = vec![0x80, 0x00, 0x10, 0x00];
        let flags: Vec<Vec<u8>> = vec![pass_flags, fail_flags];
        let locations: Vec<u16> = vec![0x1234, 0x0003];
        system_data.mmu.mem_map[0x0001] = 0x34;
        system_data.mmu.mem_map[0x0002] = 0x12;
        for i in 0..opcodes.len()
        {
            for j in 0..locations.len()
            {
                registers.program_counter = 0x00;
                registers.flags = flags[j][i];
                jump_address_with_conditional(&mut system_data, &mut registers, opcodes[i]);
                assert_eq!(registers.program_counter, locations[j]);
            }
        }
    }

    #[test]
    fn rotate_right_through_carry_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x1F, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D];
        //No Set
        registers.accumulator = 0x02;
        rotate_right_through_carry(&mut system_data, &mut registers, 0x1F);
        assert_eq!(registers.flags, 0x00);
        assert_eq!(registers.accumulator, 0x01);

        //Zero Flag
        registers.accumulator = 0x00;
        rotate_right_through_carry(&mut system_data, &mut registers, 0x1F);
        assert_eq!(registers.flags, 0x80);
        assert_eq!(registers.accumulator, 0x00);

        for i in 0..opcodes.len()
        {
            registers.flags = 0x10;
            registers.mapped_register_setter(i as u8, 0xFF);
            rotate_right_through_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.flags, 0x10);
            assert_eq!(registers.mapped_register_getter(i as u8), 0xFF);
        }
    }

    #[test]
    fn rotate_hl_location_right_through_carry_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);

        //No Set
        system_data.mmu.mem_map[0x1234] = 0x02;
        registers.flags = 0x00;
        rotate_right_through_carry(&mut system_data, &mut registers, 0x1E);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x01);
        assert_eq!(registers.flags, 0x00);

        //Zero Flag
        system_data.mmu.mem_map[0x1234] = 0x00;
        registers.flags = 0x00;
        rotate_right_through_carry(&mut system_data, &mut registers, 0x1E);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x00);
        assert_eq!(registers.flags, 0x80);

        //Carry
        system_data.mmu.mem_map[0x1234] = 0xFF;
        registers.flags = 0x10;
        rotate_right_through_carry(&mut system_data, &mut registers, 0x1E);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0xFF);
        assert_eq!(registers.flags, 0x10);
    }

    #[test]
    fn xor_accumulator_with_n_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        system_data.mmu.mem_map[0x01] = 0xFF;
        system_data.mmu.mem_map[0x03] = 0xFF;
        registers.accumulator = 0xFF;
        xor_accumulator_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x80);
        assert_eq!(registers.accumulator, 0x00);
        xor_accumulator_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x00);
        assert_eq!(registers.accumulator, 0xFF);
    }

    #[test]

    fn add_registers_to_accumulator_with_carry_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D];
        //No Set & Zero
        registers.flags = 0x00;
        registers.accumulator = 0x00;
        add_registers_to_accumulator_with_carry(&mut system_data, &mut registers, 0x8F);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0x80);

        //Set
        //Half Carry
        registers.flags = 0x10;
        registers.accumulator = 0x0F;
        add_registers_to_accumulator_with_carry(&mut system_data, &mut registers, 0x8F);
        assert_eq!(registers.accumulator, 0x1F);
        assert_eq!(registers.flags, 0x20);

        //Carry and register tests
        for i in 0..opcodes.len()
        {
            registers.flags = 0x10;
            registers.accumulator = 0xF0;
            registers.mapped_register_setter(i as u8 + 1, 0x10);
            add_registers_to_accumulator_with_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.accumulator, 0x01);
            assert_eq!(registers.flags, 0x10);
        } 
    }

    #[test]
    fn reset_bit_in_register_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let test_file_raw = File::open("src/cpu/test_csvs/reset_bit.csv").unwrap();
        let mut test_file = csv::ReaderBuilder::new().has_headers(false).from_reader(test_file_raw);
        let mut opcodes: Vec<String> = Vec::new();
        for record_raw in test_file.records()
        {
            let record = &(record_raw.unwrap());
            opcodes.push(record[0].to_string());
        }
        for i in 0..8
        {
            for j in 0..7
            {
                let opcode = Vec::from_hex(&opcodes[(i * 7) + j]).unwrap();
                registers.mapped_register_setter(j as u8, 0xFF);
                reset_bit_in_register(&mut system_data, &mut registers, opcode[0]);
                assert_eq!((registers.mapped_register_getter(j as u8) >> i) & 0x01, 0x00);
            }
        }
    }

    #[test]
    fn or_hl_location_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);
        system_data.mmu.mem_map[0x1234] = 0x00;
        //Zero-flag
        registers.accumulator = 0x00;
        or_hl_location(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x80);
        assert_eq!(registers.accumulator, 0x00);
        //Non-zero;
        registers.accumulator = 0x00;
        system_data.mmu.mem_map[0x1234] = 0xFF;
        or_hl_location(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x00);
        assert_eq!(registers.accumulator, 0xFF);

    }

    #[test]
    fn decrement_hl_location_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);
        //H flag
        system_data.mmu.mem_map[0x1234] = 0x10;
        decrement_hl_location(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x60);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x0F);
        //No flag
        decrement_hl_location(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x40);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x0E);
        //Z flag
        system_data.mmu.mem_map[0x1234] = 0x01;
        decrement_hl_location(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0xC0);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0x00);
    }

    #[test]
    fn load_de_location_with_accumulator_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(2, 0x1234);
        registers.accumulator = 0xFF;
        load_de_location_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0xFF);
    }

    #[test]
    fn load_hl_with_stack_pointer_plus_n_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //Half Carry
        registers.stack_pointer = 0x000F;
        system_data.mmu.mem_map[0x01] = 0x01;
        load_hl_with_stack_pointer_plus_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x20);
        assert_eq!(registers.mapped_16_bit_register_getter(3), 0x10);
        //Carry
        registers.stack_pointer = 0x00F0;
        system_data.mmu.mem_map[0x03] = 0x10;
        load_hl_with_stack_pointer_plus_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x10);
        assert_eq!(registers.mapped_16_bit_register_getter(3), 0x100);
        //Negative
        registers.stack_pointer = 0x0001;
        system_data.mmu.mem_map[0x05] = 0xFF;
        load_hl_with_stack_pointer_plus_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x30);
        assert_eq!(registers.mapped_16_bit_register_getter(3), 0x00);
    }

    #[test]
    fn compare_register_to_accumulator_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD];
        let accumulator_values: Vec<u8> = vec![0x10, 0x02, 0x10, 0x10];
        let register_values: Vec<u8> = vec![0x20, 0x01, 0x01, 0x10];
        let flag_values: Vec<u8> = vec![0x50, 0x40, 0x60, 0xC0];
        //Accumulator
        registers.accumulator = accumulator_values[0];
        compare_register_to_accumulator(&mut system_data, &mut registers, 0xBF);
        assert_eq!(registers.flags, flag_values[3]);

        //Registers
        for i in 0..opcodes.len()
        {
            //Carry -> No Carry -> Half Carry -> Zero
            for j in 0..flag_values.len()
            {
                registers.accumulator = accumulator_values[j];
                registers.mapped_register_setter(i as u8 + 1, register_values[j]);
                compare_register_to_accumulator(&mut system_data, &mut registers, opcodes[i]);
                assert_eq!(registers.flags, flag_values[j]);
            }
        }
    }

    #[test]
    fn load_hl_to_stack_pointer_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(3, 0x1234);
        load_hl_to_stack_pointer(&mut system_data, &mut registers);
        assert_eq!(registers.stack_pointer, 0x1234);
    }

    #[test]
    fn bcd_adjust_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        //N = 0
        //NO CH 0-9 | 0-9
        registers.accumulator = 0x99;
        registers.flags = 0x00;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x99);
        assert_eq!(registers.flags, 0x00);

        //NO CH 0-8 | A-F | +0x06
        registers.accumulator = 0x8A;
        registers.flags = 0x00;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x90);
        assert_eq!(registers.flags, 0x00);

        //NO C | H | 0-9 | 0-3 | +0x06
        registers.accumulator = 0x80;
        registers.flags = 0x20;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x86);
        assert_eq!(registers.flags, 0x00);

        //NO CH | A-F | 0-9 | +0x60 | Set C
        registers.accumulator = 0xA0;
        registers.flags = 0x00;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0x90);

        //NO CH | 9-F | A-F | +0x66 | Set C
        registers.accumulator = 0xAA;
        registers.flags = 0x00;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x10);
        assert_eq!(registers.flags, 0x10);

        //NO C | H | A-F | 0-3 | +0x66 | Set C
        registers.accumulator = 0xA0;
        registers.flags = 0x20;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x06);
        assert_eq!(registers.flags, 0x10);

        // C | NO H | 0-2 | 0-9 | +0x60 | Set C
        registers.accumulator = 0x00;
        registers.flags = 0x10;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x60);
        assert_eq!(registers.flags, 0x10);

        // C | NO H | 0-2 | A-F | +0x66 | Set C
        registers.accumulator = 0x0A;
        registers.flags = 0x10;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x70);
        assert_eq!(registers.flags, 0x10);

        // C |  H | 0-3 | 0-3 | +0x66 | Set C
        registers.accumulator = 0x00;
        registers.flags = 0x30;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x66);
        assert_eq!(registers.flags, 0x10);

        //N = 1
        // NO CH | 0-9 | 0-9 | +0x00
        registers.accumulator = 0x00;
        registers.flags = 0x40;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x00);
        assert_eq!(registers.flags, 0xC0);
                
        // No C |  H | 0-8 | 6-F | +0xFA
        registers.accumulator = 0x86;
        registers.flags = 0x60;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x80);
        assert_eq!(registers.flags, 0x40);

        // C |  NO H | 7-F | 0-9 | +0xA0 | Set C
        registers.accumulator = 0x70;
        registers.flags = 0x50;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x10);
        assert_eq!(registers.flags, 0x50);

        // C |  NO H | 7-F | 0-9 | +0xA0 | Set C
        registers.accumulator = 0x67;
        registers.flags = 0x70;
        bcd_adjust(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x01);
        assert_eq!(registers.flags, 0x50);
    }

    #[test]
    fn rlca_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.accumulator = 0x80;
        registers.flags = 0x00;
        rlca(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x01);
        assert_eq!(registers.flags, 0x10);
        rlca(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x02);
        assert_eq!(registers.flags, 0x00);
    }

    #[test]
    fn rrca_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.accumulator = 0x01;
        registers.flags = 0x00;
        rrca(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x80);
        assert_eq!(registers.flags, 0x10);
        rrca(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 0x40);
        assert_eq!(registers.flags, 0x00);
    }

    #[test]
    fn set_carry_flag_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.flags = 0xE0;
        set_carry_flag(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x90);
    }

    #[test]
    fn flip_carry_flag_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.flags = 0xE0;
        flip_carry_flag(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x90);
        flip_carry_flag(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x80);
    }

    #[test]
    fn rotate_register_left_carry_set_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x07, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        for i in 0..opcodes.len()
        {   
            registers.mapped_register_setter(i as u8, 0x80);
            rotate_register_left_carry_set(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x01);
            assert_eq!(registers.flags, 0x10);
            registers.mapped_register_setter(i as u8, 0x00);
            rotate_register_left_carry_set(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x00);
            assert_eq!(registers.flags, 0x80);
        }
    }

    #[test]
    fn rotate_register_right_carry_set_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x0F, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D];
        for i in 0..opcodes.len()
        {   
            registers.mapped_register_setter(i as u8, 0x01);
            rotate_register_right_carry_set(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x80);
            assert_eq!(registers.flags, 0x10);
            registers.mapped_register_setter(i as u8, 0x00);
            rotate_register_right_carry_set(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x00);
            assert_eq!(registers.flags, 0x80);
        }
    }

    #[test]
    fn shift_left_load_carry_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x27, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25];
        for i in 0..opcodes.len()
        {   
            registers.mapped_register_setter(i as u8, 0x80);
            shift_left_load_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x00);
            assert_eq!(registers.flags, 0x90);
            registers.mapped_register_setter(i as u8, 0x00);
            shift_left_load_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x00);
            assert_eq!(registers.flags, 0x80);
        }
    }

    #[test]
    fn shift_right_load_carry_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let opcodes: Vec<u8> = vec![0x2F, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D];
        for i in 0..opcodes.len()
        {   
            registers.mapped_register_setter(i as u8, 0x01);
            shift_right_load_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x00);
            assert_eq!(registers.flags, 0x90);
            registers.mapped_register_setter(i as u8, 0x00);
            shift_right_load_carry(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(registers.mapped_register_getter(i as u8), 0x00);
            assert_eq!(registers.flags, 0x80);
        }
    }

    #[test]
    fn load_stack_pointer_to_nn_address_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.stack_pointer = 0xFFEE;
        system_data.mmu.mem_map[0x0001] = 0x34;
        system_data.mmu.mem_map[0x0002] = 0x12;
        load_stack_pointer_to_nn_address(&mut system_data, &mut registers);
        assert_eq!(system_data.mmu.mem_map[0x1234], 0xEE);
        assert_eq!(system_data.mmu.mem_map[0x1235], 0xFF);
    }

    #[test]
    fn load_accumulator_to_address_at_bc_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        registers.mapped_16_bit_register_setter(1, 0x1234);
        registers.accumulator = 0xFF;
        load_accumulator_to_address_at_bc(&mut system_data, &mut registers);       
        assert_eq!(system_data.mmu.mem_map[0x1234], 0xFF);
    }

    #[test]
    fn add_signed_8_bit_to_stack_pointer_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = Registers::new();
        let stack_pointer_init_values: Vec<u16> = vec![0x0000, 0x0001, 0x000F, 0x00F0];
        let stack_pointer_result_values: Vec<u16> = vec![0x0001, 0x0000, 0x0010, 0x0100];
        let add_values: Vec<u8> = vec![0x01, 0xFF, 0x01, 0x10];
        let flag_values: Vec<u8> = vec![0x00, 0x30, 0x20, 0x10];

        for i in 0..add_values.len()
        {
            println!("@@@@{}", i);
            registers.program_counter = 0;
            system_data.mmu.mem_map[0x0001] = add_values[i];
            registers.stack_pointer = stack_pointer_init_values[i];
            add_signed_8_bit_to_stack_pointer(&mut system_data, &mut registers);
            assert_eq!(registers.stack_pointer, stack_pointer_result_values[i]);
            assert_eq!(registers.flags, flag_values[i]);
        }
    }
}