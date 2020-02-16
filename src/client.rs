use crate::message::Message;
use crate::proto::stomp::{STOMPCodec, STOMPFrame};
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio::stream::{Stream, StreamExt};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::Framed;

type Rx = mpsc::UnboundedReceiver<STOMPFrame>;

pub struct Client {
    pub rx: Option<Rx>,
    pub stream: Framed<TcpStream, STOMPCodec>,
}

impl Stream for Client {
    type Item = Result<Message, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rx.is_some() {
            //we initialize rx only when a socket subscribes to a queue
            if let Poll::Ready(Some(v)) = Pin::new(self.rx.as_mut().unwrap()).poll_next(cx) {
                return Poll::Ready(Some(Ok(Message::ChannelMessage(v))));
            }
        }

        let result: Option<_> = futures::ready!(Pin::new(&mut self.stream).poll_next(cx));
        Poll::Ready(match result {
            Some(Ok(message)) => Some(Ok(Message::StreamMessage(message))),
            Some(Err(e)) => Some(Err(e)),
            None => None,
        })
    }
}
