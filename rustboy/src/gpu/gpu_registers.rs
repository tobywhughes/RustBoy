use system::SystemData;
use image::{RgbaImage, Rgba};

pub struct GPU_Registers
{
    pub lcd_position: LCD_Position,
    pub v_blank: bool,
    pub v_blank_draw_flag: bool,
    pub lcdc_register: LCDC_Register,
    pub lcdc_status: LCDC_Status,
    pub shade_profile: ShadeProfile,
}

impl GPU_Registers
{
    pub fn new() -> GPU_Registers
    {
        return GPU_Registers
        {
            lcd_position: LCD_Position::new(),
            v_blank: false,
            v_blank_draw_flag: false,
            lcdc_register: LCDC_Register::new(),
            lcdc_status: LCDC_Status::new(),
            shade_profile: ShadeProfile::new(0),
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
    pub scroll_x_buffer: Vec<u8>,
    pub scroll_y_buffer: Vec<u8>,
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
            scroll_x_buffer: vec![0; 144],
            scroll_y_buffer: vec![0; 144],
        }
    }

    pub fn update(&mut self, system_data: &mut SystemData, interrupt_enabled: bool)
    {
        self.scroll_x = system_data.mmu.mem_map[0xFF43];
        self.scroll_y = system_data.mmu.mem_map[0xFF42];
        self.ly_compare = system_data.mmu.mem_map[0xFF45];
        self.window_x = system_data.mmu.mem_map[0xFF4B];
        self.window_y =system_data.mmu.mem_map[0xFF4A];
        if self.ly_compare == self.ly_register.value
        {
            system_data.mmu.mem_map[0xFF41] |= 0x04;
            if interrupt_enabled
            {
                system_data.mmu.mem_map[0xFF0F] |= 0x02;
            }
        }
        else 
        {
            system_data.mmu.mem_map[0xFF41] &= 0xFB;
            if interrupt_enabled
            {
                system_data.mmu.mem_map[0xFF0F] &= 0xFD;
            }
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
            system_data.mmu.mem_map[0xFF44] = self.value;
            return true;
        }
        system_data.mmu.mem_map[0xFF44] = self.value;
        return false;
    }

    pub fn reset(&mut self, system_data: &mut SystemData)
    {
        self.value == 0;
        system_data.mmu.mem_map[0xFF44] = self.value;
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
        self.value = system_data.mmu.mem_map[0xFF40];
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
        self.value = system_data.mmu.mem_map[0xFF41];
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

pub struct ShadeProfile
{
    pub shade_0: Rgba<u8>,
    pub shade_1: Rgba<u8>,
    pub shade_2: Rgba<u8>,
    pub shade_3: Rgba<u8>,
    pub default: Rgba<u8>,
}

impl ShadeProfile
{
    pub fn new(profile: u8) -> ShadeProfile
    {
        match profile
        {
            0 => return ShadeProfile
            {
                shade_0: Rgba([130,160,15, 0xFF]),
                shade_1: Rgba([80,125,15, 0xFF]),
                shade_2: Rgba([40,90,40, 0xFF]),
                shade_3: Rgba([13, 50, 13, 0xFF]),
                default: Rgba([0, 0, 0, 0xFF]),
            },
            1 => return ShadeProfile
            {
                shade_0: Rgba([255,0,0, 0xFF]),
                shade_1: Rgba([0,255,0, 0xFF]),
                shade_2: Rgba([0,0, 255, 0xFF]),
                shade_3: Rgba([255, 255, 255, 0xFF]),
                default: Rgba([0, 0, 0, 0xFF]),
            },
            2 => return ShadeProfile
            {
                shade_0: Rgba([230,10,10, 0xFF]),
                shade_1: Rgba([250,243,0, 0xFF]),
                shade_2: Rgba([50,0, 0, 0xFF]),
                shade_3: Rgba([0, 0, 0, 0xFF]),
                default: Rgba([0, 0, 0, 0xFF]),
            },
            _ => return ShadeProfile
            {
                shade_0: Rgba([0, 0, 0, 0xFF]),
                shade_1: Rgba([0, 0, 0, 0xFF]),
                shade_2: Rgba([0, 0, 0, 0xFF]),
                shade_3: Rgba([0, 0, 0, 0xFF]),
                default: Rgba([0, 0, 0, 0xFF]),
            },
        }
    }
}