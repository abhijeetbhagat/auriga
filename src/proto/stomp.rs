use std::collections::HashMap;
use std::io;
use tokio_util::codec::{ Decoder, Encoder };
use bytes::{BufMut, Bytes, BytesMut};
use std::fmt;

#[derive(Debug, Clone)]
pub struct STOMPFrame {
    pub r#type: Frame,
    pub headers: HashMap<String, String>
}

impl STOMPFrame {
    pub fn new() -> Self {
        STOMPFrame {
            r#type: Frame::Invalid,
            headers: HashMap::new()
        }
    }
}

impl fmt::Display for STOMPFrame { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", self.r#type);
        for (k, v) in &self.headers {
            write!(f, "{}, {}", k, v);
        }
        Ok(())
    }
}

pub struct STOMPCodec;

impl STOMPCodec {
    pub fn new() -> STOMPCodec {
        STOMPCodec { }
    }
}


impl Decoder for STOMPCodec{
    type Item = STOMPFrame;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<STOMPFrame>> { 
        if !buf.is_empty() {
            let len = buf.len();
            let buf = buf.split_to(len);
            let parser = STOMPParser;
            let frame = parser.parse(std::str::from_utf8(&buf).unwrap());
            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for STOMPCodec{
    type Item = STOMPFrame;
    type Error = io::Error;

    fn encode(&mut self, item: STOMPFrame, dst: &mut BytesMut) -> Result<(), io::Error> { 
        dst.reserve(10);
        dst.extend(b"abhi\r\n");
        Ok(())
    }
}

#[derive(Debug, Clone)]
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

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {     
            Frame::Send => write!(f, "SEND"),
            Frame::Subscribe => write!(f, "SUBSCRIBE"),
            Frame::Unsubscribe => write!(f, "UNSUBSCRIBE"),
            Frame::Begin => write!(f, "BEGIN"),
            Frame::Commit => write!(f, "COMMIT"),
            Frame::Abort => write!(f, "ABORT"),
            Frame::Ack => write!(f, "ACK"),
            Frame::Nack => write!(f, "NACK"),
            Frame::Disconnect => write!(f, "DISCONNECT"),
            Frame::Message => write!(f, "MESSAGE"),
            Frame::Receipt => write!(f, "RECEIPT"),
            Frame::Error => write!(f, "ERROR"),
            Frame::Invalid => write!(f, "INVALID")
        }
    }
}

pub struct STOMPParser;

impl STOMPParser {
    pub fn new() -> Self {
        STOMPParser
    }

    pub fn parse(&self, command: &str) -> STOMPFrame {
        let lines: Vec<&str> = command
            .split('\n')
            .filter(|l| l != &"\0" && l != &"")
            .collect();
        let command_str = lines[0];
        let mut hm = HashMap::new();

        for line in &lines[1..] { 
            println!("splitting line - {}", line);
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