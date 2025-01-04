#[cfg(feature = "MultiplayerPlayerLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerPlayerLayout {
    #[default]
    Circle = 1i32,
    Duel = 2i32,
    NotDetermined = 0i32,
}
#[cfg(feature = "MultiplayerPlayerLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerPlayerLayout => ""
    ."MultiplayerPlayerLayout"
);
