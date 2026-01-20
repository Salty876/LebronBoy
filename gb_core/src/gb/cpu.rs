use std::{intrinsics::wrapping_add, ptr::addr_of, result};

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

            Instruction::INC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let (value, did_overflow) = self.registers.a_reg.overflowing_add(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)

                    },
                    ArithmeticTarget::B => {
                        let (value, did_overflow) = self.registers.b_reg.overflowing_add(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)


                    },
                    ArithmeticTarget::C => {
                        let (value, did_overflow) = self.registers.c_reg.overflowing_add(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)


                    },
                    ArithmeticTarget::D => {
                        let (value, did_overflow) = self.registers.d_reg.overflowing_add(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)


                    },
                    ArithmeticTarget::E => {
                        let (value, did_overflow) = self.registers.e_reg.overflowing_add(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)


                    },
                    ArithmeticTarget::H => {
                        let (value, did_overflow) = self.registers.h_reg.overflowing_add(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)


                    },
                    ArithmeticTarget::L => {
                        let (value, did_overflow) = self.registers.l_reg.overflowing_add(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)


                    }

                    _ => {self.pc}
                }
            }

            Instruction::INC_HL => {
                let (value, did_overflow) = self.registers.get_hl().overflowing_add(1);
                self.registers.set_hl(value);
                self.registers.set_z(self.registers.a_reg == 0);
                self.registers.set_n(false);
                self.registers.set_h(did_overflow);
                self.pc.wrapping_add(1)
            }

            Instruction::INC_R16(target) => {
                match target {
                    BigRegisterTarget::AF => {
                        let og_value = self.registers.get_af();
                        let new_value = og_value.wrapping_add(1);
                        self.registers.set_af(new_value);
                        self.pc.wrapping_add(1)
                    },
                    BigRegisterTarget::BC => {
                        let og_value = self.registers.get_bc();
                        let new_value = og_value.wrapping_add(1);
                        self.registers.set_bc(new_value);
                        self.pc.wrapping_add(1)
                    },BigRegisterTarget::DE => {
                        let og_value = self.registers.get_de();
                        let new_value = og_value.wrapping_add(1);
                        self.registers.set_de(new_value);
                        self.pc.wrapping_add(1)
                    },BigRegisterTarget::HL => {
                        let og_value = self.registers.get_hl();
                        let new_value = og_value.wrapping_add(1);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    },
                    _ => {self.pc}
                }
            }

            Instruction::INC_SP => {
                self.sp.wrapping_add(1);
                self.pc.wrapping_add(1)
            }

            Instruction::DEC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let (value, did_overflow) = self.registers.a_reg.overflowing_sub(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(true);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::B => {
                        let (value, did_overflow) = self.registers.b_reg.overflowing_sub(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(true);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::C => {
                        let (value, did_overflow) = self.registers.c_reg.overflowing_sub(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(true);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => {
                        let (value, did_overflow) = self.registers.d_reg.overflowing_sub(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(true);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::E => {
                        let (value, did_overflow) = self.registers.e_reg.overflowing_sub(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(true);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::H => {
                        let (value, did_overflow) = self.registers.h_reg.overflowing_sub(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(true);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::L => {
                        let (value, did_overflow) = self.registers.l_reg.overflowing_sub(1);
                        self.registers.set_z(self.registers.a_reg == 0);
                        self.registers.set_n(true);
                        self.registers.set_h(did_overflow);
                        self.pc.wrapping_add(1)
                    }

                }
            }

            Instruction::DEC_HL => {
                let (value, did_overflow) = self.registers.get_hl().overflowing_sub(1);
                self.registers.set_hl(value);
                self.registers.set_z(self.registers.a_reg == 0);
                self.registers.set_n(true);
                self.registers.set_h(did_overflow);
                self.pc.wrapping_add(1)
            }

            Instruction::DEC_R16(target) => {
                match target {
                    BigRegisterTarget::AF => {
                        let og_value = self.registers.get_af();
                        let new_value = og_value.wrapping_sub(1);
                        self.registers.set_af(new_value);
                        self.pc.wrapping_add(1)
                    },
                    BigRegisterTarget::BC => {
                        let og_value = self.registers.get_bc();
                        let new_value = og_value.wrapping_sub(1);
                        self.registers.set_bc(new_value);
                        self.pc.wrapping_add(1)
                    },
                    BigRegisterTarget::DE => {
                        let og_value = self.registers.get_de();
                        let new_value = og_value.wrapping_sub(1);
                        self.registers.set_de(new_value);
                        self.pc.wrapping_add(1)
                    },
                    BigRegisterTarget::HL => {
                        let og_value = self.registers.get_hl();
                        let new_value = og_value.wrapping_sub(1);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    },

                }
            }

            Instruction::DEC_SP => {
                self.sp.wrapping_sub(1);
                self.pc.wrapping_add(1)
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

            Instruction::AND(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a_reg;
                        let result = self.bit_and(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::B => {
                        let value = self.registers.b_reg;
                        let result = self.bit_and(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::C => {
                        let value = self.registers.c_reg;
                        let result = self.bit_and(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => {
                        let value = self.registers.d_reg;
                        let result = self.bit_and(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::E => {
                        let value = self.registers.e_reg;
                        let result = self.bit_and(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::H => {
                        let value = self.registers.h_reg;
                        let result = self.bit_and(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::L => {
                        let value = self.registers.l_reg;
                        let result = self.bit_and(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    }
                }
            }

            Instruction::OR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a_reg;
                        let result = self.bit_or(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::B => {
                        let value = self.registers.b_reg;
                        let result = self.bit_or(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::C => {
                        let value = self.registers.c_reg;
                        let result = self.bit_or(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => {
                        let value = self.registers.d_reg;
                        let result = self.bit_or(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::E => {
                        let value = self.registers.e_reg;
                        let result = self.bit_or(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::H => {
                        let value = self.registers.h_reg;
                        let result = self.bit_or(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::L => {
                        let value = self.registers.l_reg;
                        let result = self.bit_or(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    }
                }
            }

            Instruction::XOR(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a_reg;
                        let result = self.bit_xor(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::B => {
                        let value = self.registers.b_reg;
                        let result = self.bit_xor(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::C => {
                        let value = self.registers.c_reg;
                        let result = self.bit_xor(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => {
                        let value = self.registers.d_reg;
                        let result = self.bit_xor(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::E => {
                        let value = self.registers.e_reg;
                        let result = self.bit_xor(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::H => {
                        let value = self.registers.h_reg;
                        let result = self.bit_xor(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::L => {
                        let value = self.registers.l_reg;
                        let result = self.bit_xor(value);
                        self.registers.a_reg = result;
                        self.pc.wrapping_add(1)
                    }
                }
            }

            Instruction::JPCC(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.get_z(),
                    JumpTest::NotCarry => !self.registers.get_c(),
                    JumpTest::Zero => self.registers.get_z(),
                    JumpTest::Carry => self.registers.get_c(),
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }

            Instruction::JP => {
                self.jump(true)
            }

            Instruction::JP_HL => {
                let least_significant_byte = self.bus.read_byte(self.registers.get_hl() + 1) as u16;
                let most_significant_byte = self.bus.read_byte(self.registers.get_hl() + 2) as u16;
                self.pc = (most_significant_byte << 8) | least_significant_byte;
                self.pc
            }

            Instruction::LD(LoadType) => {
               match LoadType {
                   LoadType::R8ToR8(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a_reg,
                        LoadByteSource::B => self.registers.b_reg,
                        LoadByteSource::C => self.registers.c_reg,
                        LoadByteSource::D => self.registers.d_reg,
                        LoadByteSource::E => self.registers.e_reg,
                        LoadByteSource::H => self.registers.h_reg,
                        LoadByteSource::L => self.registers.l_reg,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl())

                    };

                    match target {
                        LoadByteTarget::A => self.registers.a_reg = source_value,
                        LoadByteTarget::B => self.registers.b_reg = source_value,
                        LoadByteTarget::C => self.registers.c_reg = source_value,
                        LoadByteTarget::D => self.registers.d_reg = source_value,
                        LoadByteTarget::E => self.registers.e_reg = source_value,
                        LoadByteTarget::H => self.registers.h_reg = source_value,
                        LoadByteTarget::L => self.registers.l_reg = source_value,
                        LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value)

                    };

                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _                  => self.pc.wrapping_add(1),
                    }

                    
                   }

                   LoadType::HLtoR8(target) => {
                    match target {
                        LoadByteTarget::A => {
                            self.registers.a_reg = self.registers.get_hl() as u8;
                            self.pc.wrapping_add(1)
                        },
                        LoadByteTarget::B => {
                            self.registers.b_reg = self.registers.get_hl() as u8;
                            self.pc.wrapping_add(1)
                        },
                        LoadByteTarget::C => {
                            self.registers.c_reg = self.registers.get_hl() as u8;
                            self.pc.wrapping_add(1)
                        },
                        LoadByteTarget::D => {
                            self.registers.d_reg = self.registers.get_hl() as u8;
                            self.pc.wrapping_add(1)
                        },
                        LoadByteTarget::E => {
                            self.registers.e_reg = self.registers.get_hl() as u8;
                            self.pc.wrapping_add(1)
                        },
                        LoadByteTarget::H => {
                            self.registers.h_reg = self.registers.get_hl() as u8;
                            self.pc.wrapping_add(1)
                        },
                        LoadByteTarget::L => {
                            self.registers.l_reg = self.registers.get_hl() as u8;
                            self.pc.wrapping_add(1)
                        },
                         _ => {self.pc}
                    }
                   }

                   LoadType::R8ToHL(target) => {
                    match target {
                        LoadByteSource::A => {
                            self.registers.set_hl(self.registers.a_reg as u16);
                            self.pc.wrapping_add(1)
                        },
                        LoadByteSource::B => {
                            self.registers.set_hl(self.registers.b_reg as u16);
                            self.pc.wrapping_add(1)
                        },
                        LoadByteSource::C => {
                            self.registers.set_hl(self.registers.c_reg as u16);
                            self.pc.wrapping_add(1)
                        },
                        LoadByteSource::D => {
                            self.registers.set_hl(self.registers.d_reg as u16);
                            self.pc.wrapping_add(1)
                        },
                        LoadByteSource::E => {
                            self.registers.set_hl(self.registers.e_reg as u16);
                            self.pc.wrapping_add(1)
                        },
                        LoadByteSource::H => {
                            self.registers.set_hl(self.registers.h_reg as u16);
                            self.pc.wrapping_add(1)
                        },
                        LoadByteSource::L => {
                            self.registers.set_hl(self.registers.l_reg as u16);
                            self.pc.wrapping_add(1)
                        },

                        _ => {self.pc}

                    }
                   }

                   LoadType::N8toR8(target) => {
                    let N8 = self.read_next_byte();
                    match target {
                        ArithmeticTarget::A => {self.registers.a_reg = N8;},
                        ArithmeticTarget::B => {self.registers.b_reg = N8;},
                        ArithmeticTarget::C => {self.registers.c_reg = N8;},
                        ArithmeticTarget::D => {self.registers.d_reg = N8;},
                        ArithmeticTarget::E => {self.registers.e_reg = N8;},
                        ArithmeticTarget::H => {self.registers.h_reg = N8;},
                        ArithmeticTarget::L => {self.registers.l_reg = N8;},
                    }
                    self.pc.wrapping_add(2)
                   }

                   LoadType::N16ADtoA => {
                    let N8 = self.read_next_byte();
                    self.registers.a_reg = self.bus.read_byte(N8 as usize);
                    self.pc.wrapping_add(2)
                   }

                   LoadType::N16toR16(target) => {
                    let N16 = self.read_next_byte() as u16;

                    match target {
                        BigRegisterTarget::AF => {self.registers.set_af(N16);},
                        BigRegisterTarget::BC => {self.registers.set_af(N16);},
                        BigRegisterTarget::DE => {self.registers.set_de(N16);},
                        BigRegisterTarget::HL => {self.registers.set_hl(N16);}

                    }
                    self.pc.wrapping_add(2)
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

            Instruction::SWAP(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let lower_nibble = self.registers.a_reg & 0x0F;
                        let upper_nibble = self.registers.a_reg & 0xF0;
                        let shifted_lower = lower_nibble << 4;
                        let shifted_upper = upper_nibble >> 4;
                        let result = shifted_lower | shifted_upper;
                        self.registers.a_reg = result;
                        
                        self.registers.set_z(result == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(false);
                        self.registers.set_c(false);

                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::B => {
                        let lower_nibble = self.registers.b_reg & 0x0F;
                        let upper_nibble = self.registers.b_reg & 0xF0;
                        let shifted_lower = lower_nibble << 4;
                        let shifted_upper = upper_nibble >> 4;
                        let result = shifted_lower | shifted_upper;
                        self.registers.b_reg = result;
                        
                        self.registers.set_z(result == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(false);
                        self.registers.set_c(false);

                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::C => {
                        let lower_nibble = self.registers.c_reg & 0x0F;
                        let upper_nibble = self.registers.c_reg & 0xF0;
                        let shifted_lower = lower_nibble << 4;
                        let shifted_upper = upper_nibble >> 4;
                        let result = shifted_lower | shifted_upper;
                        self.registers.c_reg = result;
                        
                        self.registers.set_z(result == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(false);
                        self.registers.set_c(false);

                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::D => {
                        let lower_nibble = self.registers.d_reg & 0x0F;
                        let upper_nibble = self.registers.d_reg & 0xF0;
                        let shifted_lower = lower_nibble << 4;
                        let shifted_upper = upper_nibble >> 4;
                        let result = shifted_lower | shifted_upper;
                        self.registers.d_reg = result;
                        
                        self.registers.set_z(result == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(false);
                        self.registers.set_c(false);

                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::E => {
                        let lower_nibble = self.registers.e_reg & 0x0F;
                        let upper_nibble = self.registers.ereg & 0xF0;
                        let shifted_lower = lower_nibble << 4;
                        let shifted_upper = upper_nibble >> 4;
                        let result = shifted_lower | shifted_upper;
                        self.registers.e_reg = result;
                        
                        self.registers.set_z(result == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(false);
                        self.registers.set_c(false);

                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::H => {
                        let lower_nibble = self.registers.h_reg & 0x0F;
                        let upper_nibble = self.registers.h_reg & 0xF0;
                        let shifted_lower = lower_nibble << 4;
                        let shifted_upper = upper_nibble >> 4;
                        let result = shifted_lower | shifted_upper;
                        self.registers.h_reg = result;
                        
                        self.registers.set_z(result == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(false);
                        self.registers.set_c(false);

                        self.pc.wrapping_add(1)
                    },
                    ArithmeticTarget::L => {
                        let lower_nibble = self.registers.l_reg & 0x0F;
                        let upper_nibble = self.registers.l_reg & 0xF0;
                        let shifted_lower = lower_nibble << 4;
                        let shifted_upper = upper_nibble >> 4;
                        let result = shifted_lower | shifted_upper;
                        self.registers.l_reg = result;
                        
                        self.registers.set_z(result == 0);
                        self.registers.set_n(false);
                        self.registers.set_h(false);
                        self.registers.set_c(false);

                        self.pc.wrapping_add(1)
                    }
                }
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

    // fn adc(&mut self, value: u8) -> u8 {
    //     let (new_value, did_overflow) = self.registers.a_reg.overflowing_add(value.overflowing_add(self.registers.get_c())[0]);


    //     self.registers.set_z(new_value)
    // }


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

    fn bit_and(&mut self, value: u8) -> u8{
        let new_value = self.registers.a_reg & value;
        
        self.registers.set_z(new_value == 0);
        self.registers.set_n(false);
        self.registers.set_h(true);
        self.registers.set_c(false);

        return new_value;
    }

    fn bit_or(&mut self, value: u8) -> u8{
        let new_value = self.registers.a_reg | value;
        
        self.registers.set_z(new_value == 0);
        self.registers.set_n(false);
        self.registers.set_h(true);
        self.registers.set_c(false);

        return new_value;
    }

    fn bit_xor(&mut self, value: u8) -> u8{
        let new_value = self.registers.a_reg ^ value;
        
        self.registers.set_z(new_value == 0);
        self.registers.set_n(false);
        self.registers.set_h(true);
        self.registers.set_c(false);

        return new_value;
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
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    INC_HL,
    DEC_HL,
    INC_R16(BigRegisterTarget),
    DEC_R16(BigRegisterTarget),
    INC_SP,
    DEC_SP,
    JPCC(JumpTest),
    JP,
    JP_HL,
    LD(LoadType),
    PUSH(StackTargets),
    POP(StackTargets),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    SWAP(ArithmeticTarget),
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

enum BigRegisterTarget {
    AF, BC, DE, HL
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
    R8ToR8(LoadByteTarget, LoadByteSource),
    R8ToHL(LoadByteSource),
    HLtoR8(LoadByteTarget),
    N8toR8(ArithmeticTarget),
    N16toR16(BigRegisterTarget),
    R8toHL,
    AtoR16,
    N16ADtoA,




}

enum StackTargets{
    AF, BC, DE, HL
}


 


