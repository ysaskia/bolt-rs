# WORK IN PROGRESS (DO NOT USE)

Bolt driver (Version 1) [Neo4j](https://neo4j.com/) graph database.

#### Working usage to benchmark pack/unpack performance

```rust
use packstream_v1::structs::*;
use packstream_v1::value::Value;
use std::time::Instant;
use packstream_core::error::BoltError;
use packstream_core::packer::{Packer, PackValue};
use packstream_core::unpacker::{Unpacker, UnpackValue};
use std::collections::HashMap;
use maplit::hashmap;

fn test_type<T>(title: &str, type_name: &str, raw: T) -> Result<(), BoltError>
  where Packer: PackValue<T>,
        Unpacker: UnpackValue<T>,
        T: std::fmt::Debug {
  println!();
  println!("{}: {:?}", title, raw);

  let now = Instant::now();
  let mut packer = Packer::new();
  packer.pack(raw).map(|_| {
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) +
              (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Packed   => {}({:?})", type_name, packer.out.buf);
    println!("Packed   => Seconds {}", sec);

    let now = Instant::now();
    let mut unpacker = Unpacker::new(packer.out.buf[..].to_vec());
    let val:Result<T, BoltError> = unpacker.unpack();
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) +
              (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Unpacked => {:?}", val);
    println!("Unpacked => Seconds {}", sec);
  })
}


fn test_node() -> Node {
  Node {
    id: 1,
    labels: vec!["toto".into()],
    properties: test_any_val_dict()
  }
}

fn test_any_val_dict() -> HashMap<String, Value> {
  hashmap! {
    "a".into() => Value::Int(1),
    "b".into() => Value::String("a".into()),
    "c".into() => Value::Float(1.1),
    "d".into() => Value::List(vec![
      Value::Int(1),
      Value::String("2".into()),
      Value::Float(3.0)
    ])
  }
}

fn test_alpha_dict() -> HashMap<String, i32> {
  hashmap! {
    "a".into() => 1,
    "b".into() => 2,
    "c".into() => 3,
    "d".into() => 4,
    "e".into() => 5,
    "f".into() => 6,
    "g".into() => 7,
    "h".into() => 8,
    "i".into() => 9,
    "j".into() => 10,
    "k".into() => 11,
    "l".into() => 12,
    "m".into() => 13,
    "n".into() => 14,
    "o".into() => 15,
    "p".into() => 16,
    "q".into() => 17,
    "r".into() => 18,
    "s".into() => 19,
    "t".into() => 20,
    "u".into() => 21,
    "v".into() => 22,
    "w".into() => 23,
    "x".into() => 24,
    "y".into() => 25,
    "z".into() => 26,
  }
}

fn main() -> Result<(), BoltError> {
  test_type("BOOLEAN TRUE TEST",        "Boolean",  true)?;
  test_type("BOOLEAN FALSE TEST",       "Boolean",  false)?;
  test_type("MIN -INT_64 TEST",         "Int",     -9_223_372_036_854_775_808 as i64)?;
  test_type("MIN -INT_64 TEST",         "Int",     -2_147_483_649 as i64)?;
  test_type("MIN -INT_32 TEST",         "Int",     -2_147_483_648)?;
  test_type("MAX -INT_32 TEST",         "Int",     -32_769)?;
  test_type("MIN -INT_16 TEST",         "Int",     -32_768)?;
  test_type("MAX -INT_16 TEST",         "Int",     -129)?;
  test_type("MIN -INT_8 TEST",          "Int",     -128)?;
  test_type("MAX -INT_8 TEST",          "Int",     -17)?;
  test_type("MIN -TINY_INT TEST",       "Int",     -16)?;
  test_type("MAX +TINY_INT TEST",       "Int",      127)?;
  test_type("MIN -INT_16 TEST",         "Int",      128)?;
  test_type("MAX +INT_16 TEST",         "Int",      32_767)?;
  test_type("MIN +INT_32 TEST",         "Int",      32_768)?;
  test_type("MAX +INT_32 TEST",         "Int",      2_147_483_647)?;
  test_type("MIN +INT_64 TEST",         "Int",      2_147_483_648 as i64)?;
  test_type("MAX +INT_64 TEST",         "Int",      9_223_372_036_854_775_807 as i64)?;
  test_type("MIN FLOAT TEST",           "Float",    std::f64::MIN)?;
  test_type("MAX FLOAT TEST",           "Float",    std::f64::MAX)?;
  test_type("STRING_8 TEST",            "String",  "abcdefghijklmnopqrstuvwxyz".to_string())?;
  test_type("STRING SPECIAL CHAR TEST", "String",  "En å flöt över ängen".to_string())?;
  test_type("TINY_LIST",                "List",     vec![1, 2, 3])?;
  test_type("MAP",                      "Map",      test_alpha_dict())?;
  test_type("STRUCT",                   "Node",     test_node())
}
```
