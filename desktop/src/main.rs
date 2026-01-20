use gb_core::cpu::Cpu;


pub fn main() {
    // Load a ROM file (replace "path/to/rom.gb" with an actual file path)
    let mut cpu = Cpu::new();
    // cpu.load_rom(&rom_bytes);
    cpu.run_steps(10_000);
    }


