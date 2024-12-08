#[cfg(feature = "MultiplayerPlacementErrorCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerPlacementErrorCode {
    AuthenticationFailed = 5i32,
    ConnectionCanceled = 2i32,
    MatchmakingTimeout = 7i32,
    RequestTimeout = 6i32,
    ServerAtCapacity = 4i32,
    ServerDoesNotExist = 3i32,
    Success = 0i32,
    Unknown = 1i32,
}
#[cfg(feature = "MultiplayerPlacementErrorCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerPlacementErrorCode
    => ""."MultiplayerPlacementErrorCode"
);
