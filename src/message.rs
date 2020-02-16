use crate::proto::stomp::STOMPFrame;

pub enum Message {
    StreamMessage(STOMPFrame), 
    ChannelMessage(STOMPFrame)
}