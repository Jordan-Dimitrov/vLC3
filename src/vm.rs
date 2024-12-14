mod registers;
mod instructions;
mod trap_routines;

use std::io;
use std::io::{Read, Write};
use std::process::exit;
use crate::vm::registers::{Register, Registers};
use crate::vm::instructions::{*};
use crate::vm::registers::Flags::{FL_NEG, FL_POS, FL_ZRO};
use crate::vm::trap_routines::{*};
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
                OpCodes::OP_LD => self.ld(instruction),
                OpCodes::OP_LDI => self.ldi(instruction),
                OpCodes::OP_LDR => self.ldr(instruction),
                OpCodes::OP_LEA => self.lea(instruction),
                OpCodes::OP_ST => self.st(instruction),
                OpCodes::OP_STI => self.sti(instruction),
                OpCodes::OP_STR => self.str(instruction),
                OpCodes::OP_TRAP => {
                    let r_pc = self.registers.read(Register::R_PC);
                    self.registers.write(Register::R_R7, r_pc);
                    let trap = TrapRoutine::new(instruction & 0xFF);

                    match trap {
                        TrapRoutine::TRAP_GETC => self.trap_getc(),
                        TrapRoutine::TRAP_OUT => self.trap_out(),
                        TrapRoutine::TRAP_PUTS => self.trap_put(),
                        TrapRoutine::TRAP_IN => self.trap_in(),
                        TrapRoutine::TRAP_PUTSP => self.trap_putsp(),
                        TrapRoutine::TRAP_HALT => {}
                    }
                },
                OpCodes::OP_RES | OpCodes::OP_RTI => break
            }
        }
    }

    fn trap_put(&mut self) {
        let mut c = self.registers.read(Register::R_R0);

        while self.memory[c as usize] != 0 {
            let character = self.memory[c as usize] as u8 as char;
            print!("{}", character);
            c+=1;
        }

        io::stdout().flush().unwrap();
    }

    fn get_char() -> u8 {
        io::stdin().bytes().next().and_then(|result| result.ok()).unwrap()
    }

    fn trap_getc(&mut self) {
        self.registers.write(Register::R_R0, Self::get_char() as u16);
        io::stdout().flush().unwrap();
    }

    fn trap_out(&mut self) {
        let r0 = self.registers.read(Register::R_R0);
        print!("{}", r0 as u8 as char);

        io::stdout().flush().unwrap();
    }

    fn trap_in(&mut self) {
        println!("Input: ");
        let c = Self::get_char();
        print!("{}", c);
        io::stdout().flush().unwrap();

        self.registers.write(Register::R_R0, c as u16);
        self.update_flags(Register::R_R0);
    }

    fn trap_putsp(&mut self) {
        let mut c = self.registers.read(Register::R_R0) as usize;

        while self.memory[c] != 0 {
            let char1 = (self.memory[c] & 0xFF) as u8 as char;
            print!("{}", char1);

            let char2 = (self.memory[c] >> 8) as u8 as char;
            if char2 != '\0' {
                print!("{}", char2);
            }

            c+=1;
        }

        io::stdout().flush().unwrap();
    }

    fn halt() {
        println!("HALT");
        io::stdout().flush().unwrap();

        exit(0);
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

    fn ld(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);

        let r_pc = self.registers.read(Register::R_PC);

        self.registers.write(r0, Self::mem_read(r_pc + pc_offset));

        self.update_flags(r0_clone);
    }

    fn ldr(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let r1 = (instruction >> 6) & 0x7;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        let offset = Self::sign_extend(instruction & 0x3F, 6);

        self.registers.write(r0, Self::mem_read(r1 + offset));

        self.update_flags(r0_clone);
    }

    fn lea(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0_clone = Register::new(r0.clone());
        let r0 = Register::new(r0);

        let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);
        let r_pc = self.registers.read(Register::R_PC);

        self.registers.write(r0, r_pc + pc_offset);

        self.update_flags(r0_clone);
    }

    fn st(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0 = Register::new(r0);
        let r0 = self.registers.read(r0);

        let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);
        let r_pc = self.registers.read(Register::R_PC);

        Self::mem_write(r_pc + pc_offset, r0)
    }

    fn sti(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0 = Register::new(r0);
        let r0 = self.registers.read(r0);

        let pc_offset = Self::sign_extend(instruction & 0x1FF, 9);
        let r_pc = self.registers.read(Register::R_PC);

        Self::mem_write(Self::mem_read(r_pc + pc_offset), r0);
    }

    fn str(&mut self, instruction: u16) {
        let r0 = (instruction >> 9) & 0x7;
        let r0 = Register::new(r0);
        let r0 = self.registers.read(r0);

        let r1 = (instruction >> 6) & 0x7;
        let r1 = Register::new(r1);
        let r1 = self.registers.read(r1);

        let offset = Self::sign_extend(instruction & 0x3F, 6);

        Self::mem_write(r1 + offset, r0);
    }


    fn mem_read(address: u16) -> u16{
        1
    }

    fn mem_write(address : u16, value: u16) {

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
