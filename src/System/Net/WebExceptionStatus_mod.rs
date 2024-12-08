#[cfg(feature = "System+Net+WebExceptionStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebExceptionStatus {
    CacheEntryNotFound = 18i32,
    ConnectFailure = 2i32,
    ConnectionClosed = 8i32,
    KeepAliveFailure = 12i32,
    MessageLengthLimitExceeded = 17i32,
    NameResolutionFailure = 1i32,
    Pending = 13i32,
    PipelineFailure = 5i32,
    ProtocolError = 7i32,
    ProxyNameResolutionFailure = 15i32,
    ReceiveFailure = 3i32,
    RequestCanceled = 6i32,
    RequestProhibitedByCachePolicy = 19i32,
    RequestProhibitedByProxy = 20i32,
    SecureChannelFailure = 10i32,
    SendFailure = 4i32,
    ServerProtocolViolation = 11i32,
    Success = 0i32,
    Timeout = 14i32,
    TrustFailure = 9i32,
    UnknownError = 16i32,
}
#[cfg(feature = "System+Net+WebExceptionStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::WebExceptionStatus => "System.Net"
    ."WebExceptionStatus"
);
