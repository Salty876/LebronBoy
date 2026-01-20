#[derive(Clone)]
pub struct Bus {
    pub memory: [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Self {
        Self { memory: [0; 0x10000] }
    }

    #[inline]
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    #[inline]
    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    #[inline]
    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    #[inline]
    pub fn write_word(&mut self, addr: u16, value: u16) {
        let lo = (value & 0x00FF) as u8;
        let hi = (value >> 8) as u8;
        self.write_byte(addr, lo);
        self.write_byte(addr.wrapping_add(1), hi);
    }
}
