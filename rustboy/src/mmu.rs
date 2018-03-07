pub struct MMU
{
    pub mem_map: Vec<u8>,
    pub cartridge_type: u8,
    pub rom_size: u32,
    pub ram_size: u16,
    pub rom_bank: u8,
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
            rom_bank: 1,
        }
    }

    pub fn set_to_memory(&mut self, location: usize, value: u8, masked_set: bool)
    {
        let mut rom_flag = false;
        if masked_set
        {
            match self.cartridge_type
            {
                0x00 => (),
                0x01 => rom_flag = self.mbc1_parse(location, value),
                _ => (),
            }
        }
        if !rom_flag
        {
            self.mem_map[location] = value;
        }
    }

    pub fn get_from_memory(&self, location: usize, masked_read: bool) -> u8
    {
        return self.mem_map[location];
    }

    fn mbc1_parse(&mut self, location: usize, value: u8) -> bool
    {
        match location
        {
            0x2000...0x3FFF => 
            {
                let mut bank = value & 0x1F;
                if bank == 0
                {
                    bank += 1;
                }
                self.rom_bank &= 0xE0;
                self.rom_bank |= bank;
                return true;
            },
            _ => return false,
        }
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

    #[test]
    fn mbc1_test() {
        let mut mmu = MMU::new();
        mmu.cartridge_type = 0x01;

        let memory_locations: Vec<usize> = vec![0x00, 0x2000, 0x3000, 0x3FFF, 0x4000];
        let values: Vec<u8> = vec![0xFF, 0xFF, 0x00, 0xFF, 0xFF];
        let rom_bank_values: Vec<u8> = vec![0x01, 0x1F, 0x01, 0x1F, 0x01];
        let mem_loc_values: Vec<u8> = vec![0xFF, 0x00, 0x00, 0x00, 0xFF];

        for i in 0..memory_locations.len()
        {
            mmu.rom_bank = 0x01;
            mmu.mem_map[memory_locations[i]] = 0;
            mmu.set_to_memory(memory_locations[i], values[i], true);
            assert_eq!(mmu.rom_bank, rom_bank_values[i]);
            assert_eq!(mmu.mem_map[memory_locations[i]], mem_loc_values[i]);
        }
    }
}