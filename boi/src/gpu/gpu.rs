use system::*;

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