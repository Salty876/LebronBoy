use super::{Cpu};
use super::instructions::*;


// Add function






pub fn execute(cpu: &mut Cpu, instr: Instruction, prefixed: bool) -> u16 {
    if cpu.halted {
        return cpu.pc;
    }

    // NOTE: for now keep PC math like your current style
    // Later you’ll add cycles + timing here.
    match instr {
        Instruction::NOP => cpu.pc.wrapping_add(1),

        Instruction::HALT => {
            cpu.halted = true;
            cpu.pc
        }

        Instruction::JP(test) => {
            let cond = match test {
                JumpTest::NotZero => !cpu.regs.get_z(),
                JumpTest::Zero => cpu.regs.get_z(),
                JumpTest::NotCarry => !cpu.regs.get_carry(),
                JumpTest::Carry => cpu.regs.get_carry(),
                JumpTest::Always => true,
            };
            if cond { cpu.next_word() } else { cpu.pc.wrapping_add(3) }
        }

        Instruction::CALL(test) => {
            let cond = match test {
                JumpTest::Always => true,
                JumpTest::NotZero => !cpu.regs.get_z(),
                JumpTest::Zero => cpu.regs.get_z(),
                JumpTest::NotCarry => !cpu.regs.get_carry(),
                JumpTest::Carry => cpu.regs.get_carry(),
            };
            let target = cpu.next_word();
            let ret_addr = cpu.pc.wrapping_add(3);
            if cond {
                cpu.push_word(ret_addr);
                target
            } else {
                ret_addr
            }
        }

        Instruction::RET(test) => {
            let cond = match test {
                JumpTest::Always => true,
                JumpTest::NotZero => !cpu.regs.get_z(),
                JumpTest::Zero => cpu.regs.get_z(),
                JumpTest::NotCarry => !cpu.regs.get_carry(),
                JumpTest::Carry => cpu.regs.get_carry(),
            };
            if cond { cpu.pop_word() } else { cpu.pc.wrapping_add(1) }
        }

        Instruction::ADD(ArithmeticTarget) => {
            
            match ArithmeticTarget {
                ArithmeticTarget::A => {
                    let value = cpu.regs.a();
                    let new_value = cpu.add(value);
                    cpu.regs.set_a(new_value);
                    cpu.pc.wrapping_add(1)

                },
                ArithmeticTarget::B => {
                    let value = cpu.regs.b();
                    let new_value = cpu.add(value);
                    cpu.regs.set_b(new_value);
                    cpu.pc.wrapping_add(1)

                },
                ArithmeticTarget::C => {
                    let value = cpu.regs.c();
                    let new_value = cpu.add(value);
                    cpu.regs.set_c(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::D => {
                    let value = cpu.regs.d();
                    let new_value = cpu.add(value);
                    cpu.regs.set_d(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::E => {
                    let value = cpu.regs.e();
                    let new_value = cpu.add(value);
                    cpu.regs.set_e(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::H => {
                    let value = cpu.regs.h();
                    let new_value = cpu.add(value);
                    cpu.regs.set_h(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::L => {
                    let value = cpu.regs.l();
                    let new_value = cpu.add(value);
                    cpu.regs.set_l(new_value);
                    cpu.pc.wrapping_add(1)
                }
                
                _ => {/*more targets*/ cpu.pc}
            }
        }


        Instruction::SUB(ArithmeticTarget) => {
            
            match ArithmeticTarget {
                ArithmeticTarget::A => {
                    let value = cpu.regs.a();
                    let new_value = cpu.sub(value);
                    cpu.regs.set_a(new_value);
                    cpu.pc.wrapping_add(1)

                },
                ArithmeticTarget::B => {
                    let value = cpu.regs.b();
                    let new_value = cpu.sub(value);
                    cpu.regs.set_b(new_value);
                    cpu.pc.wrapping_add(1)

                },
                ArithmeticTarget::C => {
                    let value = cpu.regs.c();
                    let new_value = cpu.sub(value);
                    cpu.regs.set_c(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::D => {
                    let value = cpu.regs.d();
                    let new_value = cpu.sub(value);
                    cpu.regs.set_d(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::E => {
                    let value = cpu.regs.e();
                    let new_value = cpu.sub(value);
                    cpu.regs.set_e(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::H => {
                    let value = cpu.regs.h();
                    let new_value = cpu.sub(value);
                    cpu.regs.set_h(new_value);
                    cpu.pc.wrapping_add(1)

                },ArithmeticTarget::L => {
                    let value = cpu.regs.l();
                    let new_value = cpu.sub(value);
                    cpu.regs.set_l(new_value);
                    cpu.pc.wrapping_add(1)
                }
                
                _ => {/*more targets*/ cpu.pc}
            }
        }

        Instruction::LD(loadType) => {
            match loadType{
                LoadType::R8ToR8(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => cpu.regs.a(),
                        LoadByteSource::B => cpu.regs.b(),
                        LoadByteSource::C => cpu.regs.c(),
                        LoadByteSource::D => cpu.regs.d(),
                        LoadByteSource::E => cpu.regs.e(),
                        LoadByteSource::H => cpu.regs.h(),
                        LoadByteSource::L => cpu.regs.l(),
                        LoadByteSource::D8 => cpu.next_byte(),
                        LoadByteSource::HLI => cpu.bus.read_byte(cpu.regs.get_hl())

                    };

                    match target {
                        LoadByteTarget::A => cpu.regs.set_a(source_value),
                        LoadByteTarget::B => cpu.regs.set_b(source_value),
                        LoadByteTarget::C => cpu.regs.set_c(source_value),
                        LoadByteTarget::D => cpu.regs.set_d(source_value),
                        LoadByteTarget::E => cpu.regs.set_e(source_value),
                        LoadByteTarget::H => cpu.regs.set_h(source_value),
                        LoadByteTarget::L => cpu.regs.set_l(source_value),
                        LoadByteTarget::HLI => cpu.bus.write_byte(cpu.regs.get_hl(), source_value)

                    };

                    match source {
                        LoadByteSource::D8 => cpu.pc.wrapping_add(2),
                        _                  => cpu.pc.wrapping_add(1),
                    }

                    
                   }

                LoadType::D16toR16(target) => {
                    let value = cpu.next_word();
                    match target {
                        BigLoadByteTarget::AB => cpu.regs.set_af(value),
                        BigLoadByteTarget::CD => cpu.regs.set_bc(value),
                        BigLoadByteTarget::DE => cpu.regs.set_de(value),
                        BigLoadByteTarget::HL => cpu.regs.set_hl(value),
                        BigLoadByteTarget::SP => cpu.sp = value,
                    };
                    cpu.pc.wrapping_add(3)
                }
                
                LoadType::HLtoSP => {
                    cpu.sp = cpu.regs.get_hl();
                    cpu.pc.wrapping_add(1)
                },

                LoadType::SPtoA16 => {
                    let addr = cpu.next_word();
                    let sp = cpu.sp;

                    cpu.bus.write_byte(addr, (sp & 0xFF) as u8);
                    cpu.bus.write_byte(addr.wrapping_add(1), (sp >> 8) as u8);
                    cpu.pc.wrapping_add(3)
                }

                LoadType::R16toSP(source) => {
                    let value = match source {
                        BigRegisterTarget::AF => cpu.regs.get_af(),
                        BigRegisterTarget::BC => cpu.regs.get_bc(),
                        BigRegisterTarget::DE => cpu.regs.get_de(),
                        BigRegisterTarget::HL => cpu.regs.get_hl(),
                    };
                    cpu.sp = value;
                    cpu.pc.wrapping_add(1)
                }

                LoadType::SP8toHL => {
                    let offset = cpu.next_byte() as i8 as i16;
                    let sp = cpu.sp;
                    let result  = sp.wrapping_add(offset as u16);
                    cpu.regs.set_hl(result);


                    let sp_low = sp & 0xFF;
                    let offset_up: u16 = (offset as u16) & 0xFF;

                    // set flags
                    cpu.regs.set_z(false);
                    cpu.regs.set_n(false);
                    cpu.regs.set_carry((((sp_low & 0xFF) + ((offset_up as u16) & 0xFF)) > 0xFF));
                    cpu.regs.set_hc(((sp_low & 0x0F) + ((offset_up as u16) & 0x0F)) > 0x0F);


                    cpu.pc.wrapping_add(2)

                },

                _ => {cpu.pc}
        }
        // Keep migrating your existing ADD/SUB/LD/PUSH/POP here next.
        
    }

    _ => cpu.pc}
}


#[cfg(test)]
mod decode_tests {
    use super::super::instructions::Instruction;

    #[test]
    fn decode_nop() {
        let i = Instruction::decode(0x00, false).unwrap();
        matches!(i, Instruction::NOP);
    }

    #[test]
    fn decode_halt() {
        let i = Instruction::decode(0x76, false).unwrap();
        matches!(i, Instruction::HALT);
    }
}


#[cfg(test)]
mod step_tests {
    use crate::cpu::Cpu;

    #[test]
    fn step_nop_advances_pc() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0100;
        cpu.bus.write_byte(0x0100, 0x00); // NOP

        cpu.step();

        assert_eq!(cpu.pc, 0x0101);
    }

    #[test]
    fn step_halt_stops_advancing() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0100;
        cpu.bus.write_byte(0x0100, 0x76); // HALT

        cpu.step();
        assert!(cpu.halted);

        let pc_after = cpu.pc;
        cpu.step(); // should do nothing
        assert_eq!(cpu.pc, pc_after);
    }
}


#[cfg(test)]
mod call_ret_tests {
    use crate::cpu::Cpu;


    #[test]
    #[ignore]
    fn call_then_ret_returns_to_next_instruction() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x0100;
        cpu.sp = 0xFFFE;

        // Program:
        // 0100: CD 00 02   CALL 0x0200
        // 0103: 00         NOP   (should execute after RET)
        // 0200: C9         RET
        cpu.bus.write_byte(0x0100, 0xCD);
        cpu.bus.write_byte(0x0101, 0x00);
        cpu.bus.write_byte(0x0102, 0x02);
        cpu.bus.write_byte(0x0103, 0x00);

        cpu.bus.write_byte(0x0200, 0xC9);

        // This assumes you’ve implemented decoding/execution for CALL (0xCD) and RET (0xC9).
        cpu.step();
        assert_eq!(cpu.pc, 0x0200, "CALL should jump to target");
        assert_eq!(cpu.sp, 0xFFFC, "CALL should push return address");

        cpu.step();
        assert_eq!(cpu.pc, 0x0103, "RET should return to next instruction after CALL");
        assert_eq!(cpu.sp, 0xFFFE, "RET should restore SP");

        cpu.step();
        assert_eq!(cpu.pc, 0x0104, "NOP after return should run");
    }
}






// use crate::cpu::Cpu;
// use crate::cpu::instructions::Instruction;
// use crate::cpu::instructions::{ArithmeticTarget, BigRegisterTarget, JumpTest, LoadType, LoadByteSource, LoadByteTarget, StackTargets};


//     pub fn execute (cpu: &mut Cpu, instruction: Instruction) -> u16 {
//         if cpu.is_halted{
//             return cpu.pc;
//         }
//         match instruction{
//             Instruction::ADD(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let value = cpu.registers.a_reg;
//                         let new_value = cpu.add(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)

//                     }
//                     ArithmeticTarget::B => {
//                         let value = cpu.registers.b_reg;
//                         let new_value = cpu.add(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)

//                     }
//                     ArithmeticTarget::C => {
//                         let value = cpu.registers.c_reg;
//                         let new_value = cpu.add(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)

//                     }
//                     ArithmeticTarget::D => {
//                         let value = cpu.registers.d_reg;
//                         let new_value = cpu.add(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)

//                     }
//                     ArithmeticTarget::E => {
//                         let value = cpu.registers.e_reg;
//                         let new_value = cpu.add(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)

//                     }
//                     ArithmeticTarget::H => {
//                         let value = cpu.registers.h_reg;
//                         let new_value = cpu.add(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)

//                     }
//                     ArithmeticTarget::L => {
//                         let value = cpu.registers.l_reg;
//                         let new_value = cpu.add(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     }
//                      _ => {/*more rargets*/ cpu.pc}
//                 }
                
//             }

//             Instruction::INC(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let (value, did_overflow) = cpu.registers.a_reg.overflowing_add(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)

//                     },
//                     ArithmeticTarget::B => {
//                         let (value, did_overflow) = cpu.registers.b_reg.overflowing_add(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)


//                     },
//                     ArithmeticTarget::C => {
//                         let (value, did_overflow) = cpu.registers.c_reg.overflowing_add(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)


//                     },
//                     ArithmeticTarget::D => {
//                         let (value, did_overflow) = cpu.registers.d_reg.overflowing_add(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)


//                     },
//                     ArithmeticTarget::E => {
//                         let (value, did_overflow) = cpu.registers.e_reg.overflowing_add(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)


//                     },
//                     ArithmeticTarget::H => {
//                         let (value, did_overflow) = cpu.registers.h_reg.overflowing_add(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)


//                     },
//                     ArithmeticTarget::L => {
//                         let (value, did_overflow) = cpu.registers.l_reg.overflowing_add(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)


//                     }

//                     _ => {cpu.pc}
//                 }
//             }

//             Instruction::INC_HL => {
//                 let (value, did_overflow) = cpu.registers.get_hl().overflowing_add(1);
//                 cpu.registers.set_hl(value);
//                 cpu.registers.set_z(cpu.registers.a_reg == 0);
//                 cpu.registers.set_n(false);
//                 cpu.registers.set_h(did_overflow);
//                 cpu.pc.wrapping_add(1)
//             }

//             Instruction::INC_R16(target) => {
//                 match target {
//                     BigRegisterTarget::AF => {
//                         let og_value = cpu.registers.get_af();
//                         let new_value = og_value.wrapping_add(1);
//                         cpu.registers.set_af(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     BigRegisterTarget::BC => {
//                         let og_value = cpu.registers.get_bc();
//                         let new_value = og_value.wrapping_add(1);
//                         cpu.registers.set_bc(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },BigRegisterTarget::DE => {
//                         let og_value = cpu.registers.get_de();
//                         let new_value = og_value.wrapping_add(1);
//                         cpu.registers.set_de(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },BigRegisterTarget::HL => {
//                         let og_value = cpu.registers.get_hl();
//                         let new_value = og_value.wrapping_add(1);
//                         cpu.registers.set_hl(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     _ => {cpu.pc}
//                 }
//             }

//             Instruction::INC_SP => {
//                 cpu.sp.wrapping_add(1);
//                 cpu.pc.wrapping_add(1)
//             }

//             Instruction::DEC(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let (value, did_overflow) = cpu.registers.a_reg.overflowing_sub(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(true);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::B => {
//                         let (value, did_overflow) = cpu.registers.b_reg.overflowing_sub(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(true);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::C => {
//                         let (value, did_overflow) = cpu.registers.c_reg.overflowing_sub(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(true);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::D => {
//                         let (value, did_overflow) = cpu.registers.d_reg.overflowing_sub(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(true);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::E => {
//                         let (value, did_overflow) = cpu.registers.e_reg.overflowing_sub(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(true);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::H => {
//                         let (value, did_overflow) = cpu.registers.h_reg.overflowing_sub(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(true);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::L => {
//                         let (value, did_overflow) = cpu.registers.l_reg.overflowing_sub(1);
//                         cpu.registers.set_z(cpu.registers.a_reg == 0);
//                         cpu.registers.set_n(true);
//                         cpu.registers.set_h(did_overflow);
//                         cpu.pc.wrapping_add(1)
//                     }

//                 }
//             }

//             Instruction::DEC_HL => {
//                 let (value, did_overflow) = cpu.registers.get_hl().overflowing_sub(1);
//                 cpu.registers.set_hl(value);
//                 cpu.registers.set_z(cpu.registers.a_reg == 0);
//                 cpu.registers.set_n(true);
//                 cpu.registers.set_h(did_overflow);
//                 cpu.pc.wrapping_add(1)
//             }

//             Instruction::DEC_R16(target) => {
//                 match target {
//                     BigRegisterTarget::AF => {
//                         let og_value = cpu.registers.get_af();
//                         let new_value = og_value.wrapping_sub(1);
//                         cpu.registers.set_af(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     BigRegisterTarget::BC => {
//                         let og_value = cpu.registers.get_bc();
//                         let new_value = og_value.wrapping_sub(1);
//                         cpu.registers.set_bc(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     BigRegisterTarget::DE => {
//                         let og_value = cpu.registers.get_de();
//                         let new_value = og_value.wrapping_sub(1);
//                         cpu.registers.set_de(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },
//                     BigRegisterTarget::HL => {
//                         let og_value = cpu.registers.get_hl();
//                         let new_value = og_value.wrapping_sub(1);
//                         cpu.registers.set_hl(new_value);
//                         cpu.pc.wrapping_add(1)
//                     },

//                 }
//             }

//             Instruction::DEC_SP => {
//                 cpu.sp.wrapping_sub(1);
//                 cpu.pc.wrapping_add(1)
//             }

//             Instruction::SUB(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let value = cpu.registers.a_reg;
//                         let new_value = cpu.sub(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::B => {
//                         let value = cpu.registers.b_reg;
//                         let new_value = cpu.sub(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::C => {
//                         let value = cpu.registers.c_reg;
//                         let new_value = cpu.sub(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::D => {
//                         let value = cpu.registers.d_reg;
//                         let new_value = cpu.sub(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::E => {
//                         let value = cpu.registers.e_reg;
//                         let new_value = cpu.sub(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::H => {
//                         let value = cpu.registers.h_reg;
//                         let new_value = cpu.sub(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::L => {
//                         let value = cpu.registers.l_reg;
//                         let new_value = cpu.sub(value);
//                         cpu.registers.a_reg = new_value;
//                         cpu.pc.wrapping_add(1)
//                     }
//                 }
//             }

//             Instruction::AND(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let value = cpu.registers.a_reg;
//                         let result = cpu.bit_and(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::B => {
//                         let value = cpu.registers.b_reg;
//                         let result = cpu.bit_and(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::C => {
//                         let value = cpu.registers.c_reg;
//                         let result = cpu.bit_and(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::D => {
//                         let value = cpu.registers.d_reg;
//                         let result = cpu.bit_and(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::E => {
//                         let value = cpu.registers.e_reg;
//                         let result = cpu.bit_and(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::H => {
//                         let value = cpu.registers.h_reg;
//                         let result = cpu.bit_and(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::L => {
//                         let value = cpu.registers.l_reg;
//                         let result = cpu.bit_and(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     }
//                 }
//             }

//             Instruction::OR(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let value = cpu.registers.a_reg;
//                         let result = cpu.bit_or(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::B => {
//                         let value = cpu.registers.b_reg;
//                         let result = cpu.bit_or(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::C => {
//                         let value = cpu.registers.c_reg;
//                         let result = cpu.bit_or(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::D => {
//                         let value = cpu.registers.d_reg;
//                         let result = cpu.bit_or(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::E => {
//                         let value = cpu.registers.e_reg;
//                         let result = cpu.bit_or(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::H => {
//                         let value = cpu.registers.h_reg;
//                         let result = cpu.bit_or(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::L => {
//                         let value = cpu.registers.l_reg;
//                         let result = cpu.bit_or(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     }
//                 }
//             }

//             Instruction::XOR(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let value = cpu.registers.a_reg;
//                         let result = cpu.bit_xor(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::B => {
//                         let value = cpu.registers.b_reg;
//                         let result = cpu.bit_xor(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::C => {
//                         let value = cpu.registers.c_reg;
//                         let result = cpu.bit_xor(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::D => {
//                         let value = cpu.registers.d_reg;
//                         let result = cpu.bit_xor(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::E => {
//                         let value = cpu.registers.e_reg;
//                         let result = cpu.bit_xor(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::H => {
//                         let value = cpu.registers.h_reg;
//                         let result = cpu.bit_xor(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::L => {
//                         let value = cpu.registers.l_reg;
//                         let result = cpu.bit_xor(value);
//                         cpu.registers.a_reg = result;
//                         cpu.pc.wrapping_add(1)
//                     }
//                 }
//             }

//             Instruction::JPCC(test) => {
//                 let jump_condition = match test {
//                     JumpTest::NotZero => !cpu.registers.get_z(),
//                     JumpTest::NotCarry => !cpu.registers.get_c(),
//                     JumpTest::Zero => cpu.registers.get_z(),
//                     JumpTest::Carry => cpu.registers.get_c(),
//                     JumpTest::Always => true
//                 };
//                 cpu.jump(jump_condition)
//             }

//             Instruction::JP => {
//                 cpu.jump(true)
//             }

//             Instruction::JP_HL => {
//                 let least_significant_byte = cpu.bus.read_byte(cpu.registers.get_hl() + 1) as u16;
//                 let most_significant_byte = cpu.bus.read_byte(cpu.registers.get_hl() + 2) as u16;
//                 cpu.pc = (most_significant_byte << 8) | least_significant_byte;
//                 cpu.pc
//             }

//             Instruction::LD(LoadType) => {
//                match LoadType {
//                    LoadType::R8ToR8(target, source) => {
//                     let source_value = match source {
//                         LoadByteSource::A => cpu.registers.a_reg,
//                         LoadByteSource::B => cpu.registers.b_reg,
//                         LoadByteSource::C => cpu.registers.c_reg,
//                         LoadByteSource::D => cpu.registers.d_reg,
//                         LoadByteSource::E => cpu.registers.e_reg,
//                         LoadByteSource::H => cpu.registers.h_reg,
//                         LoadByteSource::L => cpu.registers.l_reg,
//                         LoadByteSource::D8 => cpu.read_next_byte(),
//                         LoadByteSource::HLI => cpu.bus.read_byte(cpu.registers.get_hl())

//                     };

//                     match target {
//                         LoadByteTarget::A => cpu.registers.a_reg = source_value,
//                         LoadByteTarget::B => cpu.registers.b_reg = source_value,
//                         LoadByteTarget::C => cpu.registers.c_reg = source_value,
//                         LoadByteTarget::D => cpu.registers.d_reg = source_value,
//                         LoadByteTarget::E => cpu.registers.e_reg = source_value,
//                         LoadByteTarget::H => cpu.registers.h_reg = source_value,
//                         LoadByteTarget::L => cpu.registers.l_reg = source_value,
//                         LoadByteTarget::HLI => cpu.bus.write_byte(cpu.registers.get_hl(), source_value)

//                     };

//                     match source {
//                         LoadByteSource::D8 => cpu.pc.wrapping_add(2),
//                         _                  => cpu.pc.wrapping_add(1),
//                     }

                    
//                    }

//                    LoadType::HLtoR8(target) => {
//                     match target {
//                         LoadByteTarget::A => {
//                             cpu.registers.a_reg = cpu.registers.get_hl() as u8;
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteTarget::B => {
//                             cpu.registers.b_reg = cpu.registers.get_hl() as u8;
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteTarget::C => {
//                             cpu.registers.c_reg = cpu.registers.get_hl() as u8;
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteTarget::D => {
//                             cpu.registers.d_reg = cpu.registers.get_hl() as u8;
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteTarget::E => {
//                             cpu.registers.e_reg = cpu.registers.get_hl() as u8;
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteTarget::H => {
//                             cpu.registers.h_reg = cpu.registers.get_hl() as u8;
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteTarget::L => {
//                             cpu.registers.l_reg = cpu.registers.get_hl() as u8;
//                             cpu.pc.wrapping_add(1)
//                         },
//                          _ => {cpu.pc}
//                     }
//                    }

//                    LoadType::R8ToHL(target) => {
//                     match target {
//                         LoadByteSource::A => {
//                             cpu.registers.set_hl(cpu.registers.a_reg as u16);
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteSource::B => {
//                             cpu.registers.set_hl(cpu.registers.b_reg as u16);
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteSource::C => {
//                             cpu.registers.set_hl(cpu.registers.c_reg as u16);
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteSource::D => {
//                             cpu.registers.set_hl(cpu.registers.d_reg as u16);
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteSource::E => {
//                             cpu.registers.set_hl(cpu.registers.e_reg as u16);
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteSource::H => {
//                             cpu.registers.set_hl(cpu.registers.h_reg as u16);
//                             cpu.pc.wrapping_add(1)
//                         },
//                         LoadByteSource::L => {
//                             cpu.registers.set_hl(cpu.registers.l_reg as u16);
//                             cpu.pc.wrapping_add(1)
//                         },

//                         _ => {cpu.pc}

//                     }
//                    }

//                    LoadType::N8toR8(target) => {
//                     let N8 = cpu.read_next_byte();
//                     match target {
//                         ArithmeticTarget::A => {cpu.registers.a_reg = N8;},
//                         ArithmeticTarget::B => {cpu.registers.b_reg = N8;},
//                         ArithmeticTarget::C => {cpu.registers.c_reg = N8;},
//                         ArithmeticTarget::D => {cpu.registers.d_reg = N8;},
//                         ArithmeticTarget::E => {cpu.registers.e_reg = N8;},
//                         ArithmeticTarget::H => {cpu.registers.h_reg = N8;},
//                         ArithmeticTarget::L => {cpu.registers.l_reg = N8;},
//                     }
//                     cpu.pc.wrapping_add(2)
//                    }

//                    LoadType::N16ADtoA => {
//                     let N8 = cpu.read_next_byte();
//                     cpu.registers.a_reg = cpu.bus.read_byte(N8 as usize);
//                     cpu.pc.wrapping_add(2)
//                    }

//                    LoadType::N16toR16(target) => {
//                     let N16 = cpu.read_next_byte() as u16;

//                     match target {
//                         BigRegisterTarget::AF => {cpu.registers.set_af(N16);},
//                         BigRegisterTarget::BC => {cpu.registers.set_af(N16);},
//                         BigRegisterTarget::DE => {cpu.registers.set_de(N16);},
//                         BigRegisterTarget::HL => {cpu.registers.set_hl(N16);}

//                     }
//                     cpu.pc.wrapping_add(2)
//                    }

//                    _ => {   panic!("ADD THE REST OF LOAD TYPES")    }
//                }
//             }

//             Instruction::PUSH(target) => {
//                 let value = match target {
//                     StackTargets::BC => cpu.registers.get_bc(),
//                     _ => {  panic!("ALL TARGETS NOT DONE FN")   }
//                 };
//                 cpu.push(value);
//                 return cpu.pc.wrapping_add(1);
//             }

//             Instruction::POP(target) => {
//                 let result = cpu.pop();

//                 match target {
//                     StackTargets::BC => cpu.registers.set_bc(result),
//                     _ => {  panic!("REST OF TARGETS")  }
//                 };
//                 return cpu.pc.wrapping_add(1);
//             }

//             Instruction::CALL(test) => {
//                 let jump_condition = match test {
//                     JumpTest::NotZero => !cpu.registers.get_z(),
//                     _ => {/*DO the other condisions */ false}

//                 };
//                 cpu.call(jump_condition)
//             }

//             Instruction::RET(test) => {
//                 let jump_condition = match test {
//                     JumpTest::NotZero => !cpu.registers.get_z(),
//                     _ => {/*Other condiitons */ false}
//                 };
//                 cpu.return_(jump_condition)
//             }

//             Instruction::SWAP(target) => {
//                 match target {
//                     ArithmeticTarget::A => {
//                         let lower_nibble = cpu.registers.a_reg & 0x0F;
//                         let upper_nibble = cpu.registers.a_reg & 0xF0;
//                         let shifted_lower = lower_nibble << 4;
//                         let shifted_upper = upper_nibble >> 4;
//                         let result = shifted_lower | shifted_upper;
//                         cpu.registers.a_reg = result;
                        
//                         cpu.registers.set_z(result == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(false);
//                         cpu.registers.set_c(false);

//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::B => {
//                         let lower_nibble = cpu.registers.b_reg & 0x0F;
//                         let upper_nibble = cpu.registers.b_reg & 0xF0;
//                         let shifted_lower = lower_nibble << 4;
//                         let shifted_upper = upper_nibble >> 4;
//                         let result = shifted_lower | shifted_upper;
//                         cpu.registers.b_reg = result;
                        
//                         cpu.registers.set_z(result == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(false);
//                         cpu.registers.set_c(false);

//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::C => {
//                         let lower_nibble = cpu.registers.c_reg & 0x0F;
//                         let upper_nibble = cpu.registers.c_reg & 0xF0;
//                         let shifted_lower = lower_nibble << 4;
//                         let shifted_upper = upper_nibble >> 4;
//                         let result = shifted_lower | shifted_upper;
//                         cpu.registers.c_reg = result;
                        
//                         cpu.registers.set_z(result == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(false);
//                         cpu.registers.set_c(false);

//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::D => {
//                         let lower_nibble = cpu.registers.d_reg & 0x0F;
//                         let upper_nibble = cpu.registers.d_reg & 0xF0;
//                         let shifted_lower = lower_nibble << 4;
//                         let shifted_upper = upper_nibble >> 4;
//                         let result = shifted_lower | shifted_upper;
//                         cpu.registers.d_reg = result;
                        
//                         cpu.registers.set_z(result == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(false);
//                         cpu.registers.set_c(false);

//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::E => {
//                         let lower_nibble = cpu.registers.e_reg & 0x0F;
//                         let upper_nibble = cpu.registers.ereg & 0xF0;
//                         let shifted_lower = lower_nibble << 4;
//                         let shifted_upper = upper_nibble >> 4;
//                         let result = shifted_lower | shifted_upper;
//                         cpu.registers.e_reg = result;
                        
//                         cpu.registers.set_z(result == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(false);
//                         cpu.registers.set_c(false);

//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::H => {
//                         let lower_nibble = cpu.registers.h_reg & 0x0F;
//                         let upper_nibble = cpu.registers.h_reg & 0xF0;
//                         let shifted_lower = lower_nibble << 4;
//                         let shifted_upper = upper_nibble >> 4;
//                         let result = shifted_lower | shifted_upper;
//                         cpu.registers.h_reg = result;
                        
//                         cpu.registers.set_z(result == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(false);
//                         cpu.registers.set_c(false);

//                         cpu.pc.wrapping_add(1)
//                     },
//                     ArithmeticTarget::L => {
//                         let lower_nibble = cpu.registers.l_reg & 0x0F;
//                         let upper_nibble = cpu.registers.l_reg & 0xF0;
//                         let shifted_lower = lower_nibble << 4;
//                         let shifted_upper = upper_nibble >> 4;
//                         let result = shifted_lower | shifted_upper;
//                         cpu.registers.l_reg = result;
                        
//                         cpu.registers.set_z(result == 0);
//                         cpu.registers.set_n(false);
//                         cpu.registers.set_h(false);
//                         cpu.registers.set_c(false);

//                         cpu.pc.wrapping_add(1)
//                     }
//                 }
//             }

//             Instruction::NOP => {
//                 cpu.pc.wrapping_add(1)
//             }

//             Instruction::HALT => {
//                 cpu.is_halted = true;
//                 return cpu.pc;
//             }

//             _ => {/*more instructions*/ cpu.pc}
//         }
//     }
