#[cfg(feature = "MultiplayerUnavailableReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerUnavailableReason {
    #[default]
    MaintenanceMode = 4i32,
    NetworkUnreachable = 1i32,
    ServerOffline = 3i32,
    UpdateRequired = 2i32,
}
#[cfg(feature = "MultiplayerUnavailableReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerUnavailableReason =>
    ""."MultiplayerUnavailableReason"
);
