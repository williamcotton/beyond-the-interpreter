use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = match source_filename_from_args(&args) {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: rlox source <file>");
            return ExitCode::FAILURE;
        }
    };
    let source = match std::fs::read_to_string(filename) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("rlox: could not read '{filename}': {error}");
            return ExitCode::FAILURE;
        }
    };
    print!("{source}");
    ExitCode::SUCCESS
}

fn filename_from_args(args: &[String]) -> Option<&str> {
    if args.len() != 1 {
        return None;
    }

    Some(&args[0])
}

fn source_filename_from_args(args: &[String]) -> Option<&str> {
    let (command, command_args) = args.split_first()?;

    match command.as_str() {
        "source" => filename_from_args(command_args),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{filename_from_args, source_filename_from_args};

    #[test]
    fn accepts_one_filename() {
        let args = vec![String::from("hello.lox")];

        assert_eq!(filename_from_args(&args), Some("hello.lox"));
    }

    #[test]
    fn rejects_missing_filename() {
        assert_eq!(filename_from_args(&[]), None);
    }

    #[test]
    fn rejects_extra_filenames() {
        let args = vec![String::from("hello.lox"), String::from("other.lox")];

        assert_eq!(filename_from_args(&args), None);
    }

    #[test]
    fn accepts_source_command() {
        let args = vec![String::from("source"), String::from("hello.lox")];

        assert_eq!(source_filename_from_args(&args), Some("hello.lox"));
    }

    #[test]
    fn requires_command() {
        assert_eq!(source_filename_from_args(&[]), None);
    }

    #[test]
    fn rejects_other_command_names() {
        let args = vec![String::from("unknown"), String::from("hello.lox")];
        assert_eq!(source_filename_from_args(&args), None);

        let args = vec![String::from("hello.lox")];
        assert_eq!(source_filename_from_args(&args), None);
    }

    #[test]
    fn source_requires_one_filename() {
        let args = vec![String::from("source")];
        assert_eq!(source_filename_from_args(&args), None);

        let args = vec![
            String::from("source"),
            String::from("hello.lox"),
            String::from("other.lox"),
        ];
        assert_eq!(source_filename_from_args(&args), None);
    }
}
