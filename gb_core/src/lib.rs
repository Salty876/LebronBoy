use crate::gb::cpu::Cpu;
use std::fs::File;
use std::io::Read;
use std::env;

mod gb;


pub struct Emulator { 
    pub cpu: gb::cpu::Cpu
}

impl Emulator {
    pub fn new() -> Emulator {
        let cpu: Cpu = Cpu::new();

        Emulator { cpu: cpu }
    }
    pub fn load(&mut self, data: &[u8]){
        let start = 0 as usize;
        let end = (start as usize) + 0x100;
        self.cpu.bus.memory[start..end].copy_from_slice(data);
    }

}



fn test() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2{
        println!("PATH/TO/GAME FN");
    }

    let mut emu: Emulator = Emulator::new();

    let mut rom = File::open(&args[1]).expect("unable to ppen file");
    let mut buffer: Vec<_> = Vec::new();
    rom.read_to_end(&mut buffer).expect("CANNOT OPEN TS NIGGA");
    emu.load(&buffer);

    loop {
        emu.cpu.step();
    }



}
