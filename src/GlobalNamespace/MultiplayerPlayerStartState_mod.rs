#[cfg(feature = "MultiplayerPlayerStartState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerPlayerStartState {
    #[default]
    InSync = 0i32,
    Late = 1i32,
}
#[cfg(feature = "MultiplayerPlayerStartState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerPlayerStartState =>
    ""."MultiplayerPlayerStartState"
);
