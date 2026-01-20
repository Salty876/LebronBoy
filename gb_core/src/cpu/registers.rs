use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const Z = 0b_1000_0000;
        const N = 0b_0100_0000;
        const H = 0b_0010_0000;
        const C = 0b_0001_0000;
    }
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    f: Flags,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0,
            f: Flags::empty(),
        }
    }

    pub fn af(&self) -> u16 { ((self.a as u16) << 8) | (self.f.bits() as u16) }
    pub fn set_af(&mut self, v: u16) {
        self.a = (v >> 8) as u8;
        self.f = Flags::from_bits_truncate(v as u8) & (Flags::Z | Flags::N | Flags::H | Flags::C);
    }

    pub fn bc(&self) -> u16 { ((self.b as u16) << 8) | (self.c as u16) }
    pub fn set_bc(&mut self, v: u16) { self.b = (v >> 8) as u8; self.c = v as u8; }

    pub fn de(&self) -> u16 { ((self.d as u16) << 8) | (self.e as u16) }
    pub fn set_de(&mut self, v: u16) { self.d = (v >> 8) as u8; self.e = v as u8; }

    pub fn hl(&self) -> u16 { ((self.h as u16) << 8) | (self.l as u16) }
    pub fn set_hl(&mut self, v: u16) { self.h = (v >> 8) as u8; self.l = v as u8; }

    // flag getters
    pub fn z(&self) -> bool { self.f.contains(Flags::Z) }
    pub fn n(&self) -> bool { self.f.contains(Flags::N) }
    pub fn h(&self) -> bool { self.f.contains(Flags::H) }
    pub fn c(&self) -> bool { self.f.contains(Flags::C) }

    // flag setters
    pub fn set_z(&mut self, on: bool) { self.f.set(Flags::Z, on); }
    pub fn set_n(&mut self, on: bool) { self.f.set(Flags::N, on); }
    pub fn set_h(&mut self, on: bool) { self.f.set(Flags::H, on); }
    pub fn set_c(&mut self, on: bool) { self.f.set(Flags::C, on); }
}
