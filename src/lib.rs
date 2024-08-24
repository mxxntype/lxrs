use std::{env, fs, process, str::FromStr};
use strum::EnumMessage;
use strum_macros::{Display, EnumMessage, EnumString};

#[derive(Display, EnumString, EnumMessage)]
#[allow(clippy::upper_case_acronyms)]
enum TokenType {
    #[strum(serialize = "(", message = "LEFT_PAREN")]
    LeftParen,
    #[strum(serialize = ")", message = "RIGHT_PAREN")]
    RightParen,
    #[strum(serialize = "{", message = "LEFT_BRACE")]
    LeftBrace,
    #[strum(serialize = "}", message = "RIGHT_BRACE")]
    RightBrace,
    #[strum(serialize = "*", message = "STAR")]
    Star,
    #[strum(serialize = ".", message = "DOT")]
    Dot,
    #[strum(serialize = ",", message = "COMMA")]
    Comma,
    #[strum(serialize = "+", message = "PLUS")]
    Plus,
    #[strum(serialize = "-", message = "MINUS")]
    Minus,
    #[strum(serialize = ";", message = "SEMICOLON")]
    Semicolon,
    #[strum(serialize = "", message = "EOF")]
    EOF,
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => match tokenize(filename) {
            Ok(token_stream) => println!("{token_stream}"),
            Err((exit_code, _)) => process::exit(exit_code),
        },
        _ => eprintln!("Unknown command: {command}"),
    }
}

/// Try to tokenize the file located at `path`.
///
/// # Errors
///
/// This function will return an error if the file contains syntax errors.
pub fn tokenize(path: &str) -> Result<String, (i32, String)> {
    let file_contents = fs::read_to_string(path)
        .inspect_err(|error| eprintln!("Couldn't read file {path}: {error}"))
        .map(|contents| contents.trim().to_string())
        .unwrap_or_default();

    let mut token_stream = Vec::<String>::default();
    let mut exit_code = 0;

    for (linenr, line) in file_contents
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .filter(|(_, line)| !line.starts_with("//"))
    {
        for word in line.split_whitespace() {
            for c in word.chars() {
                match TokenType::from_str(c.to_string().as_str()) {
                    Ok(token) => {
                        let token_representation =
                            format!("{} {} null", token.get_message().unwrap_or_default(), c);
                        token_stream.push(token_representation);
                    }
                    Err(_) => {
                        eprintln!("[Line {linenr}] Error: Unexpected character: {c}");
                        exit_code = 65;
                    }
                }
            }
        }
    }

    token_stream.push(format!(
        "{}  null",
        TokenType::EOF.get_message().unwrap_or_default()
    ));

    let token_stream = token_stream.join("\n");

    if exit_code != 0 {
        Err((exit_code, token_stream))
    } else {
        Ok(token_stream)
    }
}
