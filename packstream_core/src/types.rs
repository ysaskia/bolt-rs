use crate::consts::*;

/**
 * Primitive types that PackStream can represent. They map to the non-graph
 * primitives of the Neo4j type system. Graph primitives and rich composites
 * are represented as [Struct](PackType::Struct).
 */
pub enum PackType {
    Null,
    Boolean,
    Int,
    Float,
    Bytes,
    String,
    List,
    Map,
    Struct,
    EndOfStream,
    Reserved,
}

impl PackType {
    /**
     * Gets primitive type from the specified `marker` argument. If a `marker`
     * is not handled by the PackStream specification returns:
     * [Reserved](PackType::Reserved)
     */
    pub fn from(marker: u8) -> Self {
        match marker & HIGH_NIBBLE {
            TINY_STRING..=0x8F => PackType::String,
            TINY_LIST..=0x9F   => PackType::List,
            TINY_MAP..=0xAF    => PackType::Map,
            TINY_STRUCT..=0xBF => PackType::Struct,
            _ => if marker >= (TINY_MIN as u8) { PackType::Int } else {
                match marker {
                    /* BOOLEAN ***********************************************/
                      FALSE
                    | TRUE => PackType::Boolean,
                    /* BYTE **************************************************/
                      BYTES_8
                    | BYTES_16
                    | BYTES_32 => PackType::Bytes,
                    /* INTEGER ***********************************************/
                      TINY_INT_PF..=TINY_INT_PL
                    | TINY_INT_NF..=TINY_INT_NL
                    | INT_8
                    | INT_16
                    | INT_32
                    | INT_64 => PackType::Int,
                    /* FLOAT *************************************************/
                      FLOAT_64 => PackType::Float,
                    /* STRING ************************************************/
                      STRING_8
                    | STRING_16
                    | STRING_32 => PackType::String,
                      LIST_8
                    | LIST_16
                    | LIST_32
                    | LIST_STREAM => PackType::List,
                    /* MAP<K,V> **********************************************/
                      MAP_8
                    | MAP_16
                    | MAP_32
                    | MAP_STREAM => PackType::Map,
                    /* STRUCT ************************************************/
                      STRUCT_8
                    | STRUCT_16 => PackType::Struct,
                    /* STREAM ************************************************/
                    END_OF_STREAM => PackType::EndOfStream,
                    _ => PackType::Reserved
                }
            }
        }
    }
}
