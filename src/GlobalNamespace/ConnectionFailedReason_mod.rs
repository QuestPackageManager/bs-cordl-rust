#[cfg(feature = "ConnectionFailedReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionFailedReason {
    AuthenticationFailed = 10i32,
    CertificateValidationFailed = 12i32,
    ConnectionCanceled = 2i32,
    FailedToFindMatch = 15i32,
    InvalidPassword = 8i32,
    MultiplayerApiUnreachable = 9i32,
    NetworkNotConnected = 11i32,
    None = 0i32,
    ServerAlreadyExists = 4i32,
    ServerAtCapacity = 6i32,
    ServerDoesNotExist = 5i32,
    ServerIsTerminating = 13i32,
    ServerUnreachable = 3i32,
    Timeout = 14i32,
    Unknown = 1i32,
    VersionMismatch = 7i32,
}
#[cfg(feature = "ConnectionFailedReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ConnectionFailedReason => ""
    ."ConnectionFailedReason"
);
