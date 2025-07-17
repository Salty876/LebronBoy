use gb_core::Emulator;

use std::env;
use std::fs::File;
use std::io::Read;


fn main() {
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
