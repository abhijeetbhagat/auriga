pub enum Frame {
    Send, //sends a message to the destination
    Subscribe, //register to a given destination
    Unsubscribe, //unregister from a given destination
    Begin, //start a transaction
    Commit, //commit a transaction
    Abort, //roll back a transaction
    Ack, //ack consumption of a message from a subscription
    Nack, //tell the server that the message hasn't be consumed yet
    Disconnect, //disconnect from the server
    Message, //convey messages from subscriptions to clients
    Receipt, //ack from server that a client frame was processed
    Error, //let clients know of any errors
    Invalid //invalid frame
}

pub struct STOMPParser;

impl STOMPParser {
    pub fn new() -> Self {
        STOMPParser
    }

    pub fn parse(&self, command: &str) -> Frame {
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


