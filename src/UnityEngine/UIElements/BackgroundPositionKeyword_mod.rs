#[cfg(feature = "UnityEngine+UIElements+BackgroundPositionKeyword")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundPositionKeyword {
    Bottom = 2i32,
    Center = 0i32,
    Left = 3i32,
    Right = 4i32,
    Top = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+BackgroundPositionKeyword")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::BackgroundPositionKeyword => "UnityEngine.UIElements"
    ."BackgroundPositionKeyword"
);
