use crate::{
    instructions::Instruction,
    operations::Operation,
    parse::{self, Parse},
    read,
};

pub type Set = [Option<Instruction>; 255];

pub struct Instructions {
    base: Set,
    extended: Set,
}

impl Default for Instructions {
    fn default() -> Self {
        Instruction::set()
    }
}

impl Instructions {
    pub fn new(base: Set, extended: Set) -> Self {
        Self { base, extended }
    }

    pub fn get(&self, opcode: u8) -> Option<&Instruction> {
        let set = if opcode == 0xCB {
            &self.extended
        } else {
            &self.base
        };

        set.get(opcode as usize).and_then(Option::as_ref)
    }

    pub fn parse<R: read::Read>(&self, data: &R, offset: u16) -> parse::Result<Operation> {
        let opcode = data.read_byte(offset)?;

        let instr = self.get(opcode);
        let instr = instr.ok_or(parse::Error::UnknownOpcode(opcode))?;

        instr.parse(data, offset)
    }
}

pub struct Builder {
    base: Set,
    extended: Set,
}

impl const Default for Builder {
    fn default() -> Self {
        Self::new([None; 255], [None; 255])
    }
}

impl Builder {
    pub const fn new(base: Set, extended: Set) -> Self {
        Self { base, extended }
    }

    pub const fn base<I>(&mut self, opcode: u8, instruction: I)
    where
        I: ~const Into<Instruction>,
    {
        self.base[opcode as usize].replace(instruction.into());
    }

    pub const fn extended<I>(&mut self, opcode: u8, instruction: I)
    where
        I: ~const Into<Instruction>,
    {
        self.extended[opcode as usize].replace(instruction.into());
    }

    pub const fn build(self) -> Instructions {
        Instructions {
            base: self.base,
            extended: self.extended,
        }
    }
}
