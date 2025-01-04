#[cfg(feature = "CannotStartGameReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CannotStartGameReason {
    #[default]
    AllPlayersNotInLobby = 4i32,
    AllPlayersSpectating = 2i32,
    DoNotOwnSong = 5i32,
    NoSongSelected = 3i32,
    None = 1i32,
}
#[cfg(feature = "CannotStartGameReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CannotStartGameReason => ""
    ."CannotStartGameReason"
);
