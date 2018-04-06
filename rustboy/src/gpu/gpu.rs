use system::*;
use gpu::gpu_registers::GPU_Registers;
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
            let upper_bit: u8 = (system_data.mmu.mem_map[(mem_loc + (2 * byte_pair as u16)) as usize] >> (7 - bit)) & 0x01;
            let lower_bit: u8 = (system_data.mmu.mem_map[(mem_loc + (2 * byte_pair as u16)) as usize + 1] >> (7 - bit)) & 0x01;
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
            system_data.mmu.mem_map[0xFFFE] |= 0x01;
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
            system_data.mmu.mem_map[0xFFFE] &= 0xFE;
        }
    }
}

pub fn create_background_img(background_tile_map: &TileMap, gpu_registers: &GPU_Registers) -> RgbaImage
{
    let mut image_buffer = ImageBuffer::new(160, 144);
    let background_buffer = build_background_bitmap(background_tile_map);
    for row_y in 0..144
    {
        for row_x in 0..160
        {
           let mut row_x_scrolled = (row_x + gpu_registers.lcd_position.scroll_x_buffer[row_y] as usize) % 256;
           let mut row_y_scrolled = (row_y + gpu_registers.lcd_position.scroll_y_buffer[row_y] as usize) % 256;
           let pixel_data = background_buffer[(row_y_scrolled * 256) + row_x_scrolled];
           let pixel = pixel_color_map(pixel_data);
           image_buffer.put_pixel(row_x as u32, row_y as u32, pixel);
        }
    }
   return image_buffer;
}

fn build_background_bitmap(background_tile_map: &TileMap) -> Vec<u8>
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
                    let tile = background_tile_map.map[(tile_y * 32) + tile_x];
                    let pixel_data = background_tile_map.tiles[tile as usize].data[(pixel_y * 8) + pixel_x];
                    buffer[(256 * ((tile_y * 8) + pixel_y)) + ((tile_x * 8) + pixel_x)] = pixel_data;
                }
            }
        }
    }
    return buffer;
}

fn pixel_color_map(pixel_data: u8) -> Rgba<u8>
{
    match pixel_data 
    {
        0 => return Rgba([156,189,15, 0xFF]),
        1 => return Rgba([140,173,15, 0xFF]),
        2 => return Rgba([48,98,48, 0xFF]),
        3 => return Rgba([15, 56, 15, 0xFF]),
        _ => return Rgba([0, 0, 0, 0xFF]),
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

        let temp_memory_values : Vec<u8> = vec![0b00001111, 0b00110011, 0b11110000, 0b11001100];

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
