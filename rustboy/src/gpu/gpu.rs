use system::*;
use gpu::gpu_registers::{GPU_Registers, ShadeProfile, LCD_Position};
use image::ImageBuffer;
use image::{RgbaImage, Rgba};

pub struct TileMap
{
    pub tiles: Vec<TileData>,
    pub map: Vec<u8>,
}

impl TileMap {
     pub fn new() -> TileMap
     {
         return TileMap
         {
             tiles: vec![TileData::new();256],
             map: vec![0, 1024],
         }
     }

     pub fn populate_tile_map(&mut self, system_data_original: &mut SystemData, lcdc_data_select: bool, lcdc_display_select: bool)
     {
        let mut system_data = system_data_original;
        for tile_index in 0..self.tiles.len()
        {
            self.tiles[tile_index] = get_tile_data(tile_index as u8, &mut system_data, lcdc_data_select);
        }
        
        self.map = self.vectorize_map(&mut system_data, lcdc_display_select);
     }

     fn vectorize_map(&mut self, system_data: &mut SystemData, lcdc_display_select: bool) -> Vec<u8>
     {
        let mut map: Vec<u8> = vec![0;1024];
        let mut map_offset: u16 = 0;
        if lcdc_display_select
        {
            map_offset = 0x9C00;
        }
        else {
            map_offset = 0x9800;
        }

        for i in 0..map.len()
        {
            map[i] = system_data.mmu.mem_map[map_offset as usize + i];
        }

        return map
     }
}

#[derive(Clone)]
pub struct SpriteAttribute
{
    pub y_position: u8,
    pub x_position: u8,
    pub tile_number: u8,
    pub flags: u8,
}

impl SpriteAttribute {
    pub fn new() -> SpriteAttribute
    {
        return SpriteAttribute
        {
            y_position: 0,
            x_position: 0,
            tile_number: 0,
            flags: 0,
        }
    }

    pub fn update_sprite(&mut self, y_position: u8, x_position: u8, tile_number: u8, flags: u8)
    {
        self.y_position = y_position;
        self.x_position = x_position;
        self.tile_number = tile_number;
        self.flags = flags;
    }
}

pub struct OAM_Table
{
    pub table: Vec<SpriteAttribute>,
}

impl OAM_Table {
    pub fn new() -> OAM_Table
    {
        return OAM_Table
        {
            table: vec![SpriteAttribute::new();40],
        }
    }

    pub fn populate_oam_table(&mut self, system_data: &SystemData)
    {
        for i in 0..40
        {
            let mut sprite_attribute = SpriteAttribute::new();
            let y_position = system_data.mmu.mem_map[0xFE00 + (i * 4)];;
            let x_position = system_data.mmu.mem_map[0xFE01 + (i * 4)];
            let tile_number = system_data.mmu.mem_map[0xFE02 + (i * 4)];
            let flags = system_data.mmu.mem_map[0xFE03 + (i * 4)];
            sprite_attribute.update_sprite(y_position, x_position, tile_number, flags);
            self.table[i] = sprite_attribute;
        }
    }
}

#[derive(Clone)]
pub struct TileData
{
    pub data: Vec<u8>,
}

impl TileData
{
    pub fn new() -> TileData
    {
        return TileData
        {
            data: vec![0; 64],
        }
    }

}


pub fn get_tile_data(tile_index: u8, system_data: &mut SystemData, lcdc_data_select: bool) -> TileData
{
    let mut tile_data = TileData::new();
    let mut vram_offset: u16 = 0;

    if lcdc_data_select
    {
        vram_offset = 0x8000;
    }
    else {
        vram_offset = 0x8800
    }

    let mem_loc: u16 = vram_offset + (tile_index as u16 * 16);
    for byte_pair in 0..8
    {
        for bit in 0..8
        {
            let upper_bit: u8 = (system_data.mmu.mem_map[(mem_loc + (2 * byte_pair as u16)) as usize + 1] >> (7 - bit)) & 0x01;
            let lower_bit: u8 = (system_data.mmu.mem_map[(mem_loc + (2 * byte_pair as u16)) as usize] >> (7 - bit)) & 0x01;
            tile_data.data[(byte_pair as usize * 8) + bit as usize] = (upper_bit << 1) | lower_bit;
        }
    }
    return tile_data;
}

pub fn update_gpu(system_data_original: &mut SystemData, registers_original: &mut Registers, gpu_registers_original: &mut GPU_Registers)
{
    let mut system_data = system_data_original;
    let mut registers = registers_original;
    let mut gpu_registers = gpu_registers_original;
    LCD_Y_Coordinate_Update(&mut system_data, &mut gpu_registers);
    gpu_registers.lcdc_register.update_lcdc_register(&system_data);
    gpu_registers.lcdc_status.update_lcdc_status(&system_data);
    gpu_registers.lcd_position.update(&mut system_data, gpu_registers.lcdc_status.lyc_ly_coincidence_interrupt);
}

pub fn LCD_Y_Coordinate_Update(system_data_original: &mut SystemData, gpu_registers: &mut GPU_Registers)
{
    let mut system_data = system_data_original;
    gpu_registers.lcd_position.ly_register.add_cycles(&system_data);
    let tick_flag = gpu_registers.lcd_position.ly_register.add_sub_cycles(&system_data);
    if tick_flag
    {
        let reset_flag = gpu_registers.lcd_position.ly_register.tick(&mut system_data);
        let ly_value = gpu_registers.lcd_position.ly_register.value;
        if ly_value == 144
        {
            gpu_registers.v_blank = true;
            system_data.mmu.mem_map[0xFF0F] |= 0x01;
            gpu_registers.v_blank_draw_flag = true;
        }
        else if ly_value < 144
        {
            gpu_registers.lcd_position.scroll_x_buffer[ly_value as usize] = gpu_registers.lcd_position.scroll_x;
            gpu_registers.lcd_position.scroll_y_buffer[ly_value as usize] = gpu_registers.lcd_position.scroll_y;
        }
        if reset_flag
        {
            gpu_registers.v_blank = false;
            system_data.mmu.mem_map[0xFF0F] &= 0xFE;
        }
    }
}

pub fn create_background_img(background_tile_map: &TileMap, window_tile_map: &TileMap, gpu_registers: &GPU_Registers, system_data: &SystemData, oam_table: &OAM_Table, oam_tile_map: &TileMap) -> RgbaImage
{
    //Technically switchable by the row, will implement later
    let palette_data = system_data.mmu.mem_map[0xFF47];
    let object_palette_0 = system_data.mmu.mem_map[0xFF48];
    let object_palette_1 = system_data.mmu.mem_map[0xFF49];
    let mut image_buffer = ImageBuffer::new(160, 144);
    let background_buffer = build_bitmap(background_tile_map, gpu_registers.lcdc_register.tile_data, palette_data, gpu_registers.lcdc_register.display_enable);
    let mut scrolled_buffer = scroll_background_bitmap(background_buffer, &gpu_registers.lcd_position);
    if gpu_registers.lcdc_register.window_enable
    {
        let window_buffer = build_bitmap(window_tile_map, gpu_registers.lcdc_register.tile_data, palette_data, gpu_registers.lcdc_register.window_enable);
        scrolled_buffer = place_window(scrolled_buffer, window_buffer, &gpu_registers.lcd_position)
    }
    if gpu_registers.lcdc_register.sprite_enable
    {
        scrolled_buffer = apply_oam_table_to_bitmap(&oam_table, scrolled_buffer, object_palette_0, object_palette_1, gpu_registers.lcdc_register.sprite_size, &oam_tile_map);
    }
    for row_y in 0..144
    {
        for row_x in 0..160
        {
           let pixel_shade = scrolled_buffer[(row_y * 160) + row_x];
           let pixel = pixel_color_map(pixel_shade, &gpu_registers.shade_profile);
           image_buffer.put_pixel(row_x as u32, row_y as u32, pixel);
        }
    }
   return image_buffer;
}

fn scroll_background_bitmap(buffer: Vec<u8>, scroll: &LCD_Position) -> Vec<u8>
{
    let mut bitmap = vec![0; 0x5A00];
    for row_y in 0..144
    {
        for row_x in 0..160
        {
           let row_x_scrolled = (row_x + scroll.scroll_x_buffer[row_y] as usize) % 256;
           let row_y_scrolled = (row_y + scroll.scroll_y_buffer[row_y] as usize) % 256;
           bitmap[(row_y * 160) + row_x] = buffer[(row_y_scrolled * 256) + row_x_scrolled];
        }
    }

    return bitmap;
}

fn place_window(scrolled_bitmap: Vec<u8>, window_bitmap: Vec<u8>, scroll: &LCD_Position) -> Vec<u8>
{
    let mut bitmap = scrolled_bitmap;
    for row_y in 0..144
    {
        for row_x in 0..160
        {
            let scrolled_y = (row_y + scroll.window_y as usize);
            let mut possible_negative = (row_x + scroll.window_x as usize - 7);
            if possible_negative < 0
            {
                possible_negative = 0xFF;
            }
            let scrolled_x = possible_negative as usize;
            if scrolled_y < 144 && scrolled_x < 160
            {
                bitmap[(scrolled_y * 160) + scrolled_x] = window_bitmap[(row_y as usize * 160) + row_x as usize];
            }
        }
    }
    return bitmap;
}

fn build_bitmap(background_tile_map: &TileMap, tile_data_select: bool, palette_data:u8, enable: bool) -> Vec<u8>
{

    let mut buffer = vec![0; 0x10000];
    for tile_y in 0..32
    {
        for tile_x in 0..32
        {
            for pixel_y in 0..8
            {
                for pixel_x in 0..8
                {
                    let mut tile = background_tile_map.map[(tile_y * 32) + tile_x];
                    if !tile_data_select
                    {
                        let tile_temp = tile as i8 as i16;
                        tile = (tile_temp + 0x80) as u8;
                    }
                    let mut pixel_data = background_tile_map.tiles[tile as usize].data[(pixel_y * 8) + pixel_x];
                    pixel_data = pixel_shade_map(pixel_data, palette_data);
                    if enable{
                        buffer[(256 * ((tile_y * 8) + pixel_y)) + ((tile_x * 8) + pixel_x)] = pixel_data;
                    }
                    else 
                    {
                        buffer[(256 * ((tile_y * 8) + pixel_y)) + ((tile_x * 8) + pixel_x)] = 0;
                    }
                }
            }
        }
    }
    return buffer;
}

fn apply_oam_table_to_bitmap(oam_table: &OAM_Table, scrolled_bitmap: Vec<u8>, palette_0: u8, palette_1: u8, large_sprites: bool, tile_map: &TileMap) -> Vec<u8>
{
    let mut bitmap = scrolled_bitmap;
    let mut tile = TileData::new();
    let mut tile_big = TileData::new();
    for i in 0..oam_table.table.len()
    {
        let y_position = oam_table.table[i].y_position as i16 - 16;
        let x_position = oam_table.table[i].x_position as i16 - 8;
        let tile_num = oam_table.table[i].tile_number as usize;
        if !large_sprites
        {
            tile = tile_map.tiles[tile_num].clone(); 
        }
        else 
        {
            tile = tile_map.tiles[tile_num & 0xFE].clone();
            tile_big = tile_map.tiles[tile_num | 0x01].clone();
        }

        let flags = oam_table.table[i].flags;
        let y_flip = (flags & 0x40) >> 6;
        let x_flip = (flags & 0x20) >> 5;
        let mut palette = 0;
        let behind = (flags & 0x80) >> 7;
        if ((flags & 0x10) >> 4) == 1
        {
            palette = palette_1;
        }
        else 
        {
            palette = palette_0;    
        }

        if !large_sprites
        {
            if y_flip == 1
            {
                for row in 0..4
                {
                    for index in 0..8
                    {
                        let temp = tile.data[(row * 8) + index];
                        tile.data[(row * 8) + index] = tile.data[((7 - row) * 8) + index];
                        tile.data[((7 - row) * 8) + index] = temp;
                    }
                }
            }

            if x_flip == 1
            {
                for column in 0..4
                {
                    for row in 0..8
                    {
                        let temp = tile.data[(row * 8) + column];
                        tile.data[(row * 8) + column] = tile.data[(row * 8) + (7 - column)];
                        tile.data[(row * 8) + (7 - column)] = temp;
                    }
                }
            }
        }
        else 
        {
            if y_flip == 1
            {
                for row in 0..8
                {
                    for index in 0..8
                    {
                        let temp = tile.data[(row * 8) + index];
                        tile.data[(row * 8) + index] = tile_big.data[((7 - row) * 8) + index];
                        tile_big.data[((7 - row) * 8) + index] = temp;
                    }
                }
            }

            if x_flip == 1
            {
                for column in 0..4
                {
                    for row in 0..8
                    {
                        let temp = tile.data[(row * 8) + column];
                        tile.data[(row * 8) + column] = tile.data[(row * 8) + (7 - column)];
                        tile.data[(row * 8) + (7 - column)] = temp;
                    }
                    for row in 0..8
                    {
                        let temp = tile_big.data[(row * 8) + column];
                        tile_big.data[(row * 8) + column] = tile_big.data[(row * 8) + (7 - column)];
                        tile_big.data[(row * 8) + (7 - column)] = temp;
                    }
                }
            }
        }
        
        if !large_sprites
        {
            for row in 0..8
            {
                for index in 0..8
                {
                    if (y_position + row) >= 0 && (y_position + row) < 144
                    {
                        if (x_position + index) >= 0 && (x_position + index) < 160
                        {
                            let tile_position = ((y_position + row) * 160) + (x_position + index);
                            let pixel = tile.data[(row as usize* 8) + index as usize];
                            if pixel != 0
                            {
                                let shade = pixel_shade_map(pixel, palette);
                                if behind == 0 || bitmap[tile_position as usize] == 0
                                {
                                    bitmap[tile_position as usize] = shade;
                                }
                            }
                        }
                    }
                }
            }
        }
        else 
        {
            for row in 0..8
            {
                for index in 0..8
                {
                    if (y_position + row) >= 0 && (y_position + row) + 8 < 144
                    {
                        if (x_position + index) >= 0 && (x_position + index) < 160
                        {
                            let mut tile_position = ((y_position + row) * 160) + (x_position + index);
                            let mut pixel = tile.data[(row as usize* 8) + index as usize];
                            if pixel != 0
                            {
                                let shade = pixel_shade_map(pixel, palette);
                                bitmap[tile_position as usize] = shade;
                            }

                            tile_position = ((y_position + row + 8) * 160) + (x_position + index);
                            pixel = tile_big.data[(row as usize* 8) + index as usize];
                            if pixel != 0
                            {
                                let shade = pixel_shade_map(pixel, palette);
                                bitmap[tile_position as usize] = shade;
                            }
                        }
                    }
                }
            }
        }
    }
    return bitmap;
}


//Split into two
fn pixel_shade_map(pixel_data: u8, palette_data: u8,) -> u8
{
    let mut pixel_shade = 0;
    match pixel_data
    {
        0 => return palette_data & 0x03,
        1 => return (palette_data & 0x0C) >> 2,
        2 => return (palette_data & 0x30) >> 4,
        3 => return (palette_data & 0xC0) >> 6,
        _ => return 4,
    }
}

fn pixel_color_map(pixel_shade: u8, shade_profile: &ShadeProfile) -> Rgba<u8>
{
    match pixel_shade 
    {
        0 => return shade_profile.shade_0,
        1 => return shade_profile.shade_1,
        2 => return shade_profile.shade_2,
        3 => return shade_profile.shade_3,
        _ => return shade_profile.default,
    }
}


#[cfg(test)]
mod gpu_tests
{
    use system::get_system_data;
    use gpu::gpu::*;

    #[test]
    fn get_tile_data_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));

        let temp_memory_values : Vec<u8> = vec![0b00110011, 0b00001111, 0b11001100, 0b11110000];

        for vram_offset in vec![0x8000, 0x8800].iter()
        {
            for i in 0..temp_memory_values.len()
            {
                system_data.mmu.mem_map[*vram_offset + i] = temp_memory_values[i];
            }
        }
        let tiles = vec![get_tile_data(0, &mut system_data, false), get_tile_data(0, &mut system_data, true)];
        for tile in tiles.iter()
        {
            for i in 0..4
            {
                assert_eq!(tile.data[i as usize * 2], i);
                assert_eq!(tile.data[(i as usize * 2) + 1 ], i);
                assert_eq!(tile.data[8 + (i as usize * 2)], 3 - i);
                assert_eq!(tile.data[(8 + (i as usize * 2)) + 1], 3 - i);
            }
        }
    }

    #[test]
    fn get_tile_map_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut TileMap = TileMap::new();

        let memory_values: Vec<u8> = vec![0x00, 0xFF];
        let test_values: Vec<u8> = vec![0, 3];
        let tiles_offset: Vec<usize> = vec![0x8800, 0x8000];
        let bools: Vec<bool> = vec![false, true];

        for offset_index in 0..tiles_offset.len()
        {
            for index in 0..test_values.len()
            {
                for i in 0..0x1000
                {
                    system_data.mmu.mem_map[tiles_offset[offset_index] + i] = memory_values[index];   
                }
                
                TileMap.populate_tile_map(&mut system_data, bools[offset_index], true);
                
                for i in 0..TileMap.tiles.len()
                {
                    for j in 0..64
                    {
                        assert_eq!(TileMap.tiles[i].data[j], test_values[index]);
                    }
                }
            }
        }

        let display_offset: Vec<usize> = vec![0x9800, 0x9C00];

        for offset_index in 0..display_offset.len()
        {
            for value_index in 0..memory_values.len()
            {
                for i in 0..TileMap.map.len()
                {
                    system_data.mmu.mem_map[display_offset[offset_index] + i] = memory_values[value_index];
                }

                TileMap.populate_tile_map(&mut system_data, false, bools[offset_index]);

                for i in 0..TileMap.map.len()
                {
                    assert_eq!(TileMap.map[i], memory_values[value_index]);
                }
            }
        }   

    }

}
