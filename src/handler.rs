use interface_errors::ErrorKind;

use commands;
use commands::Action;

pub struct Handler {
    last_error: Option<ErrorKind>,
}

impl Handler {
    pub fn new() -> Handler {
        Handler { last_error: None }
    }

    fn handle_err(&mut self) {
        self.last_error = Some(ErrorKind::UnknownCommand);
        println!("?")
    }

    pub fn handle_line(&mut self, line: &str) {
        let command = commands::parse_command(line);
        println!("Got: {:?}", command);

        let command = match command {
            Some(command) => command,
            None => return,
        };

        match command.action {
            Action::Print => unimplemented!(),
            Action::Append => unimplemented!(),
            Action::Delete => unimplemented!(),
            Action::Unknown => self.handle_err(),
        }
    }
}
