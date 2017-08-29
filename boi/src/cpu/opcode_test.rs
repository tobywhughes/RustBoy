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
        let mut registers : Registers = init_registers();
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
            assert_eq!(registers.flags, 0x80);
        }
    }


    #[test]
    fn increment_16_bit_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
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
        let mut registers : Registers = init_registers();
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
    fn load_increment_hl_register_location_with_accumulator_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.h_register = 0xFF;
        registers.l_register = 0xFE;
        registers.accumulator = 1;
        load_increment_hl_register_location_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(system_data.mem_map[0xFFFE], 1);
        assert_eq!(registers.h_register, 0xFF);
        assert_eq!(registers.l_register, 0xFF);
    }

    //TODO: Write a better test for this
    #[test]
    fn bit_check_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
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
        let mut registers : Registers = init_registers();
        registers.d_register = 0xFF;
        registers.e_register = 0xEE;
        system_data.mem_map[0xFFEE] = 1;
        load_accumulator_with_de_address(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 1);
    }

    #[test]
    fn call_nn_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.program_counter = 1;
        system_data.mem_map[2] = 0xCD;
        system_data.mem_map[3] = 0xAB;
        registers.stack_pointer = 0xFFFE;
        call_nn(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0xABCD);
        assert_eq!(registers.stack_pointer, 0xFFFC);
        assert_eq!(system_data.mem_map[registers.stack_pointer as usize], 4);
    }

    #[test]
    fn return_from_calltest() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        system_data.mem_map[0x2000] = 0xFF;
        system_data.mem_map[0x2001] = 0xEE;
        registers.stack_pointer = 0x2000;
        return_from_call(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0xEEFF);
        assert_eq!(registers.stack_pointer, 0x2002);
    }

    #[test]
    fn push_16_bit_register_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        let opcodes: Vec<u8> = vec![0xF5, 0xC5, 0xD5, 0xE5];
        registers.stack_pointer = 0xFFFE;
        for i in 0..8
        {
            registers.mapped_register_setter_with_flags(i, i);
        }
        for i in 0..4
        {
            push_16_bit_register(&mut system_data, &mut registers, opcodes[i]);
            assert_eq!(system_data.mem_map[registers.stack_pointer as usize + 1], i as u8 * 2);
            assert_eq!(system_data.mem_map[registers.stack_pointer as usize], (i as u8 * 2) + 1);
        }
    }

    #[test]
    fn pop_16_bit_register_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        let opcodes: Vec<u8> = vec![0xF1, 0xC1, 0xD1, 0xE1];
        registers.stack_pointer = 0xFFF0;
        for i in 0..8
        {
            system_data.mem_map[registers.stack_pointer as usize + i] = i as u8; 
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
        let mut registers : Registers = init_registers();
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
        let mut registers : Registers = init_registers();
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
        let mut registers : Registers = init_registers();
        registers.accumulator = 0xF1;
        //Normal
        system_data.mem_map[1] = 0x01;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x40);
        //Carry
        system_data.mem_map[3] = 0x0F2;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x70);
        //Half Carry
        system_data.mem_map[5] = 0x02;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x60);
        //Zero
        system_data.mem_map[7] = 0xF1;
        compare_with_n(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0xC0);
    }

    #[test]
    fn load_nn_with_accumulator_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.accumulator = 1;
        system_data.mem_map[1] = 0xEE;
        system_data.mem_map[2] = 0xFF;
        load_nn_with_accumulator(&mut system_data, &mut registers);
        assert_eq!(system_data.mem_map[0xFFEE], 1);
    }
    #[test]
    fn load_accumulator_with_io_port_with_n_offset_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        system_data.mem_map[0xFF0F] = 1;
        system_data.mem_map[1] = 0x0F;
        load_accumulator_with_io_port_with_n_offset(&mut system_data, &mut registers);
        assert_eq!(registers.accumulator, 1);
    }

    #[test]
    fn jump_displacement_on_zero_flag_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        registers.program_counter = 0x100;
        //Forward
        registers.flags = 0xFF;
        system_data.mem_map[0x101] = 0x02;
        jump_displacement_on_zero_flag(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0x104);
        registers.program_counter = 0x100;
        //Backward
        registers.flags = 0xFF;
        system_data.mem_map[0x101] = 0xF0;
        jump_displacement_on_zero_flag(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0xF2);
        registers.program_counter = 0x100;
        //Nonzero 
        registers.flags = 0x00;
        system_data.mem_map[0x101] = 0xFF;
        jump_displacement_on_zero_flag(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0x102);
    }

    #[test]
    fn jump_displacement_test() 
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
        system_data.mem_map[1] = 0xE;
        jump_displacement(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0x10);
        system_data.mem_map[0x11] = 0xFA;
        jump_displacement(&mut system_data, &mut registers);
        assert_eq!(registers.program_counter, 0xC);
        
    }

    #[test]
    fn subract_8_bit_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
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
        let mut registers : Registers = init_registers();
        registers.accumulator = 0x19;
        registers.h_register = 0x12;
        registers.l_register = 0x34;
        system_data.mem_map[0x1234] = 0x04;
        //Normal
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x40);
        //Carry
        system_data.mem_map[0x1234] = 0xFF;
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x70);
        //Half Carry
        system_data.mem_map[0x1234] = 0x0F;
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0x60);
        //Zero
        system_data.mem_map[0x1234] = 0x19;
        compare_with_hl_address(&mut system_data, &mut registers);
        assert_eq!(registers.flags, 0xC0);
    }

    #[test]
    fn add_8_bit_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut registers : Registers = init_registers();
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
        system_data.mem_map[0xFFEE] = 1;
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
}