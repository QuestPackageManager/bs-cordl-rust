#[cfg(feature = "UnityEngine+TextCore+Text+TextureMapping")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureMapping {
    Character = 0i32,
    Line = 1i32,
    MatchAspect = 3i32,
    Paragraph = 2i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextureMapping")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextureMapping =>
    "UnityEngine.TextCore.Text"."TextureMapping"
);
