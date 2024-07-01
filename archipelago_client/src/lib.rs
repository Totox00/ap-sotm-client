use archipelago_protocol::{
    network_version, Bounce, ClientMessage, ClientStatus, Connect, Connected, DataPackage, DataStorageOperation, Get, GetDataPackage, LocationChecks, LocationInfo, LocationScouts, ReceivedItems,
    Retrieved, RoomInfo, Say, ServerMessage, Set, SetReply, StatusUpdate,
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, Stream, StreamExt,
};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message;

#[derive(Error, Debug)]
pub enum Error {
    #[error("illegal response")]
    IllegalResponse { received: ServerMessage, expected: &'static str },
    #[error("connection closed by server")]
    ConnectionClosed,
    #[error("data failed to serialize")]
    FailedSerialize(#[from] serde_json::Error),
    #[error("unexpected non-text result from websocket")]
    NonTextWebsocketResult(Message),
    #[error("network error")]
    NetworkError(#[from] tungstenite::Error),
}

/**
 * A convenience layer to manage your connection to and communication with Archipelago
 */
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Client {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    message_buffer: Vec<ServerMessage>,
}

impl Client {
    /**
     * Create an instance of the client and connect to the server on the given URL
     */
    pub async fn new(url: &str) -> Result<(Client, RoomInfo), Error> {
        let (mut ws, _) = connect_async(url).await?;
        let response = recv_messages(&mut ws).await.ok_or(Error::ConnectionClosed)??;
        let mut iter = response.into_iter();
        let room_info = match iter.next() {
            Some(ServerMessage::RoomInfo(room)) => room,
            Some(received) => {
                return Err(Error::IllegalResponse {
                    received,
                    expected: "Expected RoomInfo",
                })
            }
            None => return Err(Error::ConnectionClosed),
        };

        Ok((Client { ws, message_buffer: iter.collect() }, room_info))
    }

    pub async fn get_data_package(&mut self, games: Option<Box<[String]>>) -> Result<DataPackage, Error> {
        self.send(ClientMessage::GetDataPackage(GetDataPackage { games })).await?;
        let response = self.recv().await?;
        match response {
            Some(ServerMessage::DataPackage(pkg)) => Ok(pkg),
            Some(received) => Err(Error::IllegalResponse { received, expected: "DataPackage" }),
            None => Err(Error::ConnectionClosed),
        }
    }

    pub async fn send(&mut self, message: ClientMessage) -> Result<(), Error> {
        let request = serde_json::to_string(&[message])?;
        self.ws.send(Message::Text(request)).await?;

        Ok(())
    }

    /**
     * Read a message from the server
     *
     * Will buffer results locally, and return results from buffer or wait on network
     * if buffer is empty
     */
    pub async fn recv(&mut self) -> Result<Option<ServerMessage>, Error> {
        if let Some(message) = self.message_buffer.pop() {
            return Ok(Some(message));
        }
        let messages = recv_messages(&mut self.ws).await;
        if let Some(result) = messages {
            let mut messages = result?;
            messages.reverse();
            let first = messages.pop();
            self.message_buffer = messages;
            Ok(first)
        } else {
            Ok(None)
        }
    }

    /**
     * Send a connect request to the Archipelago server
     *
     * Will attempt to read a Connected packet in response, and will return an error if
     * another packet is found
     */
    pub async fn connect(&mut self, game: &str, name: &str, uuid: &str, pass: Option<&str>, items_handling: Option<i32>, tags: &[String]) -> Result<Connected, Error> {
        self.send(ClientMessage::Connect(Connect {
            game: game.into(),
            name: name.into(),
            uuid: uuid.into(),
            password: pass.map(|pass| pass.into()),
            version: network_version(),
            items_handling,
            tags: tags.into(),
        }))
        .await?;
        let response = self.recv().await?.ok_or(Error::ConnectionClosed)?;

        match response {
            ServerMessage::Connected(connected) => Ok(connected),
            received => Err(Error::IllegalResponse { received, expected: "Connected" }),
        }
    }

    /**
     * Basic chat command which sends text to the server to be distributed to other clients.
     */
    pub async fn say(&mut self, message: &str) -> Result<(), Error> {
        self.send(ClientMessage::Say(Say { text: message.into() })).await
    }

    /**
     * Sent to server to request a `ReceivedItems` packet to synchronize items.
     *
     * Will buffer any non-ReceivedItems packets returned
     */
    pub async fn sync(&mut self) -> Result<ReceivedItems, Error> {
        self.send(ClientMessage::Sync).await?;
        while let Some(response) = self.recv().await? {
            match response {
                ServerMessage::ReceivedItems(items) => return Ok(items),
                resp => self.message_buffer.push(resp),
            }
        }

        Err(Error::ConnectionClosed)
    }

    /**
     * Sent to server to inform it of locations that the client has checked.
     *
     * Used to inform the server of new checks that are made, as well as to sync state.
     */
    pub async fn location_checks(&mut self, locations: Vec<i64>) -> Result<(), Error> {
        self.send(ClientMessage::LocationChecks(LocationChecks { locations })).await
    }

    /**
     * Sent to the server to inform it of locations the client has seen, but not checked.
     *
     * Useful in cases in which the item may appear in the game world, such as 'ledge items' in A Link to the Past. Non-LocationInfo packets will be buffered
     */
    pub async fn location_scouts(&mut self, locations: Vec<i64>, create_as_hint: i32) -> Result<LocationInfo, Error> {
        self.send(ClientMessage::LocationScouts(LocationScouts { locations, create_as_hint })).await?;
        while let Some(response) = self.recv().await? {
            match response {
                ServerMessage::LocationInfo(items) => return Ok(items),
                resp => self.message_buffer.push(resp),
            }
        }

        Err(Error::ConnectionClosed)
    }

    /**
     * Sent to the server to update on the sender's status.
     *
     * Examples include readiness or goal completion. (Example: defeated Ganon in A Link to the Past)
     */
    pub async fn status_update(&mut self, status: ClientStatus) -> Result<(), Error> {
        self.send(ClientMessage::StatusUpdate(StatusUpdate { status })).await
    }

    /**
     * Send this message to the server, tell it which clients should receive the message and the server will forward the message to all those targets to which any one requirement applies.
     */
    pub async fn bounce(&mut self, games: Option<Vec<String>>, slots: Option<Vec<String>>, tags: Option<Vec<String>>, data: serde_json::Value) -> Result<(), Error> {
        self.send(ClientMessage::Bounce(Bounce { games, slots, tags, data })).await
    }

    /**
     * Used to request a single or multiple values from the server's data storage, see the Set package for how to write values to the data storage.
     *
     * A Get package will be answered with a Retrieved package. Non-Retrieved responses are
     * buffered
     */
    pub async fn get(&mut self, keys: Vec<String>) -> Result<Retrieved, Error> {
        self.send(ClientMessage::Get(Get { keys })).await?;
        while let Some(response) = self.recv().await? {
            match response {
                ServerMessage::Retrieved(items) => return Ok(items),
                resp => self.message_buffer.push(resp),
            }
        }

        Err(Error::ConnectionClosed)
    }

    /**
     * Used to write data to the server's data storage, that data can then be shared across worlds or just saved for later.
     *
     * Values for keys in the data storage can be retrieved with a Get package, or monitored with a `SetNotify` package. Non-SetReply responses are buffered
     */
    pub async fn set(&mut self, key: String, default: serde_json::Value, want_reply: bool, operations: Vec<DataStorageOperation>) -> Result<SetReply, Error> {
        self.send(ClientMessage::Set(Set { key, default, want_reply, operations })).await?;
        while let Some(response) = self.recv().await? {
            match response {
                ServerMessage::SetReply(items) => return Ok(items),
                resp => self.message_buffer.push(resp),
            }
        }

        Err(Error::ConnectionClosed)
    }

    /**
     * Split the client into two parts, one to handle sending and one to handle receiving.
     *
     * This removes access to a few convenience methods (like `get` or `set`) because it's
     * there's now extra coordination required to match a read and write, but it brings
     * the benefits of allowing simultaneous reading and writing.
     */
    pub fn split(self) -> (ClientSender, ClientReceiver) {
        let Self { ws, message_buffer } = self;
        let (send, recv) = ws.split();
        (ClientSender { ws: send }, ClientReceiver { ws: recv, message_buffer })
    }
}

/**
 * Once split, this struct handles the sending-side of your connection
 *
 * For helper method docs, see `Client`. Helper methods that require
 * both sending and receiving are intentionally unavailable; for those messages,
 * use `send`.
 */
pub struct ClientSender {
    ws: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl ClientSender {
    pub async fn send(&mut self, message: ClientMessage) -> Result<(), Error> {
        let request = serde_json::to_string(&[message])?;
        self.ws.send(Message::Text(request)).await?;

        Ok(())
    }

    pub async fn say(&mut self, message: &str) -> Result<(), Error> {
        self.send(ClientMessage::Say(Say { text: message.into() })).await
    }

    pub async fn location_checks(&mut self, locations: Vec<i64>) -> Result<(), Error> {
        self.send(ClientMessage::LocationChecks(LocationChecks { locations })).await
    }

    pub async fn status_update(&mut self, status: ClientStatus) -> Result<(), Error> {
        self.send(ClientMessage::StatusUpdate(StatusUpdate { status })).await
    }

    pub async fn bounce(&mut self, games: Option<Vec<String>>, slots: Option<Vec<String>>, tags: Option<Vec<String>>, data: serde_json::Value) -> Result<(), Error> {
        self.send(ClientMessage::Bounce(Bounce { games, slots, tags, data })).await
    }
}

/**
 * Once split, this struct handles the receiving-side of your connection
 *
 * For helper method docs, see `Client`. Helper methods that require
 * both sending and receiving are intentionally unavailable; for those messages,
 * use `recv`.
 */
pub struct ClientReceiver {
    ws: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    message_buffer: Vec<ServerMessage>,
}

impl ClientReceiver {
    pub async fn recv(&mut self) -> Result<Option<ServerMessage>, Error> {
        if let Some(message) = self.message_buffer.pop() {
            return Ok(Some(message));
        }
        let messages = recv_messages(&mut self.ws).await;
        if let Some(result) = messages {
            let mut messages = result?;
            messages.reverse();
            let first = messages.pop();
            self.message_buffer = messages;
            Ok(first)
        } else {
            Ok(None)
        }
    }
}

async fn recv_messages(mut ws: impl Stream<Item = Result<Message, tungstenite::error::Error>> + std::marker::Unpin) -> Option<Result<Vec<ServerMessage>, Error>> {
    match ws.next().await? {
        Ok(Message::Text(response)) => Some(serde_json::from_str::<Vec<ServerMessage>>(&response).map_err(Into::into)),
        Ok(Message::Close(_)) => Some(Err(Error::ConnectionClosed)),
        Ok(msg) => Some(Err(Error::NonTextWebsocketResult(msg))),
        Err(e) => Some(Err(e.into())),
    }
}
