#[cfg(feature = "MultiplayerPlayerStartState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerPlayerStartState {
    InSync = 0i32,
    Late = 1i32,
}
#[cfg(feature = "MultiplayerPlayerStartState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for MultiplayerPlayerStartState => ""
    ."MultiplayerPlayerStartState"
);
