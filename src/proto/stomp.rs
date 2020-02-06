pub enum Frame {
    Send,
    Subscribe,
    Unsubscribe,
    Begin,
    Commit,
    Abort,
    Ack,
    Nack,
    Disconnect,
    Message,
    Receipt,
    Error,
    Invalid
}

pub struct STOMPParser;

impl STOMPParser {
    pub fn new() -> Self {
        STOMPParser
    }

    pub fn parse(self, command: &str) -> Frame {
        let lines: Vec<&str> = command.split('\n').collect();
        let command_str = lines[0];
        match command_str { 
            "SEND" => Frame::Send,
            "SUBSCRIBE" => Frame::Subscribe,
            "UNSUBSCRIBE" => Frame::Unsubscribe,
            "BEGIN" => Frame::Begin,
            "COMMIT" => Frame::Commit,
            "ABORT" => Frame::Abort,
            "ACK" =>  Frame::Ack,
            "NACK" =>  Frame::Nack,
            "DISCONNECT" => Frame::Disconnect,
            _ => Frame::Invalid
        }
    }
}


