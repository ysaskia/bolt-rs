use packstream_core::packer::{Packer,PackValue};
use packstream_core::error::BoltError;
use crate::value::*;
use crate::structs::{Struct};
use crate::messages::Message;

impl PackValue<Value> for Packer {
  fn pack(&mut self, val: Value) -> Result<(), BoltError> {
    match val {
      // TODO found a way to impl <T> for Option<T>
      // Value::Null       => self.pack(None),
      Value::Boolean(b) => self.pack(b),
      Value::Int(n)     => self.pack(n),
      Value::Float(n)   => self.pack(n),
      Value::String(cs) => self.pack(cs),
      Value::List(xs)   => self.pack(xs),
      Value::Map(ps)    => self.pack(ps),
      Value::Struct(x)  => self.pack(x),
      _                 => Err(BoltError::Unmanaged)
    }
  }
}

impl PackValue<Struct> for Packer {
  fn pack(&mut self, val: Struct) -> Result<(), BoltError> {
    match val {
      Struct::Node(x)                => self.pack(x),
      Struct::Relationship(x)        => self.pack(x),
      Struct::Path(x)                => self.pack(x),
      Struct::UnboundRelationship(x) => self.pack(x),
    }
  }
}

impl PackValue<Message> for Packer {
  fn pack(&mut self, val: Message) -> Result<(), BoltError> {
    match val {
      Message::Init(x)       => self.pack(x),
      Message::Run(x)        => self.pack(x),
      Message::DiscardAll(x) => self.pack(x),
      Message::PullAll(x)    => self.pack(x),
      Message::AckFailure(x) => self.pack(x),
      Message::Reset(x)      => self.pack(x),
      Message::Record(x)     => self.pack(x),
      Message::Success(x)    => self.pack(x),
      Message::Failure(x)    => self.pack(x),
      Message::Ignored(x)    => self.pack(x),
    }
  }
}
