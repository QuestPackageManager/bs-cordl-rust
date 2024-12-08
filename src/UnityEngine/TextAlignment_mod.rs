#[cfg(feature = "UnityEngine+TextAlignment")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Center = 1i32,
    Left = 0i32,
    Right = 2i32,
}
#[cfg(feature = "UnityEngine+TextAlignment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextAlignment => "UnityEngine"
    ."TextAlignment"
);
