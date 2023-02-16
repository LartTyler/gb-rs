use console::{style, StyledObject};
use gb_rs_asm::{containers::Pair, instructions::Instruction, operations::Operation};
use gb_rs_cpu::{registers::Registers, Cpu};

pub fn prefix_style() -> StyledObject<&'static str> {
    style(">>").black().bright()
}

pub fn title_style<'a>(title: &'a str) -> StyledObject<&'a str> {
    style(title).bold()
}

pub fn value_style<T>(value: T) -> StyledObject<T> {
    style(value).white().dim()
}

pub trait Show {
    fn show(&self);
}

impl Show for Cpu {
    fn show(&self) {
        println!("{}", title_style("Registers:"));
        self.registers.show();

        println!("interrupts: {}", value_style(self.interrupts_enabled));
    }
}

impl Show for Registers {
    fn show(&self) {
        let Self {
            a,
            b,
            c,
            d,
            e,
            h,
            l,
            flags,
            program_counter,
            stack_pointer,
            ..
        } = self;

        println!("a: {:>3}, flags: {}", value_style(a), value_style(flags));

        println!(
            "b: {:>3}, c: {:>3} || bc: {:>5}",
            value_style(b),
            value_style(c),
            value_style(self.get_pair(Pair::BC)),
        );

        println!(
            "d: {:>3}, e: {:>3} || de: {:>5}",
            value_style(d),
            value_style(e),
            value_style(self.get_pair(Pair::DE))
        );

        println!(
            "h: {:>3}, l: {:>3} || hl: {:>5}",
            value_style(h),
            value_style(l),
            value_style(self.get_pair(Pair::HL))
        );

        let pc = value_style(program_counter);
        let sp = value_style(stack_pointer);
        println!("pc: {pc:#06X} ({pc:>5}), sp: {sp:#06X} ({sp:>5})");
    }
}

impl Show for Operation {
    fn show(&self) {
        println!("{} {}", prefix_style(), self.kind);
    }
}

impl Show for Instruction {
    fn show(&self) {
        let prefix = prefix_style();

        println!("{prefix} {}", self.kind);
        println!("{prefix} w = {}, c = {}", self.width, self.cycles);
    }
}
