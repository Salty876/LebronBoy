use std::ptr::addr_of;

use bitflags::bitflags;

// const ZERO_FLAG_BYTE_POSITION:u8 = 7;
// const SUBSTRACT_FLAG_BYTE_POSITION:u8 = 6;
// const HALF_CARRY_FLAG_BYTE_POSITION:u8 = 5;
// const CARRY_FLAG_BYTE_POSITION:u8 = 4;


// Convert flags from u8 and forth so we can manipulate that f flag
// impl std::convert::From<Flags> for u8 {
//     fn from(flag: Flags) -> u8 {
//         (if flag.z_flag      { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
//         (if flag.n_flag      { 1 } else { 0 }) << SUBSTRACT_FLAG_BYTE_POSITION |
//         (if flag.h_flag      { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
//         (if flag.c_flag      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
//     }
// }

// impl std::convert::From<u8> for Flags {
//     fn from(byte: u8) -> Self {
//         let z_flag = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
//         let n_flag = ((byte >> SUBSTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
//         let h_flag = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
//         let c_flag = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

//          return Flags {
//             z_flag,
//             n_flag,
//             h_flag,
//             c_flag
//         };
//     }
// }





pub struct Cpu {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    pub is_halted: bool

}

impl Cpu {
    pub fn new() -> Cpu{
        let registers:Registers =  Registers::new(); 

         let mem:MemoryBus = MemoryBus { memory: [0; 0xFFFF] };

         Cpu {
            registers: registers,
            pc: 0x0000,
            sp: 0,
            bus: mem,
            is_halted: false
         }
    }

    pub fn execute (&mut self, instruction: Instruction) -> u16 {
        if self.is_halted{
            return self.pc;
        }
        match instruction{
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a_reg;
                        let new_value = self.add(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)

                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b_reg;
                        let new_value = self.add(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)

                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c_reg;
                        let new_value = self.add(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)

                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d_reg;
                        let new_value = self.add(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)

                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e_reg;
                        let new_value = self.add(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)

                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h_reg;
                        let new_value = self.add(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)

                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l_reg;
                        let new_value = self.add(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    }
                     _ => {/*more rargets*/ self.pc}
                }
                
            }

            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a_reg;
                        let new_value = self.sub(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::B => {
                        let value = self.registers.b_reg;
                        let new_value = self.sub(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::C => {
                        let value = self.registers.c_reg;
                        let new_value = self.sub(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => {
                        let value = self.registers.d_reg;
                        let new_value = self.sub(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::E => {
                        let value = self.registers.e_reg;
                        let new_value = self.sub(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::H => {
                        let value = self.registers.h_reg;
                        let new_value = self.sub(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::L => {
                        let value = self.registers.l_reg;
                        let new_value = self.sub(value);
                        self.registers.a_reg = new_value;
                        self.pc.wrapping_add(1)
                    }
                  
                    
                }
            }

            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.get_z(),
                    JumpTest::NotCarry => !self.registers.get_c(),
                    JumpTest::Zero => self.registers.get_z(),
                    JumpTest::Carry => self.registers.get_c(),
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }

            Instruction::LD(LoadType) => {
               match LoadType {
                   LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a_reg,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        _ => {panic!("IMPLEMENT OTHER SOURCES FN")}
                    };

                    match target {
                        LoadByteTarget::A => self.registers.a_reg = source_value,
                        LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                        _ => {  panic!("FINSIH THE REST OF TATGETS FN") }
                    };

                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _                  => self.pc.wrapping_add(1),
                    }
                   }

                   _ => {   panic!("ADD THE REST OF LOAD TYPES")    }
               }
            }

            Instruction::PUSH(target) => {
                let value = match target {
                    StackTargets::BC => self.registers.get_bc(),
                    _ => {  panic!("ALL TARGETS NOT DONE FN")   }
                };
                self.push(value);
                return self.pc.wrapping_add(1);
            }

            Instruction::POP(target) => {
                let result = self.pop();

                match target {
                    StackTargets::BC => self.registers.set_bc(result),
                    _ => {  panic!("REST OF TARGETS")  }
                };
                return self.pc.wrapping_add(1);
            }

            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.get_z(),
                    _ => {/*DO the other condisions */ false}

                };
                self.call(jump_condition)
            }

            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.get_z(),
                    _ => {/*Other condiitons */ false}
                };
                self.return_(jump_condition)
            }

            Instruction::NOP => {
                self.pc.wrapping_add(1)
            }

            Instruction::HALT => {
                self.is_halted = true;
                return self.pc;
            }

            _ => {/*more instructions*/ self.pc}
        }
    }

    fn add(&mut self, value: u8) -> u8{

        let (new_value, did_overflow) = self.registers.a_reg.overflowing_add(value);
        // self.registers.f_reg.z_flag = new_value == 0;
        // self.registers.f_reg.n_flag = false;
        // self.registers.f_reg.c_flag = did_overflow;
        // self.registers.f_reg.h_flag = (self.registers.a_reg & 0xF) + (value & 0xF) > 0xF;
        self.registers.set_z(new_value == 0);
        self.registers.set_n(false);
        self.registers.set_c(did_overflow);
        self.registers.set_h((self.registers.a_reg & 0xF) + (value & 0xF) > 0xF);
        new_value
    }

    fn sub(&mut self, value: u8) -> u8{

        let (new_value, did_overflow) = self.registers.a_reg.overflowing_sub(value);
        // self.registers.f_reg.z_flag = new_value == 0;
        // self.registers.f_reg.n_flag = false;
        // self.registers.f_reg.c_flag = did_overflow;
        // self.registers.f_reg.h_flag = (self.registers.a_reg & 0xF) + (value & 0xF) > 0xF;
        self.registers.set_z(new_value == 0);
        self.registers.set_n(true);
        self.registers.set_c(did_overflow);
        self.registers.set_h((self.registers.a_reg & 0xF) + (value & 0xF) > 0xF);
        new_value
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump{
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        }else{
            self.pc.wrapping_add(3)
        }
    }

    pub fn step(&mut self) {

        let mut instruction_byte = self.bus.read_byte(self.pc);
        println!("{:#0x}", instruction_byte);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        
        let next_pc: u16= if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        }else{
            panic!("Unknown instruction found for 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn read_next_byte(&self) -> u8{
        self.bus.memory[(self.pc + 1) as usize]
    }

    fn push(&mut self, value: u16) {
        self.pc = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF00) as u8);
    }

    fn pop(&mut self) -> u16{
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
        
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            return next_pc;
        }else{
            return next_pc;
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump{
            return self.pop();
        }else{
            return self.pc.wrapping_add(1);
        }
    }

}

enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTargets),
    POP(StackTargets),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    HALT
}

impl Instruction{
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed{
            Instruction::from_byte_prefixed(byte)
        }else {
            Instruction::from_byte_not_prefixed(byte)
        }
       
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            _ => None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
         match byte {
            0x00 => Some(Instruction::NOP),
            _ => None
        }
    }
}
enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF]
 }

impl MemoryBus{
    fn read_byte(&self, adress: u16) -> u8{
        return self.memory[adress as usize];
    }

    fn write_byte(&self, address: u16, byte: u8) {
        // empty
    }
}

enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

enum StackTargets{
    AF, BC, DE, HL
}

pub struct Registers{
    a_reg: u8,
    b_reg: u8,
    c_reg: u8,
    d_reg: u8,
    e_reg: u8,
    f_reg: Flags,
    h_reg: u8,
    l_reg: u8
}

impl Registers {
    fn new() -> Self{
        Registers { 
            a_reg: 0,
            f_reg: Flags::empty(),
            b_reg: 0,
            c_reg: 0,
            d_reg: 0,
            e_reg: 0,
            h_reg: 0,
            l_reg: 0
         }
    }

    fn get_af(&self) -> u16 {
        return (self.a_reg as u16) << 8 | self.f_reg.bits() as u16;
    }

    fn set_af(&mut self, value: u16) {
        self.a_reg = ((value & 0xFF00) >> 8) as u8;
        self.f_reg = Flags::from_bits_truncate(value as u8);
    }

    fn get_bc(&self) -> u16 {
        return (self.b_reg as u16) << 8 | self.c_reg as u16;
    }

    fn set_bc(&mut self, value: u16) {
        self.b_reg = ((value & 0xFF00) >> 8) as u8;
        self.c_reg = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        return (self.d_reg as u16) << 8 | self.e_reg as u16;
    }

    fn set_de(&mut self, value: u16) {
        self.d_reg = ((value & 0xFF00) >> 8) as u8;
        self.e_reg = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        return (self.h_reg as u16) << 8 | self.l_reg as u16;
    }

    fn set_hl(&mut self, value: u16) {
        self.h_reg = ((value & 0xFF00) >> 8) as u8;
        self.l_reg = (value & 0xFF) as u8;
    }

    // Getting flags
    fn get_z(&self) -> bool{
        self.f_reg.contains(Flags::z_flag)
    }

    fn get_n(&self) -> bool{
        self.f_reg.contains(Flags::n_flag)
    }

    fn get_h(&self) -> bool{
        self.f_reg.contains(Flags::h_flag)
    }

    fn get_c(&self) -> bool{
        self.f_reg.contains(Flags::c_flag)
    }

    // Setting flags
    fn set_z(&mut self, zf: bool){
        self.f_reg.set(Flags::z_flag, zf);
    }

    fn set_n(&mut self, nf: bool){
        self.f_reg.set(Flags::n_flag, nf);
    }

    fn set_h(&mut self, hf: bool){
        self.f_reg.set(Flags::h_flag, hf);
    }

    fn set_c(&mut self, cf: bool){
        self.f_reg.set(Flags::c_flag, cf);
    }
}

// struct Flags {
//      z_flag: bool, //ZERO FLAG; set to 1 if current op results in 0 or two values match a CMP operation
//      n_flag: bool, //SUBSTRACTION FLAG; set to 1 if substraction happens
//      h_flag: bool, //HALF CARRY FLAG; set to 1 if a carry occured from the lower nibble in the last operation
//      c_flag: bool, //CARRY FLAG; set to 1 if a carry occured in the last operation or if A is the smaller value on CP instruction
// }

bitflags! (
    pub struct Flags: u8{
        const z_flag = 0b_1000_0000;
        const n_flag = 0b_0100_0000;
        const h_flag = 0b_0010_0000;
        const c_flag = 0b_0001_0000;

    }


);

 


