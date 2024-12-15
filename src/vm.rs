mod registers;
mod instructions;
mod trap_routines;
mod utils;

use std::fs::File;
use std::io::{Read, stdin, stdout, Write};
use crate::vm::registers::{MemoryMappedRegister, Register, Registers};
use crate::vm::instructions::{*};
use crate::vm::registers::Flags::{FL_NEG, FL_POS, FL_ZRO};
use crate::vm::trap_routines::{*};
use crate::vm::utils::{*};
const MEMORY_MAX: usize = std::u16::MAX as usize + 1;
const START_POSITION: u16 = 0x3000;

pub struct Vm {
    memory: [u16; MEMORY_MAX],
    registers: Registers,
    active: bool
}

impl Vm {
    pub fn new() -> Self {
        Self { memory: [0; MEMORY_MAX], registers: Registers::new(), active: false}
    }

    pub fn start(&mut self, arg: String) {
        let file = File::open(arg).expect("should provide existing file");
        self.read_image(file);

        self.registers.write(Register::R_PC, START_POSITION);
        self.active = true;

        while self.active {
            let pc = self.registers.read(Register::R_PC);
            let instruction = self.mem_read(pc);

            self.registers.write(Register::R_PC, pc.overflowing_add(1).0);

            let op: u16 = instruction >> 12;
            let op = OpCodes::new(op);
            match op
            {
                OpCodes::OP_ADD => add(self, instruction),
                OpCodes::OP_AND => and(self, instruction),
                OpCodes::OP_NOT => not(self, instruction),
                OpCodes::OP_BR => branch(self, instruction),
                OpCodes::OP_JMP => jmp(self, instruction),
                OpCodes::OP_JSR => jsr(self, instruction),
                OpCodes::OP_LD => ld(self, instruction),
                OpCodes::OP_LDI => ldi(self, instruction),
                OpCodes::OP_LDR => ldr(self, instruction),
                OpCodes::OP_LEA => lea(self, instruction),
                OpCodes::OP_ST => st(self, instruction),
                OpCodes::OP_STI => sti(self, instruction),
                OpCodes::OP_STR => str(self, instruction),
                OpCodes::OP_TRAP => self.trap(instruction),
                OpCodes::OP_RES | OpCodes::OP_RTI => break
            }
        }
    }

    fn trap(&mut self, instruction: u16) {
        let trap = TrapRoutine::new(instruction & 0xFF);

        match trap {
            TrapRoutine::TRAP_GETC => get_char(self),
            TrapRoutine::TRAP_OUT => trap_out(self),
            TrapRoutine::TRAP_PUTS => trap_put(self),
            TrapRoutine::TRAP_IN => trap_in(self),
            TrapRoutine::TRAP_PUTSP => trap_putsp(self),
            TrapRoutine::TRAP_HALT => halt(self)
        }
    }

    fn mem_read(&mut self, address: u16) -> u16{
        if address == MemoryMappedRegister::KeyboardStatusRegister as u16 {
            match stdin().bytes().next() {
                None => {
                    self.memory[MemoryMappedRegister::KeyboardStatusRegister as usize] = 0;
                }
                Some(a_byte) => {
                    let character = a_byte.expect("should not fail") as u16;
                    if character != 10 {
                        self.memory[MemoryMappedRegister::KeyboardStatusRegister as usize] = 1 << 15;
                        self.memory[MemoryMappedRegister::KEYBOARD_DATA_REGISTER as usize] = character;
                    } else {
                        self.memory[MemoryMappedRegister::KeyboardStatusRegister as usize] = 0;
                    }
                }
            }
        }
        self.memory[address as usize]
    }

    fn mem_write(&mut self, address : u16, value: u16) {
        self.memory[address as usize] = value;
    }

    fn update_flags(&mut self, r: Register) {
        let value = self.registers.read(r);

        if value == 0 {
            self.registers.write(Register::R_COND, FL_ZRO as u16);
        }
        else if value >> 15 == 1{
            self.registers.write(Register::R_COND, FL_NEG as u16);
        }
        else {
            self.registers.write(Register::R_COND, FL_POS as u16);
        }
    }

    fn read_image(&mut self, mut program: File) {
        let mut buffer: [u8; 2] = [0; 2];
        program.read(&mut buffer).expect("should not fail");
        let mut origin = swap_endian(buffer);
        loop {
            match program.read(&mut buffer) {
                Ok(2) => {
                    self.memory[origin as usize] = swap_endian(buffer);
                    origin = origin.overflowing_add(1).0;
                }
                Ok(0) => break,
                _ => panic!()
            }
        }
    }
}
