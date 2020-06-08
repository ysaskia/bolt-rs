This package adds procedural macro `bolt_packstream` which impl `Packer` and 
`Unpacker` for a given struct.

Here is how the native Neo4j `Node` type is implemented in `packstream_core`:

```rust
#[bolt_packstream(0x4E)]
#[derive(Debug, PartialEq)]
pub struct Node {
  pub id: i64,
  pub labels: Vec<String>,
  pub properties: HashMap<String, Value>
}
```

### Work in progress

Another proc macro `bolt_enum` which impl `Packer` and `Unpacker` for enum 
variants.
