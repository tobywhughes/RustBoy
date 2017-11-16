use system::*;

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


pub fn get_tile_data(tile_index: u8, system_data: &mut SystemData) -> TileData
{
    let mut tile_data = TileData::new();
    let mem_loc: u16 = 0x8000 + (tile_index as u16 * 16);
    for byte_pair in 0..8
    {
        for bit in 0..8
        {
            let upper_bit: u8 = (system_data.mem_map[(mem_loc + (2 * byte_pair as u16)) as usize] >> (7 - bit)) & 0x01;
            let lower_bit: u8 = (system_data.mem_map[(mem_loc + (2 * byte_pair as u16)) as usize + 1] >> (7 - bit)) & 0x01;
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
}

pub fn LCD_Y_Coordinate_Update(system_data: &mut SystemData, gpu_registers: &mut GPU_Registers)
{
    gpu_registers.ly_cycle_count += system_data.cycles as u32;
    gpu_registers.ly_sub_cycle_count += system_data.cycles as u16;
    if (gpu_registers.ly_cycle_count >= 70224)
    {
        gpu_registers.ly_cycle_count -= 70224;        
    }
    if (gpu_registers.ly_sub_cycle_count >= 456)
    {
        gpu_registers.ly_register += 1;
        if gpu_registers.ly_register == 154
        {
            gpu_registers.ly_register = 0;
        }
        system_data.mem_map[0xFF44] = gpu_registers.ly_register;
        gpu_registers.ly_sub_cycle_count -= 456;
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
        system_data.mem_map[0x8000] = 0b00001111;
        system_data.mem_map[0x8001] = 0b00110011;
        system_data.mem_map[0x8002] = 0b11110000;
        system_data.mem_map[0x8003] = 0b11001100;
        let tile_data = get_tile_data(0, &mut system_data);
        for i in 0..4
        {
            assert_eq!(tile_data.data[i as usize * 2], i);
            assert_eq!(tile_data.data[(i as usize * 2) + 1 ], i);
        }
        for i in 0..4
        {
            assert_eq!(tile_data.data[8 + (i as usize * 2)], 3 - i);
            assert_eq!(tile_data.data[(8 + (i as usize * 2)) + 1], 3 - i);
        }
    }
}
