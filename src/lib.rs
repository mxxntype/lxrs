use std::{env, fs, process, str::FromStr};

#[derive(strum_macros::Display, strum_macros::EnumString)]
#[allow(clippy::upper_case_acronyms)]
enum TokenType {
    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "")]
    EOF,
}

impl TokenType {
    pub const fn as_verbose_token(&self) -> &'static str {
        match self {
            Self::LeftParen => "LEFT_PAREN",
            Self::RightParen => "RIGHT_PAREN",
            Self::EOF => "EOF",
        }
    }
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
        for word in line.split_whitespace() {
            for c in word.chars() {
                match TokenType::from_str(c.to_string().as_str()) {
                    Ok(token_type) => {
                        token_stream.push(format!("{} {} null", token_type.as_verbose_token(), c));
                    }
                    Err(_) => todo!("Implement syntax errors"),
                }
            }
        }
    }

    token_stream.push(format!("{}  null", TokenType::EOF.as_verbose_token()));

    token_stream.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::TokenType;

    #[test]
    fn token_serialization() {
        assert_eq!(TokenType::LeftParen.as_verbose_token(), "LEFT_PAREN");
        assert_eq!(TokenType::RightParen.as_verbose_token(), "RIGHT_PAREN");
    }
}
