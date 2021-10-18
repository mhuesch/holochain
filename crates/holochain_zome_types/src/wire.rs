use holochain_serialized_bytes::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, SerializedBytes)]
#[serde(tag = "type")]
/// The messages actually sent over the wire by the holochain websocket handler.
/// If you want to implement your own server or client you
/// will need this type or be able to serialize / deserialize it.
pub enum WireMessage {
    /// A message without a response.
    Signal {
        #[serde(with = "serde_bytes")]
        /// Actual bytes of the message serialized as [message pack](https://msgpack.org/).
        data: Vec<u8>,
    },
    /// A request that requires a response.
    Request {
        /// The id of this request.
        /// Note ids are recycled once they are used.
        id: u64,
        #[serde(with = "serde_bytes")]
        /// Actual bytes of the message serialized as [message pack](https://msgpack.org/).
        data: Vec<u8>,
    },
    /// The response to a request.
    Response {
        /// The id of the request that this response is for.
        id: u64,
        #[serde(with = "serde_bytes")]
        /// Actual bytes of the message serialized as [message pack](https://msgpack.org/).
        data: Option<Vec<u8>>,
    },
}
