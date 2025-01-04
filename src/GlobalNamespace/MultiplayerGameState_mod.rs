#[cfg(feature = "MultiplayerGameState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerGameState {
    #[default]
    Game = 2i32,
    Lobby = 1i32,
    None = 0i32,
}
#[cfg(feature = "MultiplayerGameState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerGameState => ""
    ."MultiplayerGameState"
);
