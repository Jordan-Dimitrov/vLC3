pub enum OpCodes
{
    OP_BR,
    OP_ADD,
    OP_LD,
    OP_ST,
    OP_JSR,
    OP_AND,
    OP_LDR,
    OP_STR,
    OP_RTI,
    OP_NOT,
    OP_LDI,
    OP_STI,
    OP_JMP,
    OP_RES,
    OP_LEA,
    OP_TRAP
}

impl OpCodes {
    pub fn new(value: u16) -> Self {
        match value {
            0 => OpCodes::OP_BR,
            1 => OpCodes::OP_ADD,
            2 => OpCodes::OP_LD,
            3 => OpCodes::OP_ST,
            4 => OpCodes::OP_JSR,
            5 => OpCodes::OP_AND,
            6 => OpCodes::OP_LDR,
            7 => OpCodes::OP_STR,
            8 => OpCodes::OP_RTI,
            9 => OpCodes::OP_NOT,
            10 => OpCodes::OP_LDI,
            11 => OpCodes::OP_STI,
            12 => OpCodes::OP_JMP,
            13 => OpCodes::OP_RES,
            14 => OpCodes::OP_LEA,
            15 => OpCodes::OP_TRAP,
            _ => panic!("invalid opcode"),
        }
    }
}