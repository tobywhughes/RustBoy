use system::SystemData;
use system::Registers;
use cpu::opcode::parse_opcode;
//use cpu::parse_opcode;
pub fn cpu_continue(system_data: &mut SystemData, registers: &mut Registers) {
    //Splitting borrows due to borrow lock
    let mut system_data_borrow = system_data;
    let mut registers_borrow = registers;
    
    parse_opcode(&mut system_data_borrow, &mut registers_borrow);
}