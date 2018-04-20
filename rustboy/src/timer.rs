use system::SystemData;

pub struct Timer
{
    pub divider_register: u8,
    pub timer_counter: u8,
    pub timer_modulo: u8,
    pub timer_control: u8,
    pub tima_cycles: u16,
    pub divider_cycles: u16,
    pub cycle_register: u16,
    pub tima_increment: u8,
}

impl Timer
{
    pub fn new() -> Timer
    {
        return Timer
        {
            divider_register: 0,
            timer_counter: 0,
            timer_modulo: 0,
            timer_control: 0,
            divider_cycles: 0,
            tima_cycles: 0,
            cycle_register: 0,
            tima_increment: 0,
        }
    }

    pub fn cycle_calculation(&mut self, mut cycles: u8, reset:bool)
    {
        let tac_check = self.map_tac_check();
        if reset
        {
            let previous_tac_bit = ((self.cycle_register >> tac_check) & 0x0001) as u8 & ((self.timer_control & 0x04) >> 2);
            self.cycle_register = 0;
            let new_tac_bit = ((self.cycle_register >> tac_check) & 0x0001) as u8 & ((self.timer_control & 0x04) >> 2);
            if (previous_tac_bit & !new_tac_bit) == 1
            {
                self.tima_increment += 1;
            }
        }
        while cycles > 0
        {
            let previous_tac_bit = ((self.cycle_register >> tac_check) & 0x0001) as u8 & ((self.timer_control & 0x04) >> 2);
            //println!("{}", (self.cycle_register >> tac_check) & 0x0001);
            if self.cycle_register == 0xFFFC
            {
                self.cycle_register = 0;
            }
            else {
                self.cycle_register += 4;
            }
            cycles -= 4;
            let new_tac_bit = ((self.cycle_register >> tac_check) & 0x0001) as u8 & ((self.timer_control & 0x04) >> 2);
            if (previous_tac_bit & (!new_tac_bit & 0x01)) == 1
            {
                
                self.tima_increment += 1;
            }
        }

        self.divider_register = ((self.cycle_register & 0xFF00) >> 8) as u8;
    }

    pub fn map_tac_check(&self) -> u8
    {
        let select = self.timer_control & 0x03;
        match select
        {
            0x00 => return 9,
            0x01 => return 3,
            0x02 => return 5,
            0x03 => return 7,
            _ => return 0,
        }
    }

    pub fn map_timer_control_speed(&self) -> u16
    {
        let select = self.timer_control & 0x03;
        match select
        {
            0x00 => return 0x0400,
            0x01 => return 0x0010,
            0x02 => return 0x0040,
            0x03 => return 0x0100,
            _ => return 0,
        }
    }

    pub fn update_registers(&mut self, mem_map: &Vec<u8>)
    {
        self.divider_register = mem_map[0xFF04];
        self.timer_counter = mem_map[0xFF05];
        self.timer_modulo = mem_map[0xFF06];
        self.timer_control = mem_map[0xFF07];
    }

    pub fn divider_tick(&mut self, cycles: u8, reset: bool)
    {
        if reset
        {
            self.divider_cycles = 0;
            self.tima_cycles = 0;
        }
        self.divider_cycles += cycles as u16;
        if self.divider_cycles >= 0x0100
        {
            self.divider_cycles -= 0x0100;
            if self.divider_register == 0xFF
            {
                self.divider_register = 0x00;
            }
            else
            {
                self.divider_register += 1;
            }
        }
    }

//     pub fn tima_tick(&mut self, cycles: u8) -> bool
//     {
//         if (self.timer_control & 0x04) == 0x00
//         {
//             return false;
//         }
//         let mut overflow_flag = false;
//         self.tima_cycles += cycles as u16;
        
//         let cycle_tick_threshold = self.map_timer_control_speed();
//         if self.tima_cycles >= cycle_tick_threshold
//         {
//             let mut increment = 0;
//             while self.tima_cycles >= cycle_tick_threshold
//             {
//                 increment += 1;
//                 self.tima_cycles -= cycle_tick_threshold;
//             }
//             while increment > 0
//             {
//                 if self.timer_counter == 0xFF
//                 {
//                     self.timer_counter = self.timer_modulo;
                    
//                     overflow_flag = true;
//                 }
//                 else 
//                 {
//                     self.timer_counter += 1;
//                 }
//                 increment -= 1;
//             }
            
//         }
//         return overflow_flag;
//     }
// }

    pub fn tima_tick(&mut self, cycles: u8) -> bool
    {
        let mut overflow_flag = false;
        while self.tima_increment > 0
        {
            if self.timer_counter == 0xFF
            {
                self.timer_counter = self.timer_modulo;
                
                overflow_flag = true;
            }
            else 
            {
                self.timer_counter += 1;
            }
            self.tima_increment -= 1;
        }
            
        return overflow_flag;
    }
}

#[cfg(test)]
mod timer_tests
{
    use Timer;
    use system::*;
    
    #[test]
    fn timer_control_speed_test() {
        let mut timer = Timer::new();
        timer.timer_control = 0x00;
        assert_eq!(timer.map_timer_control_speed(), 0x400);
        timer.timer_control = 0x01;
        assert_eq!(timer.map_timer_control_speed(), 0x10);
        timer.timer_control = 0x02;
        assert_eq!(timer.map_timer_control_speed(), 0x40);
        timer.timer_control = 0x03;
        assert_eq!(timer.map_timer_control_speed(), 0x100);
    }

    #[test]
    fn update_register_test()
    {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        assert_eq!(system_data.timer.divider_register, 0);
        assert_eq!(system_data.timer.timer_counter, 0);
        assert_eq!(system_data.timer.timer_modulo, 0);
        assert_eq!(system_data.timer.timer_control, 0);
        system_data.mmu.mem_map[0xFF04] = 1;
        system_data.mmu.mem_map[0xFF05] = 1;
        system_data.mmu.mem_map[0xFF06] = 1;
        system_data.mmu.mem_map[0xFF07] = 1;
        system_data.timer.update_registers(&system_data.mmu.mem_map);
        assert_eq!(system_data.timer.divider_register, 1);
        assert_eq!(system_data.timer.timer_counter, 1);
        assert_eq!(system_data.timer.timer_modulo, 1);
        assert_eq!(system_data.timer.timer_control, 1);
    }

    #[test]
    fn tick_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut timer = Timer::new();
        timer.timer_control = 0x05;
        timer.timer_modulo = 0x10;
        system_data.cycles = 4;
        let mut interrupt = false;
        
        for tick in 0..0xFF
        {
            for cycles in 0..0x04
            {
                timer.cycle_calculation(system_data.cycles, false);
                interrupt = timer.tima_tick(system_data.cycles);
            }
            
            assert_eq!(timer.timer_counter, tick + 1);
            assert!(!interrupt);
        }
        for cycles in 0..0x04
        {
            timer.cycle_calculation(system_data.cycles, false);
            interrupt = timer.tima_tick(system_data.cycles);
        }
        assert_eq!(timer.timer_counter, 0x10);
        assert!(interrupt);
    }
}
