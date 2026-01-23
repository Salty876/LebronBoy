use std::io::StderrLock;

use crate::{bus::Bus, cpu::{self, execute::execute}};

pub mod registers;
pub mod instructions;
mod execute;

use registers::Registers;
use instructions::Instruction;

pub struct Cpu {
    pub regs: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: Bus,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            pc: 0x0000,
            sp: 0xFFFE,
            bus: Bus::new(),
            halted: false,
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        let mut opcode = self.bus.read_byte(self.pc);
        let prefixed = opcode == 0xCB;
        if prefixed {
            opcode = self.bus.read_byte(self.pc.wrapping_add(1));
        }

        let instr = Instruction::decode(opcode, prefixed)
            .unwrap_or_else(|| panic!("Unknown opcode: 0x{:02X} (prefixed={})", opcode, prefixed));

        let next_pc = execute(self, instr, prefixed);
        self.pc = next_pc;
    }

    #[inline]
    pub fn next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc.wrapping_add(1))
    }

    #[inline]
    pub fn next_word(&self) -> u16 {
        self.bus.read_word(self.pc.wrapping_add(1))
    }

    #[inline]
    pub fn push_word(&mut self, value: u16) {
        // stack grows down
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value >> 8) as u8); // hi
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8); // lo
    }

    #[inline]
    pub fn pop_word(&mut self) -> u16 {
        let lo = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let hi = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (hi << 8) | lo
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        // temporary simple mapping: copy to 0x0000...
        let len = rom.len().min(0x8000); // ROM0+ROMX basic
        self.bus.memory[..len].copy_from_slice(&rom[..len]);
        self.pc = 0x0100; // skip boot (or set 0x0000 if using boot rom)
    }

    pub fn run_steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    pub fn add(&mut self, value: u8) -> u8{

        let (new_value, did_overflow) = self.regs.a_reg.overflowing_add(value);
        // self.registers.f_reg.z_flag = new_value == 0;
        // self.registers.f_reg.n_flag = false;
        // self.registers.f_reg.c_flag = did_overflow;
        // self.registers.f_reg.h_flag = (self.registers.a_reg & 0xF) + (value & 0xF) > 0xF;
        self.regs.set_z(new_value == 0);
        self.regs.set_n(false);
        self.regs.set_carry(did_overflow);
        self.regs.set_hc((self.regs.a_reg & 0xF) + (value & 0xF) > 0xF);
        new_value
    }

    pub fn add_hl_rr(&mut self, register: u16) -> u16{
        let hl = self.regs.get_hl();

        // u32 helps test cleanly
        let sum = (hl as u32) + (register as u32);
        let res = (sum & 0xFFFF) as u16;


        // Set flags - ADD HL,rr only affects N, H, C (not Z)
        // Z flag remains unchanged
        self.regs.set_n(false);
        self.regs.set_hc(((hl & 0x0FFF) + (register & 0x0FFF)) > 0x0FFF);
        self.regs.set_carry(sum > 0xFFFF);

        return res;
    }

    pub fn sub(&mut self, value: u8) -> u8{

        let (new_value, did_overflow) = self.regs.a_reg.overflowing_sub(value);
        // self.registers.f_reg.z_flag = new_value == 0;
        // self.registers.f_reg.n_flag = true;
        // self.registers.f_reg.c_flag = did_overflow;
        // self.registers.f_reg.h_flag = (self.registers.a_reg & 0xF) < (value & 0xF);
        self.regs.set_z(new_value == 0);
        self.regs.set_n(true);
        self.regs.set_carry(did_overflow);
        self.regs.set_hc((self.regs.a_reg & 0xF) < (value & 0xF));
        new_value
    }
}


#[cfg(test)]
mod tests {
    use super::Cpu;

    #[test]
    fn push_pop_word_roundtrip_and_endianness() {
        let mut cpu = Cpu::new();
        cpu.sp = 0xFFFE;
        cpu.pc = 0x1234;

        cpu.push_word(0xBEEF);

        // PC must not change due to stack operations
        assert_eq!(cpu.pc, 0x1234);

        // Stack grows down by 2
        assert_eq!(cpu.sp, 0xFFFC);

        // In our implementation: [SP]=lo, [SP+1]=hi after push_word?
        // NOTE: this depends on how you implemented push_word.
        // If you used the common pattern (decrement, write hi, decrement, write lo),
        // then at final SP: mem[SP]=lo, mem[SP+1]=hi.
        assert_eq!(cpu.bus.read_byte(cpu.sp), 0xEF); // lo
        assert_eq!(cpu.bus.read_byte(cpu.sp + 1), 0xBE); // hi

        let v = cpu.pop_word();
        assert_eq!(v, 0xBEEF);
        assert_eq!(cpu.sp, 0xFFFE);
    }

    #[test]
    fn pop_word_reads_in_correct_order() {
        let mut cpu = Cpu::new();
        cpu.sp = 0xFFFC;
        cpu.bus.write_byte(0xFFFC, 0x34); // lo
        cpu.bus.write_byte(0xFFFD, 0x12); // hi

        let v = cpu.pop_word();
        assert_eq!(v, 0x1234);
        assert_eq!(cpu.sp, 0xFFFE);
    }
}



// use crate::{bus::MemoryBus, cpu::{instructions::Instruction, registers::Registers}};
// use crate::cpu::execute::execute

// pub mod registers;
// pub mod instructions;
// pub mod execute;

// pub struct Cpu {
//     pub registers: Registers,
//     pub pc: u16,
//     pub sp: u16,
//     pub bus: MemoryBus,
//     pub is_halted: bool

// }

// impl Cpu {
//     pub fn new() -> Cpu{
//         let registers:Registers =  Registers::new(); 

//          let mem:MemoryBus = MemoryBus { memory: [0; 0xFFFF] };

//          Cpu {
//             registers: registers,
//             pc: 0x0000,
//             sp: 0,
//             bus: mem,
//             is_halted: false
//          }
//     }


//     fn add(&mut self, value: u8) -> u8{

//         let (new_value, did_overflow) = self.registers.a_reg.overflowing_add(value);
//         // self.registers.f_reg.z_flag = new_value == 0;
//         // self.registers.f_reg.n_flag = false;
//         // self.registers.f_reg.c_flag = did_overflow;
//         // self.registers.f_reg.h_flag = (self.registers.a_reg & 0xF) + (value & 0xF) > 0xF;
//         self.registers.set_z(new_value == 0);
//         self.registers.set_n(false);
//         self.registers.set_carry(did_overflow);
//         self.registers.set_hc((self.registers.a_reg & 0xF) + (value & 0xF) > 0xF);
//         new_value
//     }

//     // fn adc(&mut self, value: u8) -> u8 {
//     //     let (new_value, did_overflow) = self.registers.a_reg.overflowing_add(value.overflowing_add(self.registers.get_c())[0]);


//     //     self.registers.set_z(new_value)
//     // }


//     fn sub(&mut self, value: u8) -> u8{

//         let (new_value, did_overflow) = self.registers.a_reg.overflowing_sub(value);
//         // self.registers.f_reg.z_flag = new_value == 0;
//         // self.registers.f_reg.n_flag = false;
//         // self.registers.f_reg.c_flag = did_overflow;
//         // self.registers.f_reg.h_flag = (self.registers.a_reg & 0xF) + (value & 0xF) > 0xF;
//         self.registers.set_z(new_value == 0);
//         self.registers.set_n(true);
//         self.registers.set_carry(did_overflow);
//         self.registers.set_hc((self.registers.a_reg & 0xF) + (value & 0xF) > 0xF);
//         new_value
//     }

//     fn bit_and(&mut self, value: u8) -> u8{
//         let new_value = self.registers.a_reg & value;
        
//         self.registers.set_z(new_value == 0);
//         self.registers.set_n(false);
//         self.registers.set_hc(true);
//         self.registers.set_carry(false);

//         return new_value;
//     }

//     fn bit_or(&mut self, value: u8) -> u8{
//         let new_value = self.registers.a_reg | value;
        
//         self.registers.set_z(new_value == 0);
//         self.registers.set_n(false);
//         self.registers.set_hc(true);
//         self.registers.set_carry(false);

//         return new_value;
//     }

//     fn bit_xor(&mut self, value: u8) -> u8{
//         let new_value = self.registers.a_reg ^ value;
        
//         self.registers.set_z(new_value == 0);
//         self.registers.set_n(false);
//         self.registers.set_hc(true);
//         self.registers.set_carry(false);

//         return new_value;
//     }

//     fn jump(&self, should_jump: bool) -> u16 {
//         if should_jump{
//             let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
//             let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
//             (most_significant_byte << 8) | least_significant_byte
//         }else{
//             self.pc.wrapping_add(3)
//         }
//     }

//     pub fn step(&mut self) {

//         let mut instruction_byte = self.bus.read_byte(self.pc);
//         println!("{:#0x}", instruction_byte);
//         let prefixed = instruction_byte == 0xCB;
//         if prefixed {
//             instruction_byte = self.bus.read_byte(self.pc + 1);
//         }
        
//         let next_pc: u16= if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
//             execute(&mut self, instruction)
//         }else{
//             panic!("Unknown instruction found for 0x{:x}", instruction_byte);
//         };

//         self.pc = next_pc;
//     }

//     fn read_next_byte(&self) -> u8{
//         self.bus.memory[(self.pc + 1) as usize]
//     }

//     fn push(&mut self, value: u16) {
//         self.pc = self.sp.wrapping_sub(1);
//         self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

//         self.sp = self.sp.wrapping_sub(1);
//         self.bus.write_byte(self.sp, (value & 0xFF00) as u8);
//     }

//     fn pop(&mut self) -> u16{
//         let lsb = self.bus.read_byte(self.sp) as u16;
//         self.sp = self.sp.wrapping_add(1);

//         let msb = self.bus.read_byte(self.sp) as u16;
//         self.sp = self.sp.wrapping_add(1);

//         (msb << 8) | lsb
        
//     }

//     fn call(&mut self, should_jump: bool) -> u16 {
//         let next_pc = self.pc.wrapping_add(3);
//         if should_jump {
//             self.push(next_pc);
//             return next_pc;
//         }else{
//             return next_pc;
//         }
//     }

//     fn return_(&mut self, should_jump: bool) -> u16 {
//         if should_jump{
//             return self.pop();
//         }else{
//             return self.pc.wrapping_add(1);
//         }
//     }

// }
