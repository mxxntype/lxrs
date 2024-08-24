use std::{env, fs, process, str::FromStr};

use strum::EnumMessage;

#[derive(strum_macros::Display, strum_macros::EnumString, strum_macros::EnumMessage)]
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
        .map(|contents| contents.trim().to_string())
        .unwrap_or_default();

    let mut token_stream = Vec::<String>::default();

    for line in file_contents
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("//"))
    {
        dbg!(&TokenType::RightBrace.to_string());
        for word in line.split_whitespace() {
            for c in word.chars() {
                match TokenType::from_str(c.to_string().as_str()) {
                    Ok(token) => {
                        let token_representation =
                            format!("{} {} null", token.get_message().unwrap_or_default(), c);
                        token_stream.push(token_representation);
                    }
                    Err(err) => todo!("Implement syntax errors: {err}"),
                }
            }
        }
    }

    token_stream.push(format!(
        "{}  null",
        TokenType::EOF.get_message().unwrap_or_default()
    ));

    token_stream.join("\n")
}
