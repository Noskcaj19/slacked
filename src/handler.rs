use slack_api;

use commands;
use api_utils;
use commands::{Action, ParsedCommand};
use interface_errors::ErrorKind;

pub struct Handler {
    token: String,
    last_command: Option<ParsedCommand>,
    error: Option<ErrorKind>,
    slack_client: slack_api::requests::Client,
}

impl Handler {
    pub fn new(token: &str) -> Handler {
        Handler {
            token: token.into(),
            error: None,
            last_command: None,
            slack_client: slack_api::requests::default_client().unwrap(),
        }
    }

    fn show_err(&self) {
        if let Some(err) = self.error {
            println!("{}", err)
        }
    }

    fn search_users(&self, command: &ParsedCommand) {
        let members = api_utils::get_users(
            &self.slack_client,
            &self.token,
            &slack_api::users::ListRequest::default(),
        );
        for member in members
            .iter()
            .filter(|member| member.name.starts_with(&command.arguments[1..]))
        {
            println!("{} | {}", member.id, member.name)
        }
    }

    fn get_info(&mut self, command: &ParsedCommand) {
        match command.arguments.chars().next() {
            Some('@') => self.search_users(command),
            _ => self.handle_err(ErrorKind::InvalidArguments),
        }
    }

    fn handle_err(&mut self, err: ErrorKind) {
        self.error = Some(err);
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
            Action::Help => self.show_err(),
            Action::Unknown => self.handle_err(ErrorKind::UnknownCommand),
            Action::Get => self.get_info(&command),
        }
        self.last_command = Some(command);
    }
}
