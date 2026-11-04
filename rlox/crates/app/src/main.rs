use std::process::ExitCode;

use clap::{Parser, Subcommand};
use rlox_syntax::{SyntaxKind, scan};

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
    /// Display a file's tokens and source ranges.
    Tokens {
        /// File whose tokens should be displayed.
        file: String,
    },
}

fn main() -> ExitCode {
    let command = Cli::parse().command;
    let filename = match &command {
        Command::Source { file } | Command::Tokens { file } => file,
    };
    let source = match std::fs::read_to_string(filename) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("rlox: could not read '{filename}': {error}");
            return ExitCode::FAILURE;
        }
    };
    match command {
        Command::Source { .. } => print!("{source}"),
        Command::Tokens { .. } => {
            let mut had_error = false;
            for token in scan(&source) {
                if token.kind == SyntaxKind::Error {
                    had_error = true;
                }
                let spelling = token
                    .range
                    .slice(&source)
                    .expect("scanner ranges are valid");
                println!(
                    "{:?} [{}..{}) {:?}",
                    token.kind,
                    token.range.start().0,
                    token.range.end().0,
                    spelling
                );
            }
            if had_error {
                eprintln!("rlox: scan contains lexical errors");
                return ExitCode::FAILURE;
            }
        }
    }
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
            Command::Tokens { .. } => panic!("expected the source command"),
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
        for command in ["source", "tokens"] {
            assert!(Cli::try_parse_from(["rlox", command]).is_err());
        }
    }

    #[test]
    fn rejects_extra_filenames() {
        for command in ["source", "tokens"] {
            assert!(Cli::try_parse_from(["rlox", command, "hello.lox", "other.lox"]).is_err());
        }
    }

    #[test]
    fn help_is_a_successful_request() {
        let help = Cli::try_parse_from(["rlox", "--help"])
            .err()
            .expect("help should stop parsing");

        assert_eq!(help.kind(), clap::error::ErrorKind::DisplayHelp);
        assert_eq!(help.exit_code(), 0);
    }

    #[test]
    fn accepts_tokens_command() {
        let cli = Cli::try_parse_from(["rlox", "tokens", "hello.lox"])
            .expect("tokens with a filename should parse");

        match cli.command {
            Command::Tokens { file } => assert_eq!(file, "hello.lox"),
            Command::Source { .. } => panic!("expected the tokens command"),
        }
    }
}
