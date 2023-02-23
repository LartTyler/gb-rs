use gb_rs_asm::containers::Pair;
use gb_rs_core::cpu::{self, registers::FlagsRegister};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Widget,
};

macro_rules! draw_registers {
    (
        with $buf:ident, using $width:ident;
        $( ($x:expr, $y:expr) $label:expr => $value:expr; )*
    ) => {
        $(
            let spans = create_spans($label, $value);
            $buf.set_spans($x, $y, &spans, $width);
        )*
    };
}

pub struct Registers<'a> {
    registers: &'a cpu::registers::Registers,
}

impl<'a> Registers<'a> {
    pub fn new(registers: &'a cpu::registers::Registers) -> Self {
        Self { registers }
    }
}

impl<'a> Widget for Registers<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let column_width = area.width / 3;

        let top = area.top();
        let left = area.left();

        draw_registers!(
            with buf, using column_width;

            (left, top) "A" => self.registers.a;
            (left, top + 1) "B" => self.registers.b;
            (left, top + 2) "D" => self.registers.d;
            (left, top + 3) "H" => self.registers.h;
            (left, top + 4) "PC" => RegisterKind::Address(self.registers.program_counter);
        );

        let left = left + column_width;

        draw_registers!(
            with buf, using column_width;

            (left, top) "Flags" => &self.registers.flags;
            (left, top + 1) "C" => self.registers.c;
            (left, top + 2) "E" => self.registers.e;
            (left, top + 3) "L" => self.registers.l;
            (left, top + 4) "SP" => RegisterKind::Address(self.registers.stack_pointer);
        );

        let left = left + column_width;

        draw_registers!(
            with buf, using column_width;

            (left, top + 1) "BC" => self.registers.get_pair(Pair::BC);
            (left, top + 2) "DE" => self.registers.get_pair(Pair::DE);
            (left, top + 3) "HL" => self.registers.get_pair(Pair::HL);
        );
    }
}

fn create_spans<'a, T>(label: &str, value: T) -> Spans
where
    T: Into<RegisterKind<'a>>,
{
    let value = match value.into() {
        RegisterKind::Byte(n) => format!("{n:>3}"),
        RegisterKind::Word(n) => format!("{n:>5}"),
        RegisterKind::Address(n) => format!("${n:04X} ({n:>5})"),
        RegisterKind::Flags(flags) => format!("{flags}"),
    };

    vec![
        Span::raw(format!("{label:>2}: ")),
        Span::styled(value, Style::default().fg(Color::DarkGray)),
    ]
    .into()
}

enum RegisterKind<'a> {
    Byte(u8),
    Word(u16),
    Address(u16),
    Flags(&'a FlagsRegister),
}

impl From<u8> for RegisterKind<'_> {
    fn from(value: u8) -> Self {
        Self::Byte(value)
    }
}

impl From<u16> for RegisterKind<'_> {
    fn from(value: u16) -> Self {
        Self::Word(value)
    }
}

impl<'a> From<&'a FlagsRegister> for RegisterKind<'a> {
    fn from(value: &'a FlagsRegister) -> Self {
        Self::Flags(value)
    }
}
