use crate::proto::stomp::STOMPFrame;
use bytes::Bytes;

pub enum Message {
    StreamMessage(Bytes),
    ChannelMessage(Bytes),
}
