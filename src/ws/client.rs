use super::{sign, Channel, Data, Error, IntoMessage, Message, Response, Result};
use chrono::{Duration, Utc};
use futures_util::{
    ready,
    task::{Context, Poll},
    Future, SinkExt, Stream, StreamExt,
};
use std::collections::VecDeque;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio::time::{self, Interval};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub const TESTNET: &str = "stream-testnet.bybit.com";
pub const MAINNET_BYBIT: &str = "stream.bybit.com";
pub const MAINNET_BYTICK: &str = "stream.bytick.com";

pub struct Client {
    hostname: String,
    api_key: String,
    api_secret: String,
    stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    channels: Vec<Channel>,
    ping_timer: Interval,
    buf: VecDeque<Data>,
}

impl Client {
    /// Create a new websocket client.
    /// - `hostname` - The hostname to connect to.
    /// - `api_key` - The api key used for authentication.
    /// - `api_secret` - The api secret used for authentication.
    pub fn new(hostname: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            hostname: hostname.to_owned(),
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
            stream: None,
            channels: Vec::new(),
            ping_timer: time::interval(time::Duration::from_secs(15)),
            buf: VecDeque::new(),
        }
    }

    /// Returns `true` is this client is connected.
    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    /// Returns `true` is this client is authenticated.
    pub fn is_authenticated(&self) -> bool {
        !self.api_key.is_empty()
    }

    /// Connect the client.
    pub async fn connect(&mut self) -> Result<()> {
        let expires = (Utc::now() + Duration::seconds(2)).timestamp_millis();
        let signature = sign(expires, &self.api_secret);
        let url = format!(
            "wss://{}/realtime?api_key={}&expires={}&signature={}",
            self.hostname, self.api_key, expires, signature
        );

        let (stream, _) = connect_async(url).await?;
        self.stream = Some(stream);

        Ok(())
    }

    /// Subscribe to the given channels.
    /// - `channels` - The channels to subscribe to.
    pub async fn subscribe(&mut self, channels: &[Channel]) -> Result<()> {
        for channel in channels.iter() {
            if channel.requires_authentication() && !self.is_authenticated() {
                return Err(Error::NotAuthenticated);
            }
        }

        self.subscribe_or_unsubscribe(&channels, true).await?;
        self.channels.extend_from_slice(&channels);

        Ok(())
    }

    /// Unsubscribe from the given channels.
    /// - `channels` - The channels to unsubscribe from.
    pub async fn unsubscribe(&mut self, channels: &[Channel]) -> Result<()> {
        for channel in channels.iter() {
            if !self.channels.contains(channel) {
                return Err(Error::NotSubscribed(channel.clone()));
            }
        }

        self.subscribe_or_unsubscribe(&channels, false).await?;
        self.channels.retain(|c| !channels.contains(c));

        Ok(())
    }

    /// Unsubscribe from all channels.
    pub async fn unsubscribe_all(&mut self) -> Result<()> {
        self.unsubscribe(&self.channels.clone()).await?;

        Ok(())
    }

    /// Send a message.
    /// - `message` - The message to send.
    pub async fn send<M>(&mut self, message: M) -> Result<()>
    where
        M: IntoMessage,
    {
        if let Some(stream) = self.stream.as_mut() {
            let message = message.into_message()?;
            stream.send(message).await?;
            return Ok(());
        }

        Err(Error::NotConnected)
    }

    /// Send a ping.
    pub async fn ping(&mut self) -> Result<()> {
        let message = Message::text("{\"op\":\"ping\"}");
        self.send(message).await
    }

    async fn subscribe_or_unsubscribe(
        &mut self,
        channels: &[Channel],
        subscribe: bool,
    ) -> Result<()> {
        let op = if subscribe {
            "subscribe"
        } else {
            "unsubscribe"
        };

        'channels: for channel in channels {
            let topic: String = match channel {
                Channel::OrderBook25(symbol) => format!("orderBookL2_25.{}", symbol),
                Channel::OrderBook200(symbol) => format!("orderBookL2_200.{}", symbol),
                Channel::Trade => "trade".to_owned(),
                Channel::Insurance => "insurance".to_owned(),
                Channel::InstrumentInfo(symbol, interval) => {
                    format!("instrument_info.{}.{}", interval, symbol)
                }
                Channel::KlineV2(symbol, interval) => format!("klineV2.{}.{}", interval, symbol),
                Channel::Position => "position".to_owned(),
                Channel::Execution => "execution".to_owned(),
                Channel::Order => "order".to_owned(),
                Channel::StopOrder => "stop_order".to_owned(),
            };

            let message = Message::Text(
                serde_json::json!({"op": op, "args": vec![topic.clone()]}).to_string(),
            );

            self.send(message).await?;

            match self.await_subscription_response(op, &topic).await? {
                Some(success) => {
                    if success {
                        continue 'channels;
                    } else {
                        return Err(Error::SubscriptionFailed(channel.clone()));
                    }
                }
                None => return Err(Error::MissingSubscriptionConfirmation(channel.clone())),
            }
        }

        Ok(())
    }

    async fn next_response(&mut self) -> Result<Response> {
        loop {
            if let Some(stream) = self.stream.as_mut() {
                tokio::select! {
                    _ = self.ping_timer.tick() => {
                        self.ping().await?;
                    },
                    Some(msg) = stream.next() => {
                        let msg = msg?;
                        if let Message::Text(text) = msg {
                            let response: Response = serde_json::from_str(&text)?;
                            return Ok(response)
                        }
                    }
                }
            }
        }
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            Response::Request(_res) => {}
            Response::OrderbookSnapshot(res) => self
                .buf
                .extend(res.data.into_iter().map(Data::OrderbookSnapshot)),
            Response::OrderbookDelta(res) => self.buf.push_back(Data::OrderbookDelta(res.data)),
            Response::Trade(res) => self.buf.extend(res.data.into_iter().map(Data::Trade)),
            Response::Insurance(res) => self.buf.extend(res.data.into_iter().map(Data::Insurance)),
            Response::InstrumentInfoSnapshot(res) => {
                self.buf.push_back(Data::InstrumentInfoSnapshot(res.data))
            }
            Response::InstrumentInfoDelta(res) => {
                self.buf.push_back(Data::InstrumentInfoDelta(res.data))
            }
            Response::KlineV2(res) => self.buf.extend(res.data.into_iter().map(Data::KlineV2)),
            Response::Position(res) => self.buf.extend(res.data.into_iter().map(Data::Position)),
            Response::Execution(res) => self.buf.extend(res.data.into_iter().map(Data::Execution)),
            Response::Order(res) => self.buf.extend(res.data.into_iter().map(Data::Order)),
            Response::StopOrder(res) => self.buf.extend(res.data.into_iter().map(Data::StopOrder)),
        }
    }

    async fn await_subscription_response(&mut self, op: &str, topic: &str) -> Result<Option<bool>> {
        // Confirmation should arrive within the next 100 updates
        for _ in 0..100_i32 {
            let response = self.next_response().await?;
            match response {
                Response::Request(ref res) if res.request.op == op => {
                    match res.request.args.as_ref() {
                        Some(args) if args[0] == topic => return Ok(Some(res.success)),
                        _ => self.handle_response(response),
                    }
                }
                _ => self.handle_response(response),
            }
        }

        Ok(None)
    }
}

impl Stream for Client {
    type Item = Result<Data>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            if let Some(data) = self.buf.pop_front() {
                return Poll::Ready(Some(Ok(data)));
            }
            let response = {
                let mut next_response = self.next_response();
                let pinned: Pin<_> = unsafe { Pin::new_unchecked(&mut next_response) };
                match ready!(pinned.poll(cx)) {
                    Ok(response) => response,
                    Err(e) => return Poll::Ready(Some(Err(e))),
                }
            };
            self.handle_response(response);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connect() {
        let mut client = Client::new(TESTNET, "", "");
        assert!(client.connect().await.is_ok());
    }

    #[tokio::test]
    async fn ping() {
        let mut client = Client::new(TESTNET, "", "");
        assert!(client.connect().await.is_ok());
        assert!(client.ping().await.is_ok());
    }

    #[tokio::test]
    async fn subscribe() {
        let mut client = Client::new(TESTNET, "", "");
        assert!(client.connect().await.is_ok());
        assert!(client.subscribe(&[Channel::Trade]).await.is_ok());
        assert!(client.subscribe(&[Channel::Order]).await.is_err());
    }

    #[tokio::test]
    async fn stream() {
        let mut client = Client::new(TESTNET, "", "");
        assert!(client.connect().await.is_ok());
        assert!(client.subscribe(&[Channel::Trade]).await.is_ok());
        assert!(client.next().await.is_some());
    }
}
