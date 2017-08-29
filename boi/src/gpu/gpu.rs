use system::*;

pub struct TileData
{
    pub data: Vec<u8>,
    pub mem_loc: u16,
}

impl TileData
{
    pub fn new() -> TileData
    {
        return TileData
        {
            data: vec![0; 64],
            mem_loc: 0
        }
    }
}


pub fn get_tile_data(tile_index: u8, system_data: &mut SystemData) -> TileData
{
    let mut tile_data = TileData::new();
    tile_data.mem_loc = 0x8000 + (tile_index * 16);
    for byte_pair in 0..8
    {
        for bit in 0..8
        {
            
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