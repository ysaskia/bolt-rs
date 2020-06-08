use std::collections::HashMap;
use packstream_proc::{bolt_packstream};
use packstream_core::packer::{PackValue,Packer};
use packstream_core::unpacker::{UnpackValue,Unpacker};
use packstream_core::error::BoltError;
use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Struct {
  Node(Node),
  Relationship(Relationship),
  Path(Path),
  UnboundRelationship(UnboundRelationship)
}

#[bolt_packstream(0x4E)]
#[derive(Debug, PartialEq)]
pub struct Node {
  pub id: i64,
  pub labels: Vec<String>,
  pub properties: HashMap<String, Value>
}

#[bolt_packstream(0x52)]
#[derive(Debug, PartialEq)]
pub struct Relationship {
  pub id: i64,
  pub start_node_id: i64,
  pub end_node_id: i64,
  pub type_name: String,
  pub properties: HashMap<String, Value>
}

#[bolt_packstream(0x50)]
#[derive(Debug, PartialEq)]
pub struct Path {
  pub nodes: Vec<Node>,
  pub relationships: Vec<UnboundRelationship>,
  pub sequence: Vec<i64>,
}

#[bolt_packstream(0x72)]
#[derive(Debug, PartialEq)]
pub struct UnboundRelationship {
  pub id: i64,
  pub type_name: String,
  pub properties: HashMap<String, Value>
}
