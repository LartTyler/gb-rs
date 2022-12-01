use clap::Parser;
use cli::Cli;
use gb_rs_asm::operations::Operation;
use gb_rs_asm::parse;
use gb_rs_asm::sets::Instructions;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

mod cli;

fn main() {
    let cli = Cli::parse();

    if let Some(limit) = cli.limit {
        non_interactive(cli, limit);
    } else {
        todo!("interactive mode not yet implemented");
    }
}

fn non_interactive(cli: Cli, limit: u16) {
    let mut disassembler = Disassembler::new(read_cart(&cli.file));
    disassembler.jump(cli.offset.unwrap_or_default());

    let printer = Printer::new(cli.verbose);

    for _ in 0..limit {
        let (pc, op) = match disassembler.next() {
            Ok(op) => op,
            Err(e) => panic!("{:?}", e),
        };

        printer.write((pc, &op));
    }
}

fn read_cart(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).expect("could not open input file");

    let len = file.metadata().expect("could not get file metadata").len();
    let len: usize = len.try_into().expect("file too long");

    let mut data: Vec<u8> = Vec::with_capacity(len);
    file.read_to_end(&mut data)
        .expect("could not read input file");

    data
}

struct Disassembler {
    data: Vec<u8>,
    instructions: Instructions,
    program_counter: u16,
}

impl Disassembler {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            instructions: Instructions::default(),
            program_counter: 0,
        }
    }

    pub fn jump(&mut self, pc: u16) {
        self.program_counter = pc;
    }

    pub fn next(&mut self) -> parse::Result<(u16, Operation)> {
        let op = self.instructions.parse(&self.data, self.program_counter)?;
        let pc = self.program_counter;

        self.program_counter += op.width as u16;

        Ok((pc, op))
    }
}

struct Printer {
    verbose: u8,
}

impl Printer {
    pub fn new(verbose: u8) -> Self {
        Self { verbose }
    }

    pub fn write<P: Printable>(&self, item: P) {
        item.write(self.verbose);
        println!("");

        io::stdout().flush().expect("could not flush stdout");
    }
}

trait Printable {
    fn write(&self, verbose: u8);
}

impl Printable for Operation {
    fn write(&self, verbose: u8) {
        if verbose == 0 {
            print!("{}", self.kind);
        } else {
            print!("{:?}", self.kind);
        }
    }
}

impl Printable for (u16, &Operation) {
    fn write(&self, verbose: u8) {
        print!("{:#06X} | ", self.0);
        self.1.write(verbose);
    }
}
