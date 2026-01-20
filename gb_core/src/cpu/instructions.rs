

pub enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub enum BigRegisterTarget {
    AF, BC, DE, HL
}

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}
pub enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

pub enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

pub enum LoadType {
    R8ToR8(LoadByteTarget, LoadByteSource),
    R8ToHL(LoadByteSource),
    HLtoR8(LoadByteTarget),
    N8toR8(ArithmeticTarget),
    N16toR16(BigRegisterTarget),
    R8toHL,
    AtoR16,
    N16ADtoA,
}

pub enum StackTargets{
    AF, BC, DE, HL
}

pub enum Instruction {
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
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed{
            Instruction::from_byte_prefixed(byte)
        }else {
            Instruction::from_byte_not_prefixed(byte)
        }
       
    }

    pub fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            _ => None
        }
    }

    pub fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
         match byte {
            0x00 => Some(Instruction::NOP),
            _ => None
        }
    }
}