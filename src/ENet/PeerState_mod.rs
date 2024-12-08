#[cfg(feature = "ENet+PeerState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerState {
    AcknowledgingConnect = 2i32,
    AcknowledgingDisconnect = 8i32,
    Connected = 5i32,
    Connecting = 1i32,
    ConnectionPending = 3i32,
    ConnectionSucceeded = 4i32,
    DisconnectLater = 6i32,
    Disconnected = 0i32,
    Disconnecting = 7i32,
    Uninitialized = -1i32,
    Zombie = 9i32,
}
#[cfg(feature = "ENet+PeerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::PeerState => "ENet"."PeerState"
);
