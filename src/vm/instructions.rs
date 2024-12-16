use crate::vm::registers::Register;
use crate::vm::utils::sign_extend;
use crate::vm::Vm;

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

pub fn add(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;

    let r1 = (instruction >> 6) & 0x7;
    let r1 = vm.registers.read_r(r1);

    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag != 0 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.registers.write_r(r0, r1.overflowing_add(imm5).0);
    }
    else
    {
        let r2: u16 = instruction & 0x7;
        let r2 = vm.registers.read_r(r2);

        vm.registers.write_r(r0, r1.overflowing_add(r2).0);
    }

    vm.update_flags_r(r0);
}

pub fn ldi(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let pc = vm.registers.read(Register::R_PC);

    let value = vm.mem_read(pc.overflowing_add(pc_offset).0);
    let read = vm.mem_read(value);

    vm.registers.write_r(r0, read);

    vm.update_flags_r(r0);

}

pub fn and(vm: &mut Vm, instruction: u16)
{
    let r0 = (instruction >> 9) & 0x7;

    let r1 = (instruction >> 6) & 0x7;
    let r1 = vm.registers.read_r(r1);

    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag > 0 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.registers.write_r(r0, r1 & imm5);
    }
    else
    {
        let r2: u16 = instruction & 0x7;
        let r2 = vm.registers.read_r(r2);

        vm.registers.write_r(r0, r1 & r2);
    }

    vm.update_flags_r(r0);
}

pub fn not(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;

    let r1 = (instruction >> 6) & 0x7;
    let r1 = vm.registers.read_r(r1);

    vm.registers.write_r(r0, !r1);
    vm.update_flags_r(r0);
}

pub fn branch(vm: &mut Vm, instruction: u16) {
    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let cond_flag = (instruction >> 9) & 0x7;

    let r_cond = vm.registers.read(Register::R_COND);
    let pc = vm.registers.read(Register::R_PC);

    if cond_flag & r_cond > 0 {
        vm.registers.write(Register::R_PC, pc.overflowing_add(pc_offset).0);
    }
}

pub fn jmp(vm: &mut Vm, instruction: u16) {
    let r1 = (instruction >> 6) & 0x7;
    let r1 = vm.registers.read_r(r1);

    vm.registers.write(Register::R_PC, r1);
}

pub  fn jsr(vm: &mut Vm, instruction: u16) {
    let long_flag = (instruction >> 11) & 1;
    let pc = vm.registers.read(Register::R_PC);

    vm.registers.write(Register::R_R7, pc);

    if long_flag > 0 {
        let long_pc_offset = sign_extend(instruction & 0x7FF, 11);
        vm.registers.write(Register::R_PC, pc.overflowing_add(long_pc_offset).0);
    }
    else
    {
        vm.registers.write(Register::R_PC, (instruction >> 6) & 0x7);
    }
}

pub fn ld(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let r_pc = vm.registers.read(Register::R_PC);
    let read = vm.mem_read(r_pc.overflowing_add(pc_offset).0);

    vm.registers.write_r(r0, read);

    vm.update_flags_r(r0);
}

pub fn ldr(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0b111;

    let r1 = (instruction >> 6) & 0b111;
    let r1 = vm.registers.read_r(r1);

    let offset = sign_extend(instruction & 0b11_1111, 6);
    let read = vm.mem_read(r1.overflowing_add(offset).0);

    vm.registers.write_r(r0, read);

    vm.update_flags_r(r0);
}

pub fn lea(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let r_pc = vm.registers.read(Register::R_PC);

    vm.registers.write_r(r0, r_pc.overflowing_add(pc_offset).0);

    vm.update_flags_r(r0);
}

pub fn st(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;
    let r0 = vm.registers.read_r(r0);

    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let r_pc = vm.registers.read(Register::R_PC);

    vm.mem_write(r_pc.overflowing_add(pc_offset).0, r0)
}

pub fn sti(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;
    let r0 = vm.registers.read_r(r0);

    let pc_offset = sign_extend(instruction & 0x1FF, 9);
    let r_pc = vm.registers.read(Register::R_PC);
    let read = vm.mem_read(r_pc.overflowing_add(pc_offset).0);

    vm.mem_write(read, r0);
}

pub fn str(vm: &mut Vm, instruction: u16) {
    let r0 = (instruction >> 9) & 0x7;
    let r0 = vm.registers.read_r(r0);

    let r1 = (instruction >> 6) & 0b111;
    let r1 = vm.registers.read_r(r1);

    let offset = sign_extend(instruction & 0x3F, 6);

    vm.mem_write(r1.overflowing_add(offset).0, r0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Vm, registers::{Register, Registers}, MEMORY_MAX};

    fn create_vm() -> Vm {
        Vm {
            memory: [0; MEMORY_MAX],
            registers: Registers::new(),
            active: true,
        }
    }

    #[test]
    fn test_add() {
        let mut vm = create_vm();

        let instruction: u16 = (0b000000 << 9) | (0b00001 << 6) | (1 << 5) | 5;

        vm.registers.write(Register::R_R1, 10);

        add(&mut vm, instruction);

        assert_eq!(vm.registers.read(Register::R_R0), 15);
    }

    #[test]
    fn test_jmp() {
        let mut vm = create_vm();

        let instruction: u16 = (0b000011 << 9) | (0b00001 << 6);

        vm.registers.write(Register::R_R1, 0x4000);

        jmp(&mut vm, instruction);

        assert_eq!(vm.registers.read(Register::R_PC), 0x4000);
    }

}
