use clap::Parser;
use tracing::Level;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RawArgs {
    #[arg(long, short = 'v', global = true, action = clap::ArgAction::Count)]
    verbosity: u8,
}

#[derive(Debug)]
pub(crate) struct ParsedArgs {
    pub log_level: tracing::Level,
}

impl ParsedArgs {
    pub fn parse_raw() -> Self {
        let args: RawArgs = clap::Parser::parse();

        let log_level = match args.verbosity {
            0 => Level::INFO,
            1 => Level::DEBUG,
            _ => Level::TRACE,
        };

        ParsedArgs { log_level }
    }
}
