use rust_cpu_emu::compiler::Compiler;
use rust_cpu_emu::emulator::CpuEmulator;
use rust_cpu_emu::port::Port;
use rust_cpu_emu::register::Register;
use rust_cpu_emu::rom::Rom;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Invalid args. Usage: [command] [file_path]");
    }

    let f = BufReader::new(File::open(args.get(1).unwrap()).expect("file not found"));
    let operations = f.lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    let compiler = Compiler::new();
    let program = match compiler.compile(operations) {
        Ok(program) => program,
        Err(err) => panic!("{:?}", err),
    };

    let rom = Rom::new(program);
    let register = Register::new();
    let port = Port::new(0b0000, 0b0000);
    let mut emulator = CpuEmulator::with(register, port, rom);
    match emulator.exec() {
        Ok(_) => (),
        Err(err) => panic!("{:?}", err),
    }
}
