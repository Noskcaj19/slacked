#[derive(Debug)]
pub enum Address {
    NumberOffset(isize),
    TimeOffset(isize),
}

#[derive(Debug)]
pub enum Range {
    DoubleEnded { start: Address, end: Address },
    From(Address),
    Single(Address),
}

#[derive(Debug)]
pub struct ParsedCommand {
    pub range: Option<Range>,
    pub action: Action,
    pub arguments: String,
}

#[derive(Debug)]
pub enum Action {
    // p - print
    Print,
    // a - append
    Append,
    // x - delete
    Delete,
    // h - help
    Help,
    // - `?`
    Unknown,
}

impl Default for Action {
    fn default() -> Action {
        Action::Print
    }
}

impl Action {
    pub fn from_str(c: char) -> Action {
        match c {
            'p' => Action::Print,
            'a' => Action::Append,
            'x' => Action::Delete,
            'h' => Action::Help,
            _ => Action::Unknown,
        }
    }
}

fn map_to<F: FnOnce(isize) -> Address>(
    chars: impl Iterator<Item = char>,
    op: F,
) -> Option<Address> {
    chars
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .map(op)
        .ok()
}

fn parse_address(address: &str) -> Option<Address> {
    let mut chars = address.chars();
    match chars.next() {
        Some('@') => map_to(chars, |i| Address::TimeOffset(i)),
        Some(c) if c.is_numeric() => map_to(address.chars(), |i| Address::NumberOffset(i)),
        _ => None,
    }
}

fn parse_range(range: &str) -> Option<Range> {
    if range.ends_with(',') {
        Some(Range::From(parse_address(&range[..range.len() - 1])?))
    } else if range.contains(',') {
        let seperator = range.find(',')?;
        Some(Range::DoubleEnded {
            start: parse_address(&range[..seperator])?,
            end: parse_address(&range[seperator + 1..])?,
        })
    } else {
        Some(Range::Single(parse_address(range)?))
    }
}

pub fn parse_command(command: &str) -> Option<ParsedCommand> {
    let command_start = command
        .chars()
        .position(|c| c.is_alphabetic())
        .unwrap_or(command.len());
    let range: String = command.chars().take(command_start).collect();
    let range = parse_range(&range);

    Some(ParsedCommand {
        range,
        action: command
            .chars()
            .skip(command_start)
            .next()
            .map(|command| Action::from_str(command))
            .unwrap_or_default(),
        arguments: command
            .chars()
            .skip(command_start + 1)
            .collect::<String>()
            .trim()
            .into(),
    })
}
