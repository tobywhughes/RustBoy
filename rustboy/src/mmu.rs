pub struct MMU
{
    pub mem_map: Vec<u8>,
    pub cartridge_type: u8,
    pub rom_size: u32,
    pub ram_size: u16,
}

impl MMU
{
    pub fn new() -> MMU
    {
        return MMU
        {
            mem_map: vec![0; 0x10000],
            cartridge_type: 0,
            rom_size: 0x00008000,
            ram_size: 0x0000,
        }
    }

    pub fn set_to_memory(&mut self, location: usize, value: u8, masked_set: bool)
    {
        self.mem_map[location] = value;
    }

    pub fn get_from_memory(&self, location: usize, masked_read: bool) -> u8
    {
        return self.mem_map[location];
    }
}

#[cfg(test)]
mod mmu_tests
{
    use MMU;
    
    #[test]
    fn get_and_set_legal_memory_test() {
        let mut mmu = MMU::new();
        mmu.set_to_memory(0x1234, 0xFF, false);
        assert_eq!(mmu.mem_map[0x1234], 0xFF);
        assert_eq!(mmu.get_from_memory(0x1234, false), 0xFF);
    }
}