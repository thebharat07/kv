pub enum Command<'a> {
    Get { key: &'a str },
    Set { key: &'a str, value: &'a str },
    Invalid(&'static str),
}

pub fn parse_command(line: &str) -> Command<'_> {
    let parts: Vec<&str> = line.trim().split_ascii_whitespace().collect();

    match parts.as_slice() {
        ["GET", key] => Command::Get { key },
        ["GET", ..] => Command::Invalid("Error: No key found in the query!"),
        ["SET", key, value] => Command::Set { key, value },
        ["SET", ..] => Command::Invalid("Error: Not enough parameters!"),
        _ => Command::Invalid("Error: Unknown command"),
    }
}