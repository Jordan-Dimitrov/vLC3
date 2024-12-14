pub enum TrapRoutine
{
    TRAP_GETC,
    TRAP_OUT,
    TRAP_PUTS,
    TRAP_IN,
    TRAP_PUTSP,
    TRAP_HALT
}

impl TrapRoutine {
    pub fn new(value: u16) -> Self {
        match value {
            0x20 => TrapRoutine::TRAP_GETC,
            0x21 => TrapRoutine::TRAP_OUT,
            0x22 => TrapRoutine::TRAP_PUTS,
            0x23 => TrapRoutine::TRAP_IN,
            0x24 => TrapRoutine::TRAP_PUTSP,
            0x25 => TrapRoutine::TRAP_HALT,
            _ => panic!("invalid routine"),
        }
    }
}