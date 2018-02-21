use system::SystemData;

pub struct GPU_Registers
{
    pub ly_register: u8,
    pub ly_cycle_count: u32,
    pub ly_sub_cycle_count: u16,
    pub v_blank: bool,
    pub v_blank_draw_flag: bool,
    pub lcdc_register: LCDC_Register,
}

impl GPU_Registers
{
    pub fn new() -> GPU_Registers
    {
        return GPU_Registers
        {
            ly_register: 0,
            ly_cycle_count: 0,
            ly_sub_cycle_count: 0,
            v_blank: false,
            v_blank_draw_flag: false,
            lcdc_register: LCDC_Register::new(),
        }
    }
}


pub struct LCDC_Register
{
    pub value: u8,
    pub display_enable: u8,
    pub window_display_select: u8,
    pub window_enable: u8,
    pub tile_data: u8,
    pub background_display_select: u8,
    pub sprite_size: u8,
    pub sprite_enable: u8,
    pub background_enable: u8,
}

impl LCDC_Register
{
    pub fn new() -> LCDC_Register
    {
        return LCDC_Register
        {
            value: 0,
            display_enable: 0,
            window_display_select: 0,
            window_enable: 0,
            tile_data: 0,
            background_display_select: 0,
            sprite_size: 0,
            sprite_enable: 0,
            background_enable: 0,
        }
    }

    pub fn update_lcdc_register(&mut self, system_data: &SystemData)
    {
        self.value = system_data.mem_map[0xFF40];
        self.map_bit_states();
    }

    fn map_bit_states(&mut self)
    {
        let mut states = vec![&mut self.display_enable, &mut self.window_display_select, 
                              &mut self.window_enable,  &mut self.tile_data,
                              &mut self.background_display_select, &mut self.sprite_size,
                              &mut self.sprite_enable, &mut self.background_enable];
        states.reverse();
        for i in 0..8
        {
            *states[i] = (self.value >> i) & 0b00000001;
        }
    }
}

#[cfg(test)]
mod main_tests
{

    use system::{get_system_data, SystemData};
    use gpu::gpu_registers::LCDC_Register;

    #[test]
    fn lcdc_register_test() {
        let mut system_data : SystemData = get_system_data("CLASSIC");
        let mut lcdc_register : LCDC_Register = LCDC_Register::new();
        let values = vec![0x00, 0xFF];
        for i in 0..values.len()
        {
            system_data.mem_map[0xFF40] = values[i];
            lcdc_register.update_lcdc_register(&system_data);
            assert_eq!(lcdc_register.value , values[i]);
            assert_eq!(lcdc_register.display_enable, i as u8);
            assert_eq!(lcdc_register.window_display_select, i as u8);
            assert_eq!(lcdc_register.window_enable, i as u8);
            assert_eq!(lcdc_register.tile_data, i as u8);
            assert_eq!(lcdc_register.background_display_select, i as u8);
            assert_eq!(lcdc_register.sprite_size, i as u8);
            assert_eq!(lcdc_register.sprite_enable, i as u8);
            assert_eq!(lcdc_register.background_enable, i as u8);
        }
    }
}