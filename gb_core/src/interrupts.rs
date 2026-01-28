
pub enum Interrupt { 
    VBlank = 0, 
    LCDStat = 1,
    Timer = 2,
    Serial = 3, 
    Joypad = 4 
}
pub struct Interrupts {
    pub ime: bool,          // Interrupt Master Enable
    pub ime_scheduled: bool,       // IE Register
    pub ie: u8,     // IF Register
    pub i_flag: u8, // Interrupt Flag Register
}

impl Interrupt {
    pub fn bit(self) -> u8 { 1u8 << (self as u8) }

    pub fn vector(self) -> u16 {
        match self {
            Interrupt::VBlank  => 0x0040,
            Interrupt::LCDStat => 0x0048,
            Interrupt::Timer   => 0x0050,
            Interrupt::Serial  => 0x0058,
            Interrupt::Joypad  => 0x0060,
        }
    }
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts {
            ime: false,
            ime_scheduled: false,
            ie: 0,
            i_flag: 0,
        }
    }

    pub fn pending_mask(&self) -> u8 {
        (self.ie & self.i_flag) & 0x1F
    }

    pub fn has_pending(&self) -> bool {
        self.pending_mask() != 0
    }

    pub fn highest_priority(&self) -> Option<Interrupt> {
        let p = self.pending_mask();
        if p & 0x01 != 0 { return Some(Interrupt::VBlank); }
        if p & 0x02 != 0 { return Some(Interrupt::LCDStat); }
        if p & 0x04 != 0 { return Some(Interrupt::Timer); }
        if p & 0x08 != 0 { return Some(Interrupt::Serial); }
        if p & 0x10 != 0 { return Some(Interrupt::Joypad); }
        None
    }

    pub fn request(&mut self, intr: Interrupt) {
        self.i_flag |= intr.bit();
    }

    pub fn clear_request(&mut self, intr: Interrupt) {
        self.iflag &= !intr.bit();
    }
}