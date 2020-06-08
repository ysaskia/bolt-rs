use std::collections::HashMap;
use packstream_proc::{bolt_packstream};
use packstream_core::packer::{PackValue,Packer};
use packstream_core::unpacker::{UnpackValue,Unpacker};
use packstream_core::error::BoltError;
use crate::value::Value;

/// The `Init` message is a client message used once to initialize the session.
/// This message is always the first message the client sends after negotiating
/// protocol version. Sending any message other than `Init` as the first message
/// to the server will result in a `Failure`. As described in Failure handling
/// the client must acknowledge failures using [AckFailure], after which `Init`
/// may be reattempted.
///
/// All parameters in the `Init` message are required.
/// [AckFailure]: crate::messages::AckFailure
#[bolt_packstream(0x01)]
pub struct Init {
  pub client_name: String,
  pub auth_token: HashMap<String, Value>
}

/// The `Run` message is a client message used to pass a statement for execution
/// on the server.
#[bolt_packstream(0x10)]
pub struct Run {
  pub statement: String,
  pub parameters: HashMap<String, Value>
}

/// The `DiscardAll` message is a client message used to discard all remaining
/// items from the active result stream.
#[bolt_packstream(0x2F)]
pub struct DiscardAll;

/// The `PullAll` message is a client message used to retrieve all remaining
/// items from the active result stream.
#[bolt_packstream(0x3F)]
pub struct PullAll;

/// The `AckFailure` message is a client message used to acknowledge a failure
/// the server has sent.
#[bolt_packstream(0x0E)]
pub struct AckFailure;

/// The `Reset` message is a client message used to return the current session
/// to a "clean" state. It will cause the session to IGNORE any message it is
/// currently processing, as well as any message before RESET that had not yet
/// begun processing. This allows `Reset` to abort long-running operations. It
/// also means clients must be careful about pipelining `Reset`. Only send this
/// if you are not currently waiting for a result from a prior message, or if
/// you want to explicitly abort any prior message.
#[bolt_packstream(0x0F)]
pub struct Reset;

/// The `Record` message is a server detail message used to deliver data from
/// the server to the client. Each record message contains a single List, which
/// in turn contains the fields of the record in order.
#[bolt_packstream(0x71)]
pub struct Record {
  pub fields: Vec<Value>
}

/// The `Success` message is a server summary message used to signal that a
/// corresponding client message has been received and actioned as intended.
/// The message contains a map of metadata, the contents of which depend on
/// the original request
#[bolt_packstream(0x70)]
pub struct Success {
  pub metadata: HashMap<String, Value>
}

/// The `Failure` message is a server summary message used to signal that a
/// corresponding client message has encountered an error while being processed.
#[bolt_packstream(0x7F)]
pub struct Failure {
  pub metadata: HashMap<String, Value>
}

/// The `Ignored` message is a server summary message used to signal that a
/// corresponding client message has been ignored and not actioned.
#[bolt_packstream(0x7E)]
pub struct Ignored;


pub enum Message {
  Init(Init),
  Run(Run),
  DiscardAll(DiscardAll),
  PullAll(PullAll),
  AckFailure(AckFailure),
  Reset(Reset),
  Record(Record),
  Success(Success),
  Failure(Failure),
  Ignored(Ignored)
}
