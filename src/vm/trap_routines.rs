use std::io::{Read, stdin, stdout, Write};
use crate::vm::registers::Register;
use crate::vm::Vm;

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

pub fn halt(vm: &mut Vm) {
    println!("HALT");
    vm.active = false;
}

pub fn trap_put(vm: &mut Vm) {
    let mut addr = vm.registers.read(Register::R_R0);
    let mut character = vm.memory[addr as usize];

    while character > 0 {
        print!("{}", (character & 0b1111_1111) as u8 as char);
        addr = addr.overflowing_add(1).0;
        character = vm.memory[addr as usize];
    }
    stdout().flush().expect("should not fail");
}

pub fn get_char(vm: &mut Vm) {
    let input: u16 = stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u16)
        .expect("should not fail");

    vm.registers.write(Register::R_R0, input & 0b1111_1111)
}

pub fn trap_out(vm: &mut Vm) {
    print!("{}", vm.registers.read(Register::R_R0) as u8 as char);
    stdout().flush().expect("should not fail!");
}

pub fn trap_in(vm: &mut Vm) {
    print!("Input: ");
    stdout().flush().expect("Should not fail");

    get_char(vm);
}

pub fn trap_putsp(vm: &mut Vm) {
    let mut addr = vm.registers.read(Register::R_R0);
    let mut character = vm.memory[addr as usize];

    while character > 0 {
        print!("{}", (character & 0b1111_1111) as u8 as char);
        let second_part = (character >> 8) & 0b1111_1111;
        if second_part == 0 {
            break;
        }
        print!("{}", second_part as u8 as char);
        addr = addr.overflowing_add(1).0;
        character = vm.memory[addr as usize];
    }
    stdout().flush().expect("should not fail");
}
