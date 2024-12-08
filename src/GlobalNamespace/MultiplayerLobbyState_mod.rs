#[cfg(feature = "MultiplayerLobbyState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerLobbyState {
    Error = 5i32,
    GameRunning = 4i32,
    GameStarting = 3i32,
    LobbyCountdown = 2i32,
    LobbySetup = 1i32,
    None = 0i32,
}
#[cfg(feature = "MultiplayerLobbyState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerLobbyState => ""
    ."MultiplayerLobbyState"
);
