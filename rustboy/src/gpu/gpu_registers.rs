use system::SystemData;

pub struct GPU_Registers
{
    pub ly_register: LY_Register,
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
            ly_register: LY_Register::new(),
            v_blank: false,
            v_blank_draw_flag: false,
            lcdc_register: LCDC_Register::new(),
        }
    }
}

pub struct LCD_Position
{
    pub ly_register: LY_Register,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub ly_compare: u8,
    pub window_x: u8,
    pub window_y: u8,
}

impl LCD_Position
{
    pub fn new() -> LCD_Position
    {
        return LCD_Position
        {
            ly_register: LY_Register::new(),
            scroll_x: 0,
            scroll_y: 0,
            ly_compare: 0,
            window_x: 0,
            window_y: 0,
        }
    }
}

pub struct LY_Register
{
    pub value: u8,
    pub cycle_count: u32,
    pub sub_cycle_count: u16,   
}

impl LY_Register
{
    pub fn new()-> LY_Register
    {
        return LY_Register
        {
            value: 0,
            cycle_count: 0,
            sub_cycle_count: 0,
        }
    }

    pub fn tick(&mut self, system_data: &mut SystemData) -> bool
    {
        self.value += 1;
        if self.value == 154
        {
            self.value = 0;
            system_data.mem_map[0xFF44] = self.value;
            return true;
        }
        system_data.mem_map[0xFF44] = self.value;
        return false;
    }

    pub fn reset(&mut self, system_data: &mut SystemData)
    {
        self.value == 0;
        system_data.mem_map[0xFF44] = self.value;
    }

    pub fn add_cycles(&mut self, system_data: &SystemData) -> bool
    {
        self.cycle_count += system_data.cycles as u32;
        if (self.cycle_count >= 70224)
        {
            self.cycle_count -= 70224;
            return true;        
        } 
        return false;
    }

    pub fn add_sub_cycles(&mut self, system_data: &SystemData) -> bool
    {
        self.sub_cycle_count += system_data.cycles as u16;
        if self.sub_cycle_count >= 456
        {
            self.sub_cycle_count-= 456;
            return true;
        }
        return false;
    }
}

pub struct LCDC_Register
{
    pub value: u8,
    pub display_enable: bool,
    pub window_display_select: bool,
    pub window_enable: bool,
    pub tile_data: bool,
    pub background_display_select: bool,
    pub sprite_size: bool,
    pub sprite_enable: bool,
    pub background_enable: bool,
}

impl LCDC_Register
{
    pub fn new() -> LCDC_Register
    {
        return LCDC_Register
        {
            value: 0,
            display_enable: false,
            window_display_select: false,
            window_enable: false,
            tile_data: false,
            background_display_select: false,
            sprite_size: false,
            sprite_enable: false,
            background_enable: false,
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
            if (self.value >> i) & 0b00000001 == 1
            {
                *states[i] = true;
            }
            else {
                *states[i] = false;
            }
        }
    }
}

pub struct LCDC_Status
{
    pub value: u8,
    pub lyc_ly_coincidence_interrupt: bool,
    pub mode_2_oam_interrupt: bool,
    pub mode_1_v_blank_interrupt: bool,
    pub mode_0_h_blank_interrupt: bool,
    pub coincidence_flag: bool,
    pub mode_flag: u8,
}

impl LCDC_Status
{
    pub fn new() -> LCDC_Status
    {
        return LCDC_Status
        {
            value: 0,
            lyc_ly_coincidence_interrupt: false,
            mode_2_oam_interrupt: false,
            mode_1_v_blank_interrupt: false,
            mode_0_h_blank_interrupt: false,
            coincidence_flag: false,
            mode_flag: 2,
        }
    }
        
    pub fn update_lcdc_status(&mut self, system_data: &SystemData)
    {
        self.value = system_data.mem_map[0xFF41];
        self.map_bit_states();
    }

    fn map_bit_states(&mut self)
    {
        let mut states = vec![&mut self.lyc_ly_coincidence_interrupt, &mut self.mode_2_oam_interrupt, 
                              &mut self.mode_1_v_blank_interrupt,  &mut self.mode_0_h_blank_interrupt,
                              &mut self.coincidence_flag];
        states.reverse();
        for i in 0..5
        {
            if (self.value >> i + 2) & 0b00000001 == 1
            {
                *states[i] = true;
            }
            else {
                *states[i] = false;
            }
        }

        self.mode_flag = self.value &0b00000011;
    }
}

#[cfg(test)]
mod main_tests
{

    use system::{get_system_data, SystemData};
    use gpu::gpu_registers::{LCDC_Register, LCDC_Status, LY_Register};

    #[test]
    fn lcdc_register_test() {
        let mut system_data : SystemData = get_system_data("CLASSIC");
        let mut lcdc_register : LCDC_Register = LCDC_Register::new();
        let bools: Vec<bool> = vec![false, true];
        let values = vec![0x00, 0xFF];
        for i in 0..values.len()
        {
            system_data.mem_map[0xFF40] = values[i];
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
            system_data.mem_map[0xFF41] = values[i];
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
            system_data.mem_map[0xFF41] = i;
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
                assert_eq!(system_data.mem_map[0xFF44], i + 1);
            }
            else
            {
                assert_eq!(reset_flag, true);
                assert_eq!(ly_register.value, 0);
                assert_eq!(system_data.mem_map[0xFF44], 0);
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
}