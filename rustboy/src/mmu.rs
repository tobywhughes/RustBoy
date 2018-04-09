use std::fs::File;
use std::io::prelude::*;
use std::io;

pub struct MMU
{
    pub mem_map: Vec<u8>,
    pub memory_banks: Vec<Vec<u8>>,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub rom_bank: u8,
    pub banking_mode: u8,
}

impl MMU
{
    pub fn new() -> MMU
    {
        return MMU
        {
            mem_map: vec![0; 0x10000],
            memory_banks: vec![vec![0; 0x4000]; 0x100],
            cartridge_type: 0,
            rom_size: 0x00,
            ram_size: 0x00,
            rom_bank: 1,
            banking_mode: 0,
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
        // if masked_read == true && location < 0x8000
        // {
        //     println!("DEBUG: Masked Read Location-{:04x}", location);
        // }
        return self.mem_map[location];
    }

    fn mbc1_parse(&mut self, location: usize, value: u8) -> bool
    {
        
        match location
        {
            0x0000...0x1FFF =>
            {
                println!("Value: 0x{:02X} Location: 0x{:04X}", value, location);
                println!("DEBUG: ram enable` {}", value);
                return true;
            },
            0x2000...0x3FFF => 
            {
            //     for i in 0..0x4000
            //     {
            //         self.memory_banks[self.rom_bank as usize][i] = self.mem_map[(i as usize) + 0x4000];
            //     }
            println!("Value: 0x{:02X} Location: 0x{:04X}", value, location);
                let mut bank = value & 0x1F;
                if bank == 0
                {
                    bank += 1;
                }
                self.rom_bank &= 0xE0;
                self.rom_bank |= bank;
                self.update_rom_bank();
                return true;
            },
            0x4000...0x5FFF =>
            {
                // for i in 0..0x4000
                // {
                //     self.memory_banks[self.rom_bank as usize][i] = self.mem_map[(i as usize) + 0x4000];
                // }
                println!("Value: 0x{:02X} -- {} Location: 0x{:04X}", value, (value & 0x03) << 5 ,location);
                self.rom_bank &= 0x1F;
                if self.banking_mode == 0{
                    self.rom_bank |= ((value & 0x03) << 5);
                }
                self.update_rom_bank();
                return true;
            },
            0x6000...0x7FFF =>
            {
                println!("Value: 0x{:02X} Location: 0x{:04X}", value, location);
                println!("DEBUG: mode {}", value);
                self.banking_mode = value;
                return true;
            },
            _ => 
            {
                 //println!("Value: 0x{:02X} Location: 0x{:04X}", value, location); 
                 //io::stdin().read_line(&mut String::new());
                 return false;
            },
        }
    }

    fn update_rom_bank(&mut self)
    {
        let bank = self.rom_bank as usize;
        println!("Rom-bank switch: 0x{:02x}", bank);
        for i in 0..0x4000
        {
            self.mem_map[(i as usize) + 0x4000] = self.memory_banks[bank][i];
        }
    }

    pub fn read_gb_file(&self, file_name: &str) -> Vec<u8>
    {
    let mut buffer : Vec<u8> = vec![0; 0x10000];
    let file = File::open(file_name);
    if file.is_ok()
    {
        if file.unwrap().read(&mut buffer).is_ok()
        {
            return buffer;
        }        
    }
    return buffer;
    }

    pub fn initialize_cartridge(&mut self, file_name: &str)
    {
        let mut buffer: Vec<u8> = Vec::new();
        let file = File::open(file_name);
        if file.is_ok()
        {
            if file.unwrap().read_to_end(&mut buffer).is_ok()
            {
                self.memory_banks[0] = buffer[0..0x4000].to_vec();
                self.cartridge_type = self.memory_banks[0][0x0147];
                let rom_tag = self.memory_banks[0][0x0148];
                self.rom_size = self.parse_rom_size(rom_tag);
                let ram_tag = self.memory_banks[0][0x0149];
                self.ram_size = self.parse_ram_size(ram_tag);
                if self.rom_size == 0
                {
                    self.memory_banks[1] = buffer[0x4000..0x8000].to_vec();
                }
                else
                {
                    for i in 1..self.rom_size
                    {
                        let start: usize = 0x4000 * (i as usize);
                        let end: usize = 0x4000 * ((i as usize) + 1);
                        self.memory_banks[i as usize] = buffer[start..end].to_vec();
                    }
                }
                for i in 0..0x4000
                {
                    self.mem_map[i] = self.memory_banks[0][i];
                    self.mem_map[(i as usize) + 0x4000] = self.memory_banks[1][i];
                }
                
            }
        }
    }

    fn parse_rom_size(&mut self, rom_tag: u8) -> u8
    {
        match rom_tag
        {
            0x00 => return 0,
            0x01 => return 4,
            0x02 => return 8,
            0x03 => return 16,
            0x04 => return 32,
            0x05 => return 64,
            0x06 => return 128,
            0x07 => return 256,
            0x52 => return 72,
            0x53 => return 80,
            0x54 => return 96,
            _ => return 0,
        }
    }

    fn parse_ram_size(&mut self, ram_tag: u8) -> u8
    {
        match ram_tag
        {
            0x00 => return 0,
            0x01 => return 2,
            0x02 => return 8,
            0x03 => return 32,
            _ => return 0,
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

    //#[test]
    fn mbc1_test() {
        let mut mmu = MMU::new();
        mmu.cartridge_type = 0x01;

        let memory_locations: Vec<usize> = vec![0x00, 0x2000, 0x3000, 0x3FFF, 0x4000];
        let values: Vec<u8> = vec![0xFF, 0xFF, 0x00, 0xFF, 0xFF, 0x03];
        let rom_bank_values: Vec<u8> = vec![0x01, 0x1F, 0x01, 0x1F, 0x61];
        let mem_loc_values: Vec<u8> = vec![0xFF, 0x00, 0x00, 0x00, 0x00];

        for i in 0..memory_locations.len()
        {
            mmu.rom_bank = 0x01;
            mmu.mem_map[memory_locations[i]] = 0;
            mmu.set_to_memory(memory_locations[i], values[i], true);
            assert_eq!(mmu.rom_bank, rom_bank_values[i]);
            assert_eq!(mmu.mem_map[memory_locations[i]], mem_loc_values[i]);
        }
    }
    
    #[test]
    fn passing_bad_filename_to_read_gb_file_return_empty_vec()
    {
        let mut mmu = MMU::new();
        let return_vector : Vec<u8> = mmu.read_gb_file("");
        assert_eq!(return_vector, vec![0;0x10000]);
    }

    #[test]
    fn initialize_catridge_test()
    {
        let mut mmu = MMU::new();
        mmu.initialize_cartridge("roms/cpu_instrs.gb");
        assert_eq!(mmu.rom_size, 4);
        assert_eq!(mmu.ram_size, 0);
        assert_eq!(mmu.cartridge_type, 1);
        assert_eq!(mmu.memory_banks[0][0x0000], 0x3C);
        assert_eq!(mmu.memory_banks[1][0x0000], 0xC3);
        assert_eq!(mmu.memory_banks[2][0x0000], 0xC3);
        assert_eq!(mmu.memory_banks[3][0x0000], 0xC3);
    }

    #[test]
    fn rom_bank_switch_test() 
    {
        let mut mmu = MMU::new();
        mmu.initialize_cartridge("roms/cpu_instrs.gb");
        assert_eq!(mmu.rom_size, 4);
        assert_eq!(mmu.ram_size, 0);
        assert_eq!(mmu.cartridge_type, 1);
        assert_eq!(mmu.mem_map[0x0000], 0x3C);
        assert_eq!(mmu.mem_map[0x4300], 0x3E);
        mmu.mbc1_parse(0x2000, 0x00);
        assert_eq!(mmu.mem_map[0x4300], 0x3E);
        mmu.mbc1_parse(0x2000, 0x02);
        assert_eq!(mmu.mem_map[0x4300], 0xE0);
        mmu.mbc1_parse(0x2000, 0x03);
        assert_eq!(mmu.mem_map[0x4900], 0x72);
        mmu.mbc1_parse(0x2000, 0x00);
        assert_eq!(mmu.mem_map[0x4300], 0x3E);
    }
}