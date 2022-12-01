use clap::Parser;
use cli::Cli;
use gb_rs_asm::sets::Instructions;
use std::fs::File;
use std::io::{self, Read, Write};

mod cli;

fn main() {
    let cli = Cli::parse();
    let mut file = File::open(cli.file).expect("could not open input file");

    let len = file.metadata().expect("could not get file metadata").len();
    let len: usize = len.try_into().expect("file too long");

    let mut data: Vec<u8> = Vec::with_capacity(len);
    file.read_to_end(&mut data)
        .expect("could not read input file");

    let instructions = Instructions::default();
    let mut stack_pointer = 0x100; // Skip cart header

    loop {
        let instr = match instructions.parse(&data, stack_pointer) {
            Ok(i) => i,
            Err(e) => panic!("{:?}", e),
        };

        print!("{:#06X} | {}", stack_pointer, instr);
        io::stdout().flush().expect("could not flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading stdin");

        stack_pointer += 1;
    }
}
