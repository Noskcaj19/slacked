#[derive(Fail, Debug)]
#[fail(display = "Unable to locate user home directory")]
pub struct HomeDirError;

#[derive(Fail, Debug)]
#[fail(display = "Error connecting to RTM api")]
pub struct RTMConnectError;
