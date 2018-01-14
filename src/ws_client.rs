use std::sync::mpsc;

use ws;

#[allow(dead_code)]
#[derive(Debug)]
pub enum RTMApi {
    Typing { id: u64, channel: String },
    Ping { id: u64 },
}

impl RTMApi {
    #[allow(dead_code)]
    pub fn encode(&self) -> String {
        use self::RTMApi::*;
        match *self {
            Typing { id, ref channel } => format!(
                r#"{{"id": {}, "type": "typing", "channel": "{}"}}"#,
                id, channel
            ),
            Ping { id } => format!(r#"{{"id": {}, "type": "ping"}}"#, id),
        }
    }
}

#[derive(Debug)]
pub enum WSEvent {
    Connected,
    Disconnected,
}

pub struct Client {
    /// WS Sender
    pub out: ws::Sender,
    pub status_sender: mpsc::Sender<WSEvent>,
}

impl Client {
    #[allow(dead_code)]
    pub fn send(&self, api: RTMApi) -> Result<(), ws::Error> {
        self.out.send(api.encode())
    }
}

impl ws::Handler for Client {
    fn on_open(&mut self, _: ws::Handshake) -> Result<(), ws::Error> {
        self.status_sender.send(WSEvent::Connected).map_err(|err| {
            ws::Error::new(
                ws::ErrorKind::Internal,
                format!("Unable to communicate between threads: {:?}.", err),
            )
        })
    }

    fn on_close(&mut self, _code: ws::CloseCode, _reason: &str) {
        // TODO: Error handling for this maybe?
        self.status_sender.send(WSEvent::Disconnected);
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<(), ws::Error> {
        println!("< {}", msg);
        Ok(())
    }
}
