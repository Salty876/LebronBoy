

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArithmeticTarget { A, B, C, D, E, H, L, HLI, D8 }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Add16Target { BC, DE, HL, SP }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BigRegisterTarget { AF, BC, DE, HL }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JumpTest { NotZero, Zero, NotCarry, Carry, Always }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadByteTarget { A, B, C, D, E, H, L, HLI }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadByteSource { A, B, C, D, E, H, L, D8, HLI }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BigLoadByteTarget { AF, BC, DE, HL, SP }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BigLoadByteSource { AF, BC, DE, HL }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadType {
    R8ToR8(LoadByteTarget, LoadByteSource),
    D16toR16(BigLoadByteTarget),
    HLtoSP,
    SPtoA16,
    R16toSP(BigRegisterTarget),
    SP8toHL,

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StackTargets { AF, BC, DE, HL }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    ADD16(Add16Target),
    SUB(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    JP(JumpTest),
    JR(JumpTest),
    LD(LoadType),
    PUSH(StackTargets),
    POP(StackTargets),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    HALT,
}

impl Instruction {
    pub fn decode(byte: u8, prefixed: bool) -> Option<Self> {
        println!("{:02X}", byte);
        if prefixed { Self::decode_cb(byte) } else { Self::decode_base(byte) }
    }

    fn decode_cb(_byte: u8) -> Option<Self> {
        match _byte {
            // ADD r, d8 instructions here
            // 0xC6 => Some(Self::ADD(ArithmeticTarget::D8)),
            0xC9 => Some(Self::RET(JumpTest::Always)),
            _ => None
        }
    }

    fn decode_base(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::NOP),
            0x76 => Some(Self::HALT),
            0xCD => Some(Self::CALL(JumpTest::Always)),
            0xC4 => Some(Self::CALL(JumpTest::NotZero)),
            0xCC => Some(Self::CALL(JumpTest::Zero)),
            0xD4 => Some(Self::CALL(JumpTest::NotCarry)),
            0xDC => Some(Self::CALL(JumpTest::Carry)),

            // RET cc
            0xC0 => Some(Self::RET(JumpTest::NotZero)),
            0xC8 => Some(Self::RET(JumpTest::Zero)),
            0xD0 => Some(Self::RET(JumpTest::NotCarry)),
            0xD8 => Some(Self::RET(JumpTest::Carry)),
            0xC9 => Some(Self::RET(JumpTest::Always)),

            // JP a16 / JP cc,a16
            0xC3 => Some(Self::JP(JumpTest::Always)),
            0xC2 => Some(Self::JP(JumpTest::NotZero)),
            0xCA => Some(Self::JP(JumpTest::Zero)),
            0xD2 => Some(Self::JP(JumpTest::NotCarry)),
            0xDA => Some(Self::JP(JumpTest::Carry)),

            // ADD r, r instructions here
            0x80 => Some(Self::ADD(ArithmeticTarget::B)),
            0x81 => Some(Self::ADD(ArithmeticTarget::C)),
            0x82 => Some(Self::ADD(ArithmeticTarget::D)),
            0x83 => Some(Self::ADD(ArithmeticTarget::E)),
            0x84 => Some(Self::ADD(ArithmeticTarget::H)),
            0x85 => Some(Self::ADD(ArithmeticTarget::L)),
            0x86 => Some(Self::ADD(ArithmeticTarget::HLI)),
            0x87 => Some(Self::ADD(ArithmeticTarget::A)),
            0xC6 => Some(Self::ADD(ArithmeticTarget::D8)),

            // ADD HL, rr
            0x09 => Some(Self::ADD16(Add16Target::BC)),
            0x19 => Some(Self::ADD16(Add16Target::DE)),
            0x29 => Some(Self::ADD16(Add16Target::HL)),
            0x39 => Some(Self::ADD16(Add16Target::SP)),

            // LD r, r / LD r, (HL) / LD (HL), r / LD r, d8 / LD (HL), d8
            0x40 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::B))),
            0x41 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::C))),
            0x42 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::D))),
            0x43 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::E))),
            0x44 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::H))),
            0x45 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::L))),
            0x46 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::HLI))),
            0x47 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::A))),
            0x48 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::B))),
            0x49 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::C))),
            0x4A => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::D))),
            0x4B => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::E))),
            0x4C => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::H))),
            0x4D => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::L))),
            0x4E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::HLI))),
            0x4F => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::A))),
            0x50 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::B))),
            0x51 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::C))),
            0x52 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::D))),
            0x53 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::E))),
            0x54 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::H))),
            0x55 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::L))),
            0x56 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::HLI))),
            0x57 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::A))),
            0x58 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::B))),
            0x59 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::C))),
            0x5A => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::D))),
            0x5B => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::E))),
            0x5C => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::H))),
            0x5D => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::L))),
            0x5E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::HLI))),
            0x5F => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::A))),
            0x60 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::B))),
            0x61 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::C))),
            0x62 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::D))),
            0x63 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::E))),
            0x64 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::H))),
            0x65 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::L))),
            0x66 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::HLI))),
            0x67 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::A))),
            0x68 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::B))),
            0x69 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::C))),
            0x6A => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::D))),
            0x6B => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::E))),
            0x6C => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::H))),
            0x6D => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::L))),
            0x6E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::HLI))),
            0x6F => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::A))),
            0x70 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::B))),
            0x71 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::C))),
            0x72 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::D))),
            0x73 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::E))),
            0x74 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::H))),
            0x75 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::L))),
            0x77 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::A))),
            0x78 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::B))),
            0x79 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::C))),
            0x7A => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::D))),
            0x7B => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::E))),
            0x7C => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::H))),
            0x7D => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::L))),
            0x7E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::HLI))),
            0x7F => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::A))),
            0x06 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::B, LoadByteSource::D8))),
            0x0E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::C, LoadByteSource::D8))),
            0x16 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::D, LoadByteSource::D8))),
            0x1E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::E, LoadByteSource::D8))),
            0x26 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::H, LoadByteSource::D8))),
            0x2E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::L, LoadByteSource::D8))),
            0x36 => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::HLI, LoadByteSource::D8))),
            0x3E => Some(Self::LD(LoadType::R8ToR8(LoadByteTarget::A, LoadByteSource::D8))),

            // LD rr, d16
            0x01 => Some(Self::LD(LoadType::D16toR16(BigLoadByteTarget::BC))),
            0x11 => Some(Self::LD(LoadType::D16toR16(BigLoadByteTarget::DE))),
            0x21 => Some(Self::LD(LoadType::D16toR16(BigLoadByteTarget::HL))),
            0x31 => Some(Self::LD(LoadType::D16toR16(BigLoadByteTarget::SP))),

            // LD SP, HL
            0xF9 => Some(Self::LD(LoadType::HLtoSP)),

            // LD (a16), SP
            0x08 => Some(Self::LD(LoadType::SPtoA16)),

            // LD HL, SP+e8
            0xF8 => Some(Self::LD(LoadType::SP8toHL)),

            // JR e8 / JR cc,e8
            0x18 => Some(Self::JR(JumpTest::Always)),
            0x20 => Some(Self::JR(JumpTest::NotZero)),
            0x28 => Some(Self::JR(JumpTest::Zero)),
            0x30 => Some(Self::JR(JumpTest::NotCarry)),
            0x38 => Some(Self::JR(JumpTest::Carry)),

            // SUB r, r instructions here
            0x90 => Some(Self::SUB(ArithmeticTarget::B)),
            0x91 => Some(Self::SUB(ArithmeticTarget::C)),
            0x92 => Some(Self::SUB(ArithmeticTarget::D)),
            0x93 => Some(Self::SUB(ArithmeticTarget::E)),
            0x94 => Some(Self::SUB(ArithmeticTarget::H)),
            0x95 => Some(Self::SUB(ArithmeticTarget::L)),
            0x96 => Some(Self::SUB(ArithmeticTarget::HLI)),
            0x97 => Some(Self::SUB(ArithmeticTarget::A)),
            0xD6 => Some(Self::SUB(ArithmeticTarget::D8)),
            _ => None,
        }
    }
}


