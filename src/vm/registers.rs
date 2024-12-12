pub const R_COUNT: usize = 10;

pub enum Register
{
    R_R0 = 0,
    R_R1 = 1,
    R_R2 = 2,
    R_R3 = 3,
    R_R4 = 4,
    R_R5 = 5,
    R_R6 = 6,
    R_R7 = 7,
    R_PC = 8,
    R_COND = 9,
}

impl Register {
    pub fn new(value: u16) -> Register {
        match value {
            0 => Register::R_R0,
            1 => Register::R_R1,
            2 => Register::R_R2,
            3 => Register::R_R3,
            4 => Register::R_R4,
            5 => Register::R_R5,
            6 => Register::R_R6,
            7 => Register::R_R7,
            8 => Register::R_PC,
            9 => Register::R_COND,
            _ => panic!("invalid register"),
        }
    }
}
pub struct Registers {
    registers: [u16; R_COUNT]
}

impl Registers {
    pub fn new() -> Self {
        Self { registers: [0; R_COUNT]}
    }

    pub fn read(&self, register: Register) -> u16 {
        self.registers[register as usize]
    }

    pub fn write(&mut self, register: Register, value: u16) {
        self.registers[register as usize] = value
    }


}
pub enum Flags
{
    FL_POS = 1 << 0,
    FL_ZRO = 1 << 1,
    FL_NEG = 1 << 2,
}