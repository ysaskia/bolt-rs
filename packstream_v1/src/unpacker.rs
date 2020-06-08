use packstream_core::consts::*;
use packstream_core::types::*;
use packstream_core::unpacker::*;
use packstream_core::error::BoltError;
use crate::structs::Struct;
use crate::value::Value;
use crate::messages::Message;

impl UnpackValue<Value> for Unpacker {
    fn unpack(&mut self) -> Result<Value, BoltError> {
        self.input
            .peek()
            .and_then(|byte| match PackType::from(byte) {
                PackType::Null    => self.input.read_u8().map(|_| Value::Null),
                PackType::Boolean => self.unpack().map(Value::Boolean),
                PackType::Int     => self.unpack().map(Value::Int),
                PackType::Float   => self.unpack().map(Value::Float),
                PackType::String  => self.unpack().map(Value::String),
                PackType::List    => self.unpack().map(Value::List),
                PackType::Map     => self.unpack().map(Value::Map),
                PackType::Struct  => self.unpack().map(Value::Struct),
                _                 => Err(BoltError::Unmanaged)
            })
    }
}

impl UnpackValue<Struct> for Unpacker {
    fn unpack(&mut self) -> Result<Struct, BoltError> {
        self.peek_struct_signature()
            .and_then(|byte| match byte {
                STRUCT_NODE                 => self.unpack().map(Struct::Node),
                STRUCT_RELATIONSHIP         => self.unpack().map(Struct::Relationship),
                STRUCT_PATH                 => self.unpack().map(Struct::Path),
                STRUCT_UNBOUND_RELATIONSHIP => self.unpack().map(Struct::UnboundRelationship),
                _                           => Err(BoltError::Unmanaged)
            })
    }
}

impl UnpackValue<Message> for Unpacker {
    fn unpack(&mut self) -> Result<Message, BoltError> {
        self.peek_struct_signature()
            .and_then(|byte| match byte {
                MSG_INIT        => self.unpack().map(Message::Init),
                MSG_RUN         => self.unpack().map(Message::Run),
                MSG_DISCARD_ALL => self.unpack().map(Message::DiscardAll),
                MSG_PULL_ALL    => self.unpack().map(Message::PullAll),
                MSG_ACK_FAILURE => self.unpack().map(Message::AckFailure),
                MSG_RESET       => self.unpack().map(Message::Reset),
                MSG_RECORD      => self.unpack().map(Message::Record),
                MSG_SUCCESS     => self.unpack().map(Message::Success),
                MSG_FAILURE     => self.unpack().map(Message::Failure),
                MSG_IGNORED     => self.unpack().map(Message::Ignored),
                _               => Err(BoltError::Unmanaged)
            })
    }
}
