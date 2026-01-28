use std::result;

use super::{Cpu};
use super::instructions::*;


// Helper functions for conditional and getting targets

fn condition(test: JumpTest, cpu: &Cpu) -> bool {
    match test {
        JumpTest::NotZero => !cpu.regs.get_z(),
        JumpTest::Zero => cpu.regs.get_z(),
        JumpTest::NotCarry => !cpu.regs.get_carry(),
        JumpTest::Carry => cpu.regs.get_carry(),
        JumpTest::Always => true,
    }
}

fn read_u8_target(cpu: &mut Cpu, t: ArithmeticTarget) -> u8 {
    match t {
        ArithmeticTarget::A => cpu.regs.a(),
        ArithmeticTarget::B => cpu.regs.b(),
        ArithmeticTarget::C => cpu.regs.c(),
        ArithmeticTarget::D => cpu.regs.d(),
        ArithmeticTarget::E => cpu.regs.e(),
        ArithmeticTarget::H => cpu.regs.h(),
        ArithmeticTarget::L => cpu.regs.l(),
        ArithmeticTarget::HLI => cpu.bus.read_byte(cpu.regs.get_hl()),
        ArithmeticTarget::D8 => cpu.next_byte(),
    }
}







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

        Instruction::JR(test) => {
            let cond =condition(test, cpu);
            let offset = cpu.next_byte() as i8 as i16;
            if cond {
                let target = cpu.pc.wrapping_add(2).wrapping_add(offset as u16);
                target
            } else {
                cpu.pc.wrapping_add(2)
            }
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

        Instruction::ADD(target) => {
            let value = match target {
                ArithmeticTarget::A => cpu.regs.a(),
                ArithmeticTarget::B => cpu.regs.b(),
                ArithmeticTarget::C => cpu.regs.c(),
                ArithmeticTarget::D => cpu.regs.d(),
                ArithmeticTarget::E => cpu.regs.e(),
                ArithmeticTarget::H => cpu.regs.h(),
                ArithmeticTarget::L => cpu.regs.l(),
                ArithmeticTarget::HLI => cpu.bus.read_byte(cpu.regs.get_hl()),
                ArithmeticTarget::D8 => cpu.next_byte(),
            };
            let new_value = cpu.add(value);
            cpu.regs.set_a(new_value);
            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }

        Instruction::ADC(target) => {
            let carry = if cpu.regs.get_carry() { 1 } else { 0 };

            let value = read_u8_target(cpu, target);

            let a = cpu.regs.a();

            let result = a.wrapping_add(value).wrapping_add(carry);
            cpu.regs.set_a(result);

            // Set flags
            cpu.regs.set_z(result == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_hc(((a & 0x0F) + (value & 0x0F) + carry) > 0x0F);
            cpu.regs.set_carry((a as u16) + (value as u16) + (carry as u16) > 0xFF);

            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1)
            }

        }

        Instruction::ADD16(target) => {
            let value = match target {
                Add16Target::BC => cpu.regs.get_bc(),
                Add16Target::DE => cpu.regs.get_de(),
                Add16Target::HL => cpu.regs.get_hl(),
                Add16Target::SP => cpu.sp,
            };
            let new_val = cpu.add_hl_rr(value);
            cpu.regs.set_hl(new_val);
            cpu.pc.wrapping_add(1)
        }

        Instruction::SUB(target) => {
            let value = match target {
                ArithmeticTarget::A => cpu.regs.a(),
                ArithmeticTarget::B => cpu.regs.b(),
                ArithmeticTarget::C => cpu.regs.c(),
                ArithmeticTarget::D => cpu.regs.d(),
                ArithmeticTarget::E => cpu.regs.e(),
                ArithmeticTarget::H => cpu.regs.h(),
                ArithmeticTarget::L => cpu.regs.l(),
                ArithmeticTarget::HLI => cpu.bus.read_byte(cpu.regs.get_hl()),
                ArithmeticTarget::D8 => cpu.next_byte(),
            };
            let new_value = cpu.sub(value);
            cpu.regs.set_a(new_value);
            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }

        Instruction::SBC(target) => {
            let carry = if cpu.regs.get_carry() { 1 } else { 0 };

            let value = read_u8_target(cpu, target);

            let a = cpu.regs.a();

            let result = a.wrapping_sub(value).wrapping_sub(carry);
            cpu.regs.set_a(result);

            // Set flags
            cpu.regs.set_z(result == 0);
            cpu.regs.set_n(true);
            cpu.regs.set_hc((a & 0x0F) < (value & 0x0F) + carry);
            cpu.regs.set_carry((a as u16) < (value as u16) + (carry as u16));

            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1)
            }

        }

        Instruction::INC(target) => {
            match target {
                ArithmeticTarget::B => {
                    let value = cpu.regs.b();
                    let (new_value, did_overflow) = value.overflowing_add(1);
                    cpu.regs.set_b(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(false);
                    cpu.regs.set_hc((value & 0x0F) + 1 > 0x0F);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::C => {
                    let value = cpu.regs.c();
                    let (new_value, did_overflow) = value.overflowing_add(1);
                    cpu.regs.set_c(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(false);
                    cpu.regs.set_hc((value & 0x0F) + 1 > 0x0F);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::D => {
                    let value = cpu.regs.d();
                    let (new_value, did_overflow) = value.overflowing_add(1);
                    cpu.regs.set_d(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(false);
                    cpu.regs.set_hc((value & 0x0F) + 1 > 0x0F);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::E => {
                    let value = cpu.regs.e();
                    let (new_value, did_overflow) = value.overflowing_add(1);
                    cpu.regs.set_e(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(false);
                    cpu.regs.set_hc((value & 0x0F) + 1 > 0x0F);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::H => {
                    let value = cpu.regs.h();
                    let (new_value, did_overflow) = value.overflowing_add(1);
                    cpu.regs.set_h(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(false);
                    cpu.regs.set_hc((value & 0x0F) + 1 > 0x0F);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::L => {
                    let value = cpu.regs.l();
                    let (new_value, did_overflow) = value.overflowing_add(1);
                    cpu.regs.set_l(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(false);
                    cpu.regs.set_hc((value & 0x0F) + 1 > 0x0F);

                    cpu.pc.wrapping_add(1)
                },
                ArithmeticTarget::HLI => {
                    let addr = cpu.regs.get_hl();
                    let value = cpu.bus.read_byte(addr);
                    let (new_value, did_overflow) = value.overflowing_add(1);
                    cpu.bus.write_byte(addr, new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(false);
                    cpu.regs.set_hc((value & 0x0F) + 1 > 0x0F);

                    cpu.pc.wrapping_add(1)
                },


                _ => {cpu.pc} // To be implemented
            }
        }

        Instruction::DEC(target) => {
            match target {
                ArithmeticTarget::B => {
                    let value = cpu.regs.b();
                    let (new_value, did_overflow) = value.overflowing_sub(1);
                    cpu.regs.set_b(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(true);
                    cpu.regs.set_hc((value & 0x0F) == 0);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::C => {
                    let value = cpu.regs.c();
                    let (new_value, did_overflow) = value.overflowing_sub(1);
                    cpu.regs.set_c(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(true);
                    cpu.regs.set_hc((value & 0x0F) == 0);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::D => {
                    let value = cpu.regs.d();
                    let (new_value, did_overflow) = value.overflowing_sub(1);
                    cpu.regs.set_d(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(true);
                    cpu.regs.set_hc((value & 0x0F) == 0);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::E => {
                    let value = cpu.regs.e();
                    let (new_value, did_overflow) = value.overflowing_sub(1);
                    cpu.regs.set_e(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(true);
                    cpu.regs.set_hc((value & 0x0F) == 0);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::H => {
                    let value = cpu.regs.h();
                    let (new_value, did_overflow) = value.overflowing_sub(1);
                    cpu.regs.set_h(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(true);
                    cpu.regs.set_hc((value & 0x0F) == 0);

                    cpu.pc.wrapping_add(1)
                },

                ArithmeticTarget::L => {
                    let value = cpu.regs.l();
                    let (new_value, did_overflow) = value.overflowing_sub(1);
                    cpu.regs.set_l(new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(true);
                    cpu.regs.set_hc((value & 0x0F) == 0);

                    cpu.pc.wrapping_add(1)
                },
                ArithmeticTarget::HLI => {
                    let addr = cpu.regs.get_hl();
                    let value = cpu.bus.read_byte(addr);
                    let (new_value, did_overflow) = value.overflowing_sub(1);
                    cpu.bus.write_byte(addr, new_value);

                    // set flags
                    cpu.regs.set_z(new_value == 0);
                    cpu.regs.set_n(true);
                    cpu.regs.set_hc((value & 0x0F) == 0x00);

                    cpu.pc.wrapping_add(1)
                },


                _ => {cpu.pc} // To be implemented
            }
        }

        Instruction::INC16(target) => {
            match target {
                Add16Target::BC => {
                    let value = cpu.regs.get_bc();
                    let new_value = value.wrapping_add(1);
                    cpu.regs.set_bc(new_value);
                    cpu.pc.wrapping_add(1)
                },

                Add16Target::DE => {
                    let value = cpu.regs.get_de();
                    let new_value = value.wrapping_add(1);
                    cpu.regs.set_de(new_value);
                    cpu.pc.wrapping_add(1)
                },

                Add16Target::HL => {
                    let value = cpu.regs.get_hl();
                    let new_value = value.wrapping_add(1);
                    cpu.regs.set_hl(new_value);
                    cpu.pc.wrapping_add(1)
                },

                Add16Target::SP => {
                    let value = cpu.sp;
                    let new_value = value.wrapping_add(1);
                    cpu.sp = new_value;
                    cpu.pc.wrapping_add(1)
                },
            }
        }

        Instruction::DEC16(target) => {
            match target {
                Add16Target::BC => {
                    let value = cpu.regs.get_bc();
                    let new_value = value.wrapping_sub(1);
                    cpu.regs.set_bc(new_value);
                    cpu.pc.wrapping_add(1)
                },

                Add16Target::DE => {
                    let value = cpu.regs.get_de();
                    let new_value = value.wrapping_sub(1);
                    cpu.regs.set_de(new_value);
                    cpu.pc.wrapping_add(1)
                },

                Add16Target::HL => {
                    let value = cpu.regs.get_hl();
                    let new_value = value.wrapping_sub(1);
                    cpu.regs.set_hl(new_value);
                    cpu.pc.wrapping_add(1)
                },

                Add16Target::SP => {
                    let value = cpu.sp;
                    let new_value = value.wrapping_sub(1);
                    cpu.sp = new_value;
                    cpu.pc.wrapping_add(1)
                },
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
                        BigLoadByteTarget::AF => cpu.regs.set_af(value),
                        BigLoadByteTarget::BC => cpu.regs.set_bc(value),
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
                    cpu.regs.set_carry(((sp_low & 0xFF) + ((offset_up as u16) & 0xFF)) > 0xFF);
                    cpu.regs.set_hc(((sp_low & 0x0F) + ((offset_up as u16) & 0x0F)) > 0x0F);


                    cpu.pc.wrapping_add(2)

                },

                _ => {cpu.pc}
        }
        // Keep migrating your existing ADD/SUB/LD/PUSH/POP here next.
        
    }

        Instruction::XOR(target) => {
            let value = read_u8_target(cpu, target);
            let result = cpu.regs.a() ^ value;
            cpu.regs.set_a(result);

            // Set flags
            cpu.regs.set_z(result == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_hc(false);
            cpu.regs.set_carry(false);

            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }

        Instruction::CP(target) => {
            let value = read_u8_target(cpu, target);
            // Sets flags
            cpu.sub(value);

            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }

        Instruction::AND(target) => {
            let value = read_u8_target(cpu, target);

            let result = cpu.regs.a() & value;
            cpu.regs.set_a(result);

            // Set flags
            cpu.regs.set_z(result == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_hc(true);
            cpu.regs.set_carry(false);

            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }

        Instruction::OR(target) => {
            let value = read_u8_target(cpu, target);

            let result = cpu.regs.a() | value;
            cpu.regs.set_a(result);

            // Set flags
            cpu.regs.set_z(result == 0);
            cpu.regs.set_n(false);
            cpu.regs.set_hc(false);
            cpu.regs.set_carry(false);

            match target {
                ArithmeticTarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }
    _ => cpu.pc}
}


