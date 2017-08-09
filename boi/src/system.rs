pub struct SystemData
{
    pub mem_map: Vec<u8>,
    pub width: u16,
    pub tile_width: u16,
    pub height: u16,
    pub tile_height: u16,
    pub clock_speed: u32,
    pub horizontal_sync: u32,
    pub vertical_sync: f64,
}

pub fn get_system_data(emulator_type: &str) -> SystemData
{
    match emulator_type.as_ref()
    {
        "CLASSIC" => return SystemData
        {
            mem_map: vec![0; 0x10000],
            width: 160,
            tile_width: 20,
            height: 144,
            tile_height: 18,
            clock_speed: 4194304,
            horizontal_sync: 9198000,
            vertical_sync: 59.73,
        },
        _ => {println!("NOT VALID EMULATOR TYPE");
        return SystemData
        {
            mem_map: vec![0; 0],
            width: 0,
            tile_width: 0,
            height: 0,
            tile_height: 0,
            clock_speed: 0,
            horizontal_sync: 0,
            vertical_sync: 0.0,
        }},

    }

}

#[cfg(test)]
mod main_tests
{

    use get_system_data;
    use SystemData;

    #[test]
    fn passing_bad_data_to_get_system_data_returns_empty_struct_data()
    {
        let system_data : SystemData = get_system_data("");
        assert_eq!(system_data.mem_map, vec![0; 0]);
        assert_eq!(system_data.width, 0);
        assert_eq!(system_data.tile_width, 0);
        assert_eq!(system_data.height, 0);
        assert_eq!(system_data.tile_height, 0);
        assert_eq!(system_data.clock_speed, 0);
        assert_eq!(system_data.horizontal_sync, 0);
        assert_eq!(system_data.vertical_sync, 0.0);
    }
}