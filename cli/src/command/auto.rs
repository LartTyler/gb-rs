use super::*;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AutoCommand {
    pub tick_rate: Duration,
}

impl FromArgs for AutoCommand {
    fn from_args(mut args: Vec<&str>) -> Result<Command> {
        let tick_rate = parse_time(args.pop())?;
        Ok(Command::Auto(Self { tick_rate }))
    }
}

impl Display for AutoCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#}s", self.tick_rate.as_secs_f64()))
    }
}

fn parse_time(input: Option<&str>) -> Result<Duration> {
    let Some(input) = input else {
        return Err(Error::MissingArgument);
    };

    let (fragment, ctor): (&str, fn(u64) -> Duration) = if input.len() >= 2 {
        if let Some(stripped) = input.strip_suffix("ns") {
            (stripped, Duration::from_nanos)
        } else if let Some(stripped) = input.strip_suffix("us") {
            (stripped, Duration::from_micros)
        } else if let Some(stripped) = input.strip_suffix("ms") {
            (stripped, Duration::from_millis)
        } else if let Some(stripped) = input.strip_suffix('s') {
            (stripped, Duration::from_secs)
        } else {
            (input, Duration::from_millis)
        }
    } else {
        (input, Duration::from_millis)
    };

    let value: u64 = fragment
        .trim_end()
        .parse()
        .map_err(|_| Error::InvalidArgument)?;

    Ok(ctor(value))
}
