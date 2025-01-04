#[cfg(feature = "Oculus+Platform+NetSyncDisconnectReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetSyncDisconnectReason {
    #[default]
    Failed = 3i32,
    LocalTerminated = 1i32,
    Lost = 4i32,
    ServerTerminated = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Oculus+Platform+NetSyncDisconnectReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::NetSyncDisconnectReason =>
    "Oculus.Platform"."NetSyncDisconnectReason"
);
