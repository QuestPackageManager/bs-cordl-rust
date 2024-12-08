#[cfg(feature = "UnityEngine+UIElements+TextOverflowPosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextOverflowPosition {
    End = 0i32,
    Middle = 2i32,
    Start = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+TextOverflowPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TextOverflowPosition =>
    "UnityEngine.UIElements"."TextOverflowPosition"
);
