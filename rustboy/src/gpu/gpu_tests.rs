#[cfg(test)]
mod gpu_register_tests
{

    use system::{get_system_data, SystemData};
    use gpu::gpu_registers::{LCDC_Register, LCDC_Status, LY_Register, LCD_Position};

    #[test]
    fn lcdc_register_test() {
        let mut system_data : SystemData = get_system_data("CLASSIC");
        let mut lcdc_register : LCDC_Register = LCDC_Register::new();
        let bools: Vec<bool> = vec![false, true];
        let values = vec![0x00, 0xFF];
        for i in 0..values.len()
        {
            system_data.mmu.mem_map[0xFF40] = values[i];
            lcdc_register.update_lcdc_register(&system_data);
            assert_eq!(lcdc_register.value , values[i]);
            let mut states = vec![lcdc_register.display_enable, lcdc_register.window_display_select, 
                                  lcdc_register.window_enable,  lcdc_register.tile_data,
                                  lcdc_register.background_display_select, lcdc_register.sprite_size,
                                  lcdc_register.sprite_enable, lcdc_register.background_enable];
            for state_index in 0..states.len()
            {
                assert_eq!(states[state_index], bools[i]);
            }
        }
    }

    #[test]
    fn lcdc_status_test() {
        let mut system_data : SystemData = get_system_data("CLASSIC");
        let mut lcdc_status : LCDC_Status = LCDC_Status::new();
        let bools: Vec<bool> = vec![false, true];
        let values = vec![0x00, 0xFF];
        for i in 0..values.len()
        {
            system_data.mmu.mem_map[0xFF41] = values[i];
            lcdc_status.update_lcdc_status(&system_data);
            assert_eq!(lcdc_status.value , values[i]);
            let mut states = vec![lcdc_status.lyc_ly_coincidence_interrupt, lcdc_status.mode_2_oam_interrupt, 
                                  lcdc_status.mode_1_v_blank_interrupt,  lcdc_status.mode_0_h_blank_interrupt,
                                  lcdc_status.coincidence_flag];
            for state_index in 0..states.len()
            {
                assert_eq!(states[state_index], bools[i]);
            }
        }

        for i in 0..4
        {
            system_data.mmu.mem_map[0xFF41] = i;
            lcdc_status.update_lcdc_status(&system_data);
            assert_eq!(lcdc_status.mode_flag, i);
        }
    }
    
    #[test]
    fn ly_register_test()
    {
        let mut system_data : SystemData = get_system_data("CLASSIC");
        let mut ly_register: LY_Register = LY_Register::new();
        let mut reset_flag = false;

        for i in 0..154
        {
            reset_flag = ly_register.tick(&mut system_data);
            if i < 153
            {
                assert_eq!(reset_flag, false);
                assert_eq!(ly_register.value, i + 1);
                assert_eq!(system_data.mmu.mem_map[0xFF44], i + 1);
            }
            else
            {
                assert_eq!(reset_flag, true);
                assert_eq!(ly_register.value, 0);
                assert_eq!(system_data.mmu.mem_map[0xFF44], 0);
            }
        }

        system_data.cycles = 100;
        for i in 0..703
        {
            reset_flag = ly_register.add_cycles(&system_data);
            if i < 702
            {
                assert_eq!(reset_flag, false);
                assert_eq!(ly_register.cycle_count , (i + 1) * 100);
            }
            else
            {
                assert_eq!(reset_flag, true);
                assert_eq!((ly_register.cycle_count < 100), true);
            }
        }

        for i in 0..4
        {
            reset_flag = ly_register.add_sub_cycles(&system_data);
            if i < 456
            {
                assert_eq!(reset_flag, false);
                assert_eq!(ly_register.sub_cycle_count , (i + 1) * 100);
            }
            else
            {
                assert_eq!(reset_flag, true);
                assert_eq!((ly_register.sub_cycle_count < 100), true);
            }
        }       
    }

    #[test]
    fn lcd_position_test() {
        let mut system_data : SystemData = get_system_data("CLASSIC");
        let mut lcd_position: LCD_Position = LCD_Position::new();
        let mut lcdc_status: LCDC_Status = LCDC_Status::new();
        let mem_addrs: Vec<usize> = vec![0xFF42, 0xFF43, 0xFF45, 0xFF4A, 0xFF4B];
        for value in 0..2
        {
            for addr_index in 0..5
            {
                system_data.mmu.mem_map[mem_addrs[addr_index as usize]] = value as u8;
            }
            lcd_position.update(&mut system_data);
            let registers: Vec<u8> = vec![lcd_position.scroll_x, lcd_position.scroll_y,
                                          lcd_position.window_x, lcd_position.window_y,
                                          lcd_position.ly_compare];
            
            for register in 0..5
            {
                assert_eq!(registers[register as usize], value);
            }
            
            lcdc_status.update_lcdc_status(&system_data);
            assert_eq!(lcdc_status.coincidence_flag, (value != 1));
        }
    }
}