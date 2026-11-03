use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let filename = match filename_from_args(&args) {
        Some(filename) => filename,
        None => {
            eprintln!("Usage: rlox <file>");
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
