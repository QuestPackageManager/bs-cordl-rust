#[cfg(feature = "UnityEngine+TextCore+Text+TextWrappingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextWrappingMode {
    #[default]
    NoWrap = 0i32,
    Normal = 1i32,
    PreserveWhitespace = 2i32,
    PreserveWhitespaceNoWrap = 3i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextWrappingMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextWrappingMode =>
    "UnityEngine.TextCore.Text"."TextWrappingMode"
);
