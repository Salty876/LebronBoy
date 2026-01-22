

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ArithmeticTarget { A, B, C, D, E, H, L }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BigRegisterTarget { AF, BC, DE, HL }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JumpTest { NotZero, Zero, NotCarry, Carry, Always }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadByteTarget { A, B, C, D, E, H, L, HLI }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadByteSource { A, B, C, D, E, H, L, D8, HLI }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BigLoadByteTarget { AB, CD, DE, HL, SP }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BigLoadByteSource { AB, CD, DE, HL }

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
    SUB(ArithmeticTarget),
    JP(JumpTest),
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
        if prefixed { Self::decode_cb(byte) } else { Self::decode_base(byte) }
    }

    fn decode_cb(_byte: u8) -> Option<Self> {
        match _byte {
            // ADD r, d8 instructions here
            // 0xC6 => Some(Self::ADD(ArithmeticTarget::D8)),
            _ => None
        }
    }

    fn decode_base(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::NOP),
            0x76 => Some(Self::HALT),

            // ADD r, r instructions here
            0x80 => Some(Self::ADD(ArithmeticTarget::B)),
            0x81 => Some(Self::ADD(ArithmeticTarget::C)),
            0x82 => Some(Self::ADD(ArithmeticTarget::D)),
            0x83 => Some(Self::ADD(ArithmeticTarget::E)),
            0x84 => Some(Self::ADD(ArithmeticTarget::H)),
            0x85 => Some(Self::ADD(ArithmeticTarget::L)),
            0x87 => Some(Self::ADD(ArithmeticTarget::A)),   
            0x86 => Some(Self::ADD(ArithmeticTarget::A)),

            _ => None,
        }
    }
}


// pub enum ArithmeticTarget {
//     A, B, C, D, E, H, L,
// }

// pub enum BigRegisterTarget {
//     AF, BC, DE, HL
// }

// pub enum JumpTest {
//     NotZero,
//     Zero,
//     NotCarry,
//     Carry,
//     Always
// }
// pub enum LoadByteTarget {
//     A, B, C, D, E, H, L, HLI
// }

// pub enum LoadByteSource {
//     A, B, C, D, E, H, L, D8, HLI
// }

// pub enum LoadType {
//     R8ToR8(LoadByteTarget, LoadByteSource),
//     R8ToHL(LoadByteSource),
//     HLtoR8(LoadByteTarget),
//     N8toR8(ArithmeticTarget),
//     N16toR16(BigRegisterTarget),
//     R8toHL,
//     AtoR16,
//     N16ADtoA,
// }

// pub enum StackTargets{
//     AF, BC, DE, HL
// }

// pub enum Instruction {
//     ADD(ArithmeticTarget),
//     ADC(ArithmeticTarget),
//     SUB(ArithmeticTarget),
//     AND(ArithmeticTarget),
//     OR(ArithmeticTarget),
//     XOR(ArithmeticTarget),
//     INC(ArithmeticTarget),
//     DEC(ArithmeticTarget),
//     INC_HL,
//     DEC_HL,
//     INC_R16(BigRegisterTarget),
//     DEC_R16(BigRegisterTarget),
//     INC_SP,
//     DEC_SP,
//     JPCC(JumpTest),
//     JP,
//     JP_HL,
//     LD(LoadType),
//     PUSH(StackTargets),
//     POP(StackTargets),
//     CALL(JumpTest),
//     RET(JumpTest),
//     NOP,
//     SWAP(ArithmeticTarget),
//     HALT
// }

// impl Instruction{
//     pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
//         if prefixed{
//             Instruction::from_byte_prefixed(byte)
//         }else {
//             Instruction::from_byte_not_prefixed(byte)
//         }
       
//     }

//     pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
//         match byte {
//             _ => None
//         }
//     }

//     pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
//          match byte {
//             0x00 => Some(Instruction::NOP),
//             _ => None
//         }
//     }
// }