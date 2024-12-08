#[cfg(feature = "MultiplayerPlayerLayout")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerPlayerLayout {
    Circle = 1i32,
    Duel = 2i32,
    NotDetermined = 0i32,
}
#[cfg(feature = "MultiplayerPlayerLayout")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for MultiplayerPlayerLayout => ""."MultiplayerPlayerLayout"
);
