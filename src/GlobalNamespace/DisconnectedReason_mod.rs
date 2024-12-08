#[cfg(feature = "DisconnectedReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisconnectedReason {
    ClientConnectionClosed = 8i32,
    Kicked = 4i32,
    MasterServerUnreachable = 7i32,
    NetworkDisconnected = 9i32,
    ServerAtCapacity = 5i32,
    ServerConnectionClosed = 6i32,
    ServerTerminated = 10i32,
    Timeout = 3i32,
    Unknown = 1i32,
    UserInitiated = 2i32,
}
#[cfg(feature = "DisconnectedReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DisconnectedReason => ""
    ."DisconnectedReason"
);
