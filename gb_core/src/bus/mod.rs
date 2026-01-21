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



#[cfg(test)]
mod tests {
    use super::Bus;

    #[test]
    fn read_write_byte_roundtrip() {
        let mut bus = Bus::new();
        bus.write_byte(0xC000, 0x12);
        assert_eq!(bus.read_byte(0xC000), 0x12);
    }

    #[test]
    fn read_write_word_little_endian() {
        let mut bus = Bus::new();
        bus.write_word(0xC000, 0xBEEF);
        assert_eq!(bus.read_byte(0xC000), 0xEF); // lo
        assert_eq!(bus.read_byte(0xC001), 0xBE); // hi
        assert_eq!(bus.read_word(0xC000), 0xBEEF);
    }

    #[test]
    fn word_wraparound_is_safe() {
        let mut bus = Bus::new();
        bus.write_byte(0xFFFF, 0xAA);
        bus.write_byte(0x0000, 0xBB);
        // reading a word at 0xFFFF wraps to 0x0000 for the high byte
        assert_eq!(bus.read_word(0xFFFF), 0xBBAA);
    }
}
