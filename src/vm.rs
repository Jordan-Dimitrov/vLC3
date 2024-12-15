mod registers;
mod instructions;
mod trap_routines;

use std::fs::File;
use std::io;
use std::io::{Read, stdin, stdout, Write};
use std::process::exit;
use crate::vm::registers::{MemoryMappedRegister, Register, Registers};
use crate::vm::instructions::{*};
use crate::vm::registers::Flags::{FL_NEG, FL_POS, FL_ZRO};
use crate::vm::trap_routines::{*};
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

        let file = File::open(arg).expect("should provide ");
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
                OpCodes::OP_ADD => self.add(instruction),
                OpCodes::OP_AND => self.and(instruction),
                OpCodes::OP_NOT => self.not(instruction),
                OpCodes::OP_BR => self.branch(instruction),
                OpCodes::OP_JMP => self.jmp(instruction),
                OpCodes::OP_JSR => self.jsr(instruction),
                OpCodes::OP_LD => self.ld(instruction),
                OpCodes::OP_LDI => self.ldi(instruction),
                OpCodes::OP_LDR => self.ldr(instruction),
                OpCodes::OP_LEA => self.lea(instruction),
                OpCodes::OP_ST => self.st(instruction),
                OpCodes::OP_STI => self.sti(instruction),
                OpCodes::OP_STR => self.str(instruction),
                OpCodes::OP_TRAP => self.trap(instruction),
                OpCodes::OP_RES | OpCodes::OP_RTI => break
            }
        }
    }

    fn trap(&mut self, instruction: u16) {
        let trap = TrapRoutine::new(instruction & 0xFF);

        match trap {
            TrapRoutine::TRAP_GETC => self.get_char(),
            TrapRoutine::TRAP_OUT => self.trap_out(),
            TrapRoutine::TRAP_PUTS => self.trap_put(),
            TrapRoutine::TRAP_IN => self.trap_in(),
            TrapRoutine::TRAP_PUTSP => self.trap_putsp(),
            TrapRoutine::TRAP_HALT => self.halt()
        }
    }

    fn halt(&mut self) {
        println!("HALT");
        self.active = false;
    }

    fn trap_put(&mut self) {
        let mut addr = self.registers.read(Register::R_R0);
        let mut character = self.memory[addr as usize];

        while character > 0 {
            print!("{}", (character & 0b1111_1111) as u8 as char);
            addr = addr.overflowing_add(1).0;
            character = self.memory[addr as usize];
        }
        stdout().flush().expect("should not fail");
    }

    fn get_char(&mut self) {
        let input: u16 = stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u16)
            .expect("should not fail");

        self.registers.write(Register::R_R0, input & 0b1111_1111)
    }

    fn trap_out(&mut self) {
        print!("{}", self.registers.read(Register::R_R0) as u8 as char);
        stdout().flush().expect("should not fail!");
    }

    fn trap_in(&mut self) {
        print!("Input: ");
        stdout().flush().expect("Should not fail");

        self.get_char();
    }

    fn trap_putsp(&mut self) {
        let mut addr = self.registers.read(Register::R_R0);
        let mut character = self.memory[addr as usize];

        while character > 0 {
            print!("{}", (character & 0b1111_1111) as u8 as char);
            let second_part = (character >> 8) & 0b1111_1111;
            if second_part == 0 {
                break;
            }
            print!("{}", second_part as u8 as char);
            addr = addr.overflowing_add(1).0;
            character = self.memory[addr as usize];
        }
        stdout().flush().expect("should not fail");
    }

    fn add(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let r1 = (instruction >> 6) & 0x7;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        let imm_flag = (instruction >> 5) & 0x1;

        if imm_flag != 0 {
            let imm5 = sign_extend(instruction & 0x1F, 5);
            self.registers.write(r0, r1.overflowing_add(imm5).0);
        }
        else
        {
            let r2: u16 = instruction & 0x7;
            let r2 = Register::new(r2);
            let r2 = self.registers.read(r2);

            self.registers.write(r0, r1.overflowing_add(r2).0);
        }

        self.update_flags(r0_clone);
    }

    fn ldi(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let pc_offset = sign_extend(instruction & 0x1FF, 9);

        let pc = self.registers.read(Register::R_PC);

        let value = self.mem_read(pc.overflowing_add(pc_offset).0);
        let read = self.mem_read(value);

        self.registers.write(r0, read);

        self.update_flags(r0_clone);

    }

    fn and(&mut self, instruction: u16)
    {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let r1 = (instruction >> 6) & 0x7;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        let imm_flag = (instruction >> 5) & 0x1;

        if imm_flag > 0 {
            let imm5 = sign_extend(instruction & 0x1F, 5);
            self.registers.write(r0, r1 & imm5);
        }
        else
        {
            let r2: u16 = instruction & 0x7;
            let r2 = Register::new(r2);
            let r2 = self.registers.read(r2);

            self.registers.write(r0, r1 & r2);
        }

        self.update_flags(r0_clone);
    }

    fn not(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let r1 = (instruction >> 6) & 0x7;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        self.registers.write(r0, !r1);
        self.update_flags(r0_clone);
    }

    fn branch(&mut self, instruction: u16) {
        let pc_offset = sign_extend(instruction & 0x1FF, 9);
        let cond_flag = (instruction >> 9) & 0x7;

        let r_cond = self.registers.read(Register::R_COND);
        let pc = self.registers.read(Register::R_PC);

        if cond_flag & r_cond > 0 {
            self.registers.write(Register::R_PC, pc.overflowing_add(pc_offset).0);
        }
    }

    fn jmp(&mut self, instruction: u16) {
        let r1 = (instruction >> 6) & 0x7;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        self.registers.write(Register::R_PC, r1);
    }

    fn jsr(&mut self, instruction: u16) {
        let long_flag = (instruction >> 11) & 1;
        let pc = self.registers.read(Register::R_PC);

        self.registers.write(Register::R_R7, pc);

        if long_flag > 0 {
            let long_pc_offset = sign_extend(instruction & 0x7FF, 11);
            self.registers.write(Register::R_PC, pc.overflowing_add(long_pc_offset).0);
        }
        else
        {
            self.registers.write(Register::R_PC, (instruction >> 6) & 0x7);
        }
    }

    fn ld(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let pc_offset = sign_extend(instruction & 0x1FF, 9);

        let r_pc = self.registers.read(Register::R_PC);
        let read = self.mem_read(r_pc.overflowing_add(pc_offset).0);

        self.registers.write(r0, read);

        self.update_flags(r0_clone);
    }

    fn ldr(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0b111;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let r1 = (instruction >> 6) & 0b111;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        let offset = sign_extend(instruction & 0b11_1111, 6);
        let read = self.mem_read(r1.overflowing_add(offset).0);

        self.registers.write(r0, read);

        self.update_flags(r0_clone);
    }

    fn lea(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let pc_offset = sign_extend(instruction & 0x1FF, 9);
        let r_pc = self.registers.read(Register::R_PC);

        self.registers.write(r0, r_pc.overflowing_add(pc_offset).0);

        self.update_flags(r0_clone);
    }

    fn st(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0 = Register::new(r0);
        let r0 = self.registers.read(r0);

        let pc_offset = sign_extend(instruction & 0x1FF, 9);
        let r_pc = self.registers.read(Register::R_PC);

        self.mem_write(r_pc.overflowing_add(pc_offset).0, r0)
    }

    fn sti(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0 = Register::new(r0);
        let r0 = self.registers.read(r0);

        let pc_offset = sign_extend(instruction & 0x1FF, 9);
        let r_pc = self.registers.read(Register::R_PC);
        let read = self.mem_read(r_pc.overflowing_add(pc_offset).0);

        self.mem_write(read, r0);
    }

    fn str(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0 = Register::new(r0);
        let r0 = self.registers.read(r0);

        let r1 = (instruction >> 6) & 0b111;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        let offset = sign_extend(instruction & 0x3F, 6);

        self.mem_write(r1.overflowing_add(offset).0, r0);
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

fn swap_endian(original: [u8; 2]) -> u16 {
    original[1] as u16 + ((original[0] as u16) << 8)
}

fn sign_extend(x: u16, bit_count: u16) -> u16 {
    let mut y = x;
    if ((y >> (bit_count - 1)) & 1) > 0 {
        y |= 0xFFFF << bit_count;
    }
    y
}