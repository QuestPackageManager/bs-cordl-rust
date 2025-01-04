#[cfg(feature = "UpdateConnectionStateReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UpdateConnectionStateReason {
    #[default]
    Connected = 2i32,
    ConnectionFailed = 8i32,
    Init = 0i32,
    ManualDisconnect = 6i32,
    OnDestroy = 9i32,
    PlayerOrderChanged = 1i32,
    RemoteDisconnect = 7i32,
    StartDedicatedServer = 4i32,
    StartSession = 3i32,
    SyncTimeInitialized = 5i32,
}
#[cfg(feature = "UpdateConnectionStateReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::UpdateConnectionStateReason =>
    ""."UpdateConnectionStateReason"
);
