use gb_core::Cpu;

#[test]
fn ld_a_d8_loads_immediate() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x0100;

    cpu.bus.write_byte(0x0100, 0x3E); // LD A, d8
    cpu.bus.write_byte(0x0101, 0x42);

    cpu.step();

    assert_eq!(cpu.regs.a(), 0x42);
    assert_eq!(cpu.pc, 0x0102);
}

#[test]
fn ld_b_d8_loads_immediate() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x0100;

    cpu.bus.write_byte(0x0100, 0x06); // LD B, d8
    cpu.bus.write_byte(0x0101, 0x99);

    cpu.step();

    assert_eq!(cpu.regs.b(), 0x99);
    assert_eq!(cpu.pc, 0x0102);
}

#[test]
fn ld_a_hl_reads_memory_byte() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x0100;

    cpu.regs.set_hl(0xC000);
    cpu.bus.write_byte(0xC000, 0xAB);
    cpu.bus.write_byte(0x0100, 0x7E); // LD A, (HL)

    cpu.step();

    assert_eq!(cpu.regs.a(), 0xAB);
    assert_eq!(cpu.pc, 0x0101);
}

#[test]
fn ld_hl_a_writes_memory_byte() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x0100;

    cpu.regs.set_hl(0xC000);
    cpu.regs.set_a(0x55);

    cpu.bus.write_byte(0x0100, 0x77); // LD (HL), A

    cpu.step();

    assert_eq!(cpu.bus.read_byte(0xC000), 0x55);
    assert_eq!(cpu.pc, 0x0101);
}

#[test]
fn ld_a_b_copies_register() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x0100;

    cpu.regs.set_b(0xDE);

    cpu.bus.write_byte(0x0100, 0x78); // LD A, B

    cpu.step();

    assert_eq!(cpu.regs.a(), 0xDE);
    assert_eq!(cpu.pc, 0x0101);
}

#[test]
fn ld_hl_b_writes_memory_from_register() {
    let mut cpu = Cpu::new();
    cpu.pc = 0x0100;

    cpu.regs.set_hl(0xC123);
    cpu.regs.set_b(0x7F);

    cpu.bus.write_byte(0x0100, 0x70); // LD (HL), B

    cpu.step();

    assert_eq!(cpu.bus.read_byte(0xC123), 0x7F);
    assert_eq!(cpu.pc, 0x0101);
}
