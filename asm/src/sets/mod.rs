use crate::{
    instructions::Instruction,
    operations::Operation,
    parse::{self, Opcode, Parse},
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

    pub fn base(&self, opcode: u8) -> Option<&Instruction> {
        self.base.get(opcode as usize).and_then(Option::as_ref)
    }

    pub fn extended(&self, opcode: u8) -> Option<&Instruction> {
        self.extended.get(opcode as usize).and_then(Option::as_ref)
    }

    pub fn parse<R: read::Read>(&self, data: &R, offset: u16) -> parse::Result<Operation> {
        use parse::Error::UnknownOpcode;

        let mut offset = offset;
        let instr = match data.read_byte(offset)? {
            0xCB => {
                // Extended instructions consume one extra byte, so we'll need to move the offset
                // forward one byte.
                offset += 1;

                let opcode = data.read_byte(offset)?;
                self.extended(opcode)
                    .ok_or(UnknownOpcode(Opcode::Extended(opcode)))
            }
            opcode => self.base(opcode).ok_or(UnknownOpcode(Opcode::Base(opcode))),
        }?;

        // Calls to Instruction::parse() should get an offset to the byte _after_ the instruction.
        instr.parse(data, offset + 1)
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
