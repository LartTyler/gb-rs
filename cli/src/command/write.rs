use super::*;

#[derive(Debug, Clone)]
pub struct WriteByteCommand {
    pub address: u16,
    pub value: u8,
}

impl FromArgs for WriteByteCommand {
    fn from_args(mut args: Vec<&str>) -> Result<Command> {
        let address: u16 = parse_arg(args.pop())?;
        let value: u8 = parse_arg(args.pop())?;

        Ok(Command::WriteByte(Self { address, value }))
    }
}

impl Display for WriteByteCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:04X} {}", self.address, self.value))
    }
}

#[derive(Debug, Clone)]
pub struct WriteWordCommand {
    pub address: u16,
    pub value: u16,
}

impl FromArgs for WriteWordCommand {
    fn from_args(mut args: Vec<&str>) -> Result<Command> {
        let address: u16 = parse_arg(args.pop())?;
        let value: u16 = parse_arg(args.pop())?;

        Ok(Command::WriteWord(Self { address, value }))
    }
}

impl Display for WriteWordCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:04X} {}", self.address, self.value))
    }
}
