pub struct MemoryBus {
    pub memory: [u8; 0xFFFF]
 }

impl MemoryBus{
    pub fn read_byte(&self, adress: u16) -> u8{
        return self.memory[adress as usize];
    }

    pub fn write_byte(&self, address: u16, byte: u8) {
        // empty
    }
}