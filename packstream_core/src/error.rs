#[derive(Debug)]
pub enum BoltError {
    Unmanaged,
    ConnectionFailure,
    WriteHandshakeFailure,
    WriteEndOfRequestFailure,
    WriteRequestHeaderFailure,
    WriteRequestChunkFailure,
    ReadHandshakeFailure,
    PackInputPeekInvalidRange,
    PackInputPeekAtInvalidRange,
    PackInputReadInvalidIndex,
    PackInputReadInvalidSliceRange,
    UnpackInvalidStringSize,
    UnpackInvalidListSize,
    UnpackInvalidMapSize,
    UnpackInvalidStructSize,
    UnpackInvalidBooleanMarker,
    UnpackInvalidIntMarker,
    PackHeaderSizeOverflow,
    PackStructHeaderSizeOverflow
}

impl std::error::Error for BoltError {}
impl std::fmt::Display for BoltError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unpacker error")
    }
}
