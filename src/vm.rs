mod registers;
mod instructions;

use std::process::exit;
use crate::vm::registers::{Register, Registers};
use crate::vm::instructions::{*};
use crate::vm::registers::Flags::{FL_NEG, FL_POS, FL_ZRO};

const MEMORY_MAX: usize = 65536;

pub struct Vm {
    memory: [u16; MEMORY_MAX],
    registers: Registers
}

impl Vm {
    pub fn new() -> Self {
        Self { memory: [0; MEMORY_MAX], registers: Registers::new()}
    }

    pub fn start(&mut self, args: Vec<String>) {
        Self::load_arguements(args);

        self.registers.write(Register::R_COND, FL_ZRO as u16);

        loop {
            let mut address = self.registers.read(Register::R_PC);
            let instruction = Self::mem_read(address);

            address+=1;
            self.registers.write(Register::R_PC, address);

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
                OpCodes::OP_LD => self.load(instruction),
                OpCodes::OP_LDI => self.ldi(instruction),
                OpCodes::OP_LDR => (),
                OpCodes::OP_LEA => (),
                OpCodes::OP_ST => (),
                OpCodes::OP_STI => (),
                OpCodes::OP_STR => (),
                OpCodes::OP_TRAP => (),
                OpCodes::OP_RES | OpCodes::OP_RTI => break
            }
        }
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
            let imm5 = Self::sign_extend(instruction & 0x1F, 5);
            self.registers.write(r0, r1 + imm5);
        }
        else
        {
            let r2: u16 = instruction & 0x7;
            let r2 = Register::new(r2);
            let r2 = self.registers.read(r2);

            self.registers.write(r0, r1 + r2);
        }

        self.update_flags(r0_clone);
    }

    fn ldi(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);

        let r_pc = self.registers.read(Register::R_PC);

        self.registers.write(r0, Self::mem_read(Self::mem_read(r_pc + pc_offset)));

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

        if imm_flag != 0 {
            let imm5 = Self::sign_extend(instruction & 0x1F, 5);
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
        let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);
        let cond_flag = (instruction >> 9) & 0x7;
        let cond_flag_clone = cond_flag;

        let r_cond = self.registers.read(Register::R_COND);

        if cond_flag & r_cond == 1 {
            self.registers.write(Register::R_PC, cond_flag_clone + pc_offset);
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
        let r_pc = self.registers.read(Register::R_PC);

        self.registers.write(Register::R_R7, r_pc);

        if long_flag != 0 {
            let long_pc_offset = Self::sign_extend(instruction & 0x7FF, 11);
            self.registers.write(Register::R_PC, r_pc + long_pc_offset);
        }
        else
        {
            let r1 = (instruction >> 6) & 0x7;
            let r1 = Register::new(r1);
            let r1 = self.registers.read(r1);

            self.registers.write(Register::R_PC, r1);
        }
    }

    fn load(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);

        let r_pc = self.registers.read(Register::R_PC);

        self.registers.write(r0, Self::mem_read(r_pc + pc_offset));

        self.update_flags(r0_clone);
    }

    fn mem_read(address: u16) -> u16{
        1
    }

    pub fn test() {
        println!("{}", FL_ZRO as u16);
        println!("{}", FL_NEG as u16);
        println!("{}", FL_POS as u16);
    }


    fn load_arguements(args: Vec<String>) {
        if args.len() < 2 {
            println!("lc3 [image file1]...");
            exit(2);
        }

        for i in args {
            if Self::read_image(&i) {
                println!("failed to load image: {}", i);
                exit(1);
            }
        }
    }

    fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
        if (x >> (bit_count - 1)) & 1 != 0 {
            x |= 0xFFFF << bit_count;
        }
        x
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

    fn read_image(arg: &String) -> bool{
        true
    }
}
