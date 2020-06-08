use std::collections::HashMap;
use crate::structs::Struct;

#[derive(Debug, PartialEq)]
pub enum Value {
  Null,
  Boolean (bool),
  Int     (i64),
  Float   (f64),
  String  (String),
  List    (Vec<Value>),
  Map     (HashMap<String, Value>),
  Struct  (Struct)
}
