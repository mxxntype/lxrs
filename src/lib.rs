use std::{env, fs, process};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let token_stream = tokenize(filename);
            println!("{token_stream}");
        }
        _ => eprintln!("Unknown command: {command}"),
    }
}

#[must_use]
pub fn tokenize(filename: &str) -> String {
    let file_contents = fs::read_to_string(filename)
        .inspect_err(|error| eprintln!("Failed to read file {filename}: {error}"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    if file_contents.is_empty() {
        // TODO: Placeholder, remove this line when implementing the scanner.
        "EOF  null".into()
    } else {
        unimplemented!();
    }
}
