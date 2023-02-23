use super::*;

#[derive(Debug, Clone)]
pub struct ReadByteCommand {
    pub address: u16,
}

impl FromArgs for ReadByteCommand {
    fn from_args(mut args: Vec<&str>) -> Result<Command> {
        let address = parse_arg(args.pop())?;
        Ok(Command::ReadByte(Self { address }))
    }
}

impl Display for ReadByteCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("${:04X}", self.address))
    }
}

#[derive(Debug, Clone)]
pub struct ReadWordCommand {
    pub address: u16,
}

impl FromArgs for ReadWordCommand {
    fn from_args(mut args: Vec<&str>) -> Result<Command> {
        let address = parse_arg(args.pop())?;
        Ok(Command::ReadWord(Self { address }))
    }
}

impl Display for ReadWordCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("${:04X}", self.address))
    }
}
