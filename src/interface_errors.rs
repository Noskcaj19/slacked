#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Unknown command")] UnknownCommand,
    #[fail(display = "Invalid arguments")] InvalidArguments,
}
