use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rlox", about = "Inspect Lox source files")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Display a file's source text.
    Source {
        /// File whose text should be displayed.
        file: String,
    },
}

fn main() -> ExitCode {
    let filename = match Cli::parse().command {
        Command::Source { file } => file,
    };
    let source = match std::fs::read_to_string(&filename) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("rlox: could not read '{filename}': {error}");
            return ExitCode::FAILURE;
        }
    };
    print!("{source}");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::{Cli, Command};
    use clap::Parser;

    #[test]
    fn accepts_source_command() {
        let cli = Cli::try_parse_from(["rlox", "source", "hello.lox"])
            .expect("source with a filename should parse");

        match cli.command {
            Command::Source { file } => assert_eq!(file, "hello.lox"),
        }
    }

    #[test]
    fn requires_known_command() {
        assert!(Cli::try_parse_from(["rlox"]).is_err());
        assert!(Cli::try_parse_from(["rlox", "hello.lox"]).is_err());
        assert!(Cli::try_parse_from(["rlox", "unknown", "hello.lox"]).is_err());
    }

    #[test]
    fn rejects_missing_filename() {
        assert!(Cli::try_parse_from(["rlox", "source"]).is_err());
    }

    #[test]
    fn rejects_extra_filenames() {
        assert!(Cli::try_parse_from(["rlox", "source", "hello.lox", "other.lox"]).is_err());
    }

    #[test]
    fn help_is_a_successful_request() {
        let help = Cli::try_parse_from(["rlox", "--help"])
            .err()
            .expect("help should stop parsing");

        assert_eq!(help.kind(), clap::error::ErrorKind::DisplayHelp);
        assert_eq!(help.exit_code(), 0);
    }
}
