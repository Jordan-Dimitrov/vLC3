mod registers;
mod instructions;

use registers::R_COUNT;

const MEMORY_MAX: usize = 65536;

struct Vm {
    memory: [u16; MEMORY_MAX],
    registers: [u16; R_COUNT]
}

impl Vm {
    fn new() -> Self {
        Self { memory: [0; MEMORY_MAX], registers: [0; R_COUNT]}
    }
}
