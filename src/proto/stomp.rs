use std::collections::HashMap;

pub struct STOMPFrame {
    pub r#type: Frame,
    pub headers: HashMap<String, String>
}

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

    pub fn parse(&self, command: &str) -> STOMPFrame {
        let lines: Vec<&str> = command
            .split('\n')
            .filter(|l| l != &"\n" && l != &"\0")
            .collect();
        let command_str = lines[0];
        let mut hm = HashMap::new();

        for line in &lines[1..] { 
            let hdr_line: Vec<&str> = line.split(':').collect();
            hm.insert(String::from(hdr_line[0]), String::from(hdr_line[1]));
        }

        let r#type = match command_str { 
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
        };

        STOMPFrame { 
            r#type: r#type,
            headers: hm
        }
    }
}


