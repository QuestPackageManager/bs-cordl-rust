#[cfg(feature = "UnityEngine+FontStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Bold = 1i32,
    BoldAndItalic = 3i32,
    Italic = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "UnityEngine+FontStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::FontStyle => "UnityEngine"
    ."FontStyle"
);
