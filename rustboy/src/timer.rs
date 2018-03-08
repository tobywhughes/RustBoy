use system::SystemData;

pub struct Timer
{
    pub divider_register: u8,
    pub timer_counter: u8,
    pub timer_modulo: u8,
    pub timer_control: u8,
    pub tima_cycles: u16,
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
            tima_cycles: 0,
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

    pub fn tima_tick(&mut self, system_data: &mut SystemData) -> bool
    {
        if (self.timer_control & 0x04) == 0x00
        {
            return false;
        }
        let mut overflow_flag = false;
        self.tima_cycles += system_data.cycles as u16;
        
        let cycle_tick_threshold = self.map_timer_control_speed();
        if self.tima_cycles >= cycle_tick_threshold
        {
            self.tima_cycles -= cycle_tick_threshold;
            if self.timer_counter == 0xFF
            {
                self.timer_counter = self.timer_modulo;
                overflow_flag = true;
            }
            else 
            {
                self.timer_counter += 1;
            }
            system_data.mmu.mem_map[0xFF05] = self.timer_counter;
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
    fn tick_test() {
        let mut system_data : SystemData = get_system_data(&String::from("CLASSIC"));
        let mut timer = Timer::new();
        timer.timer_control = 0x05;
        timer.timer_modulo = 0x10;
        system_data.cycles = 1;
        let mut interrupt = false;
        
        for tick in 0..0xFF
        {
            for cycles in 0..0x10
            {
                interrupt = timer.tima_tick(&mut system_data);
            }
            
            assert_eq!(timer.timer_counter, tick + 1);
            assert!(!interrupt);
        }
        for cycles in 0..0x10
        {
            interrupt = timer.tima_tick(&mut system_data);
        }
        assert_eq!(timer.timer_counter, 0x10);
        assert!(interrupt);
    }
}