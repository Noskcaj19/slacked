use commands;

pub fn handle_line(line: &str) {
    let result = commands::parse_command(line);
    println!("Got: {:?}", result);
}
