#[cfg(feature = "UnityEngine+TextCore+Text+TextOverflowMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextOverflowMode {
    #[default]
    Ellipsis = 1i32,
    Linked = 6i32,
    Masking = 2i32,
    Overflow = 0i32,
    Page = 5i32,
    ScrollRect = 4i32,
    Truncate = 3i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextOverflowMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextOverflowMode =>
    "UnityEngine.TextCore.Text"."TextOverflowMode"
);
