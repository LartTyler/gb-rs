use crate::containers::Cycles;
use crate::instructions::{Instruction, InstructionKind};
use crate::operations::Operation;
use crate::parse::{self, Opcode};
use crate::read;

pub type Set = [Option<Instruction>; 256];

#[derive(Debug)]
pub struct Instructions {
    base: Set,
    extended: Set,
}

impl Default for Instructions {
    fn default() -> Self {
        InstructionKind::set()
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

    #[cfg(debug_assertions)]
    pub fn base_len(&self) -> usize {
        self.base.iter().filter(|i| i.is_some()).count()
    }

    #[cfg(debug_assertions)]
    pub fn extended_len(&self) -> usize {
        self.extended.iter().filter(|i| i.is_some()).count()
    }
}

pub struct Builder {
    base: Set,
    extended: Set,
}

impl const Default for Builder {
    fn default() -> Self {
        Self::new([None; 256], [None; 256])
    }
}

impl Builder {
    pub const fn new(base: Set, extended: Set) -> Self {
        Self { base, extended }
    }

    pub const fn base<I, C>(&mut self, opcode: u8, instruction: I, width: u8, cycles: C)
    where
        I: ~const Into<InstructionKind>,
        C: ~const Into<Cycles>,
    {
        debug_assert!(
            self.base[opcode as usize].is_none(),
            "aborting due to duplicate base opcode; you'll have to check the backtrace to find out where this is happening"
        );

        self.base[opcode as usize].replace(Instruction {
            kind: instruction.into(),
            width,
            cycles: cycles.into(),
        });
    }

    pub const fn extended<I, C>(&mut self, opcode: u8, instruction: I, width: u8, cycles: C)
    where
        I: ~const Into<InstructionKind>,
        C: ~const Into<Cycles>,
    {
        debug_assert!(
            self.extended[opcode as usize].is_none(),
            "aborting due to duplicate extended opcode; you'll have to check the backtrace to find out where this is happening"
        );

        self.extended[opcode as usize].replace(Instruction {
            kind: instruction.into(),
            width,
            cycles: cycles.into(),
        });
    }

    pub const fn build(self) -> Instructions {
        Instructions {
            base: self.base,
            extended: self.extended,
        }
    }
}
