use clap::Parser;
use cli::{Cli, LimitUnit};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use gb_rs_asm::operations::jump::{AbsoluteJumpTarget, Jump};
use gb_rs_asm::operations::subroutine::{CallTarget, Subroutine};
use gb_rs_asm::operations::{Operation, OperationKind};
use gb_rs_asm::parse;
use gb_rs_asm::sets::Instructions;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

mod cli;

fn main() {
    let cli = Cli::parse();

    if let Some(limit) = cli.limit {
        non_interactive(cli, limit);
    } else {
        interactive(cli);
    }
}

struct Output;

impl Default for Output {
    fn default() -> Self {
        Self
    }
}

impl Output {
    pub fn op(&self, pc: u16, op: &Operation) {
        println!("{:#06X} | {}", console::style(pc).cyan().dim(), op.kind);
    }

    pub fn op_debug(&self, op: &Operation) {
        println!("         {:?}", console::style(op).white().dim());
    }

    pub fn info(&self, message: &str) {
        println!("[{}] {}", console::style("info").black().bright(), message);
    }

    pub fn warn(&self, message: &str) {
        println!("[{}] {}", console::style("warn").yellow().dim(), message);
    }

    pub fn lines(&self, lines: Vec<&str>) {
        for line in lines {
            println!("{}", line);
        }
    }
}

fn non_interactive(cli: Cli, limit: u16) {
    let mut disassembler = Disassembler::from_file(&cli.file);
    disassembler.jump(cli.offset.unwrap_or_default());

    let output = Output::default();
    let mut counter: u16 = 0;

    loop {
        let (pc, op) = match disassembler.next() {
            Ok(op) => op,
            Err(e) => panic!("{:?}", e),
        };

        output.op(pc, &op);

        match cli.limit_on {
            LimitUnit::Instructions => counter += 1,
            LimitUnit::Bytes => counter += op.width as u16,
        };

        if counter > limit {
            break;
        }
    }
}

fn interactive(cli: Cli) {
    let mut disassembler = Disassembler::from_file(&cli.file);
    disassembler.jump(cli.offset.unwrap_or_default());

    Interact::new(disassembler, cli.verbose > 0).start();
}

struct Interact {
    disassembler: Disassembler,
    output: Output,
    theme: ColorfulTheme,
    last_op: Option<Operation>,
    last_call_pc: Option<u16>,
    verbose: bool,
    stop: bool,
}

type CommandResult = Result<(), String>;

impl Interact {
    pub fn new(disassembler: Disassembler, verbose: bool) -> Self {
        let mut theme = ColorfulTheme::default();
        theme.prompt_prefix = console::style("".to_string());
        theme.prompt_suffix = console::style("> ".to_string()).black().bright();

        Self {
            disassembler,
            output: Default::default(),
            theme,
            last_op: None,
            last_call_pc: None,
            verbose,
            stop: false,
        }
    }

    pub fn start(&mut self) {
        self.stop = false;
        let mut default_command: String = Command::Next.as_str().to_owned();

        while !self.stop {
            let input: String = Input::with_theme(&self.theme)
                .default(default_command.clone())
                .interact_text()
                .unwrap();

            let mut args: Vec<&str> = input.split_whitespace().rev().collect();
            let command: Result<Command, ()> = args.pop().expect("empty input").parse();

            match command {
                Ok(command) => {
                    use Command::*;

                    let result = match command {
                        Quit => self.do_quit(),
                        Next => self.do_next(args),
                        Follow => self.do_follow(),
                        ToggleVerbose => self.do_toggle_verbose(),
                        Help => {
                            self.do_help();

                            // Skip to the start of the next loop, do_help() shouldn't be allowed
                            // to fail, and it doesn't make sense to update default_command to the
                            // help command.
                            continue;
                        }
                    };

                    if let Err(message) = result {
                        self.output.warn(&message);
                    } else {
                        default_command = input;
                    }
                }
                Err(_) => println!("Unrecognized command '{}'", input),
            };
        }
    }

    fn do_help(&mut self) {
        self.output.lines(vec![
            "==== help ====",
            "  next, n",
            "    advance one instruction forward",
            "  follow, f",
            "    follows either:",
            "      - a jump or call instruction, if the previous",
            "        instruction was a jump or call.",
            "      - a return instruction, if there was a matching",
            "        call instruction.",
            "  verbose, v",
            "    toggles verbose output",
            "  quit, q",
            "    terminates the application",
        ]);
    }

    fn do_quit(&mut self) -> CommandResult {
        self.stop = true;
        Ok(())
    }

    fn do_toggle_verbose(&mut self) -> CommandResult {
        self.verbose = !self.verbose;

        if self.verbose {
            self.output.info("verbose enabled");
        } else {
            self.output.info("verbose disabled");
        }

        Ok(())
    }

    fn do_next(&mut self, mut args: Vec<&str>) -> CommandResult {
        let next_count = if let Some(next_arg) = args.pop() {
            let Ok(next_count) = next_arg.parse::<u16>() else {
                return Err(format!("Invalid count provided to next: {} is not a number", next_arg));
            };

            next_count
        } else {
            1
        };

        for _ in 0..next_count {
            let (pc, op) = self.disassembler.next().unwrap();
            self.output.op(pc, &op);

            if self.verbose {
                self.output.op_debug(&op);
            }

            self.last_op.replace(op);
        }

        Ok(())
    }

    fn do_follow(&mut self) -> CommandResult {
        let Some(last_op) = &self.last_op else {
            return Err("cannot follow, no previous operation".to_owned());
        };

        match &last_op.kind {
            OperationKind::Jump(jump) => match jump {
                Jump::Absolute(jump) => {
                    let AbsoluteJumpTarget::DataPointer(pointer) = jump.target else {
                        return Err("cannot follow, registers are not simulated".to_owned());
                    };

                    self.disassembler.jump(**pointer);
                }
                Jump::Relative(jump) => self.disassembler.jump_relative(jump.offset.into()),
            },
            OperationKind::Subroutine(sub) => match sub {
                Subroutine::Call(call) => {
                    self.last_call_pc.replace(self.disassembler.program_counter);

                    match call.target {
                        CallTarget::Vector(vector) => self.disassembler.jump(*vector),
                        CallTarget::DataPointer(pointer) => self.disassembler.jump(**pointer),
                    };
                }
                Subroutine::Return(_) => {
                    let Some(last_call_pc) = self.last_call_pc.take() else {
                        return Err("cannot follow, RET instructions need a matching CALL".to_owned());
                    };

                    self.disassembler.program_counter = last_call_pc;
                }
            },
            _ => {
                return Err("cannot follow, previous operation is not a jump or call".to_owned());
            }
        };

        self.output.info(&format!(
            "now at ${:04X}",
            self.disassembler.program_counter
        ));

        self.do_next(vec![])
    }
}

#[derive(Debug)]
enum Command {
    Quit,
    Next,
    Follow,
    ToggleVerbose,
    Help,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matched = match s {
            "q" | "quit" => Self::Quit,
            "f" | "follow" => Self::Follow,
            "n" | "next" => Self::Next,
            "v" | "verbose" => Self::ToggleVerbose,
            "h" | "help" => Self::Help,
            _ => return Err(()),
        };

        Ok(matched)
    }
}

impl Command {
    pub fn as_str(&self) -> &'static str {
        use Command::*;

        match self {
            Quit => "quit",
            Follow => "follow",
            Next => "next",
            ToggleVerbose => "verbose",
            Help => "help",
        }
    }
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

    pub fn jump_relative(&mut self, offset: i8) {
        self.program_counter = self.program_counter.wrapping_add_signed(offset.into());
    }

    pub fn next(&mut self) -> parse::Result<(u16, Operation)> {
        let op = self.instructions.parse(&self.data, self.program_counter)?;
        let pc = self.program_counter;

        self.program_counter += op.width as u16;

        Ok((pc, op))
    }

    pub fn from_file(path: &Path) -> Self {
        let mut file = File::open(path).expect("could not open input file");

        let len = file.metadata().expect("could not get file metadata").len();
        let len: usize = len.try_into().expect("file too long");

        let mut data: Vec<u8> = Vec::with_capacity(len);
        file.read_to_end(&mut data)
            .expect("could not read input file");

        Self::new(data)
    }
}
