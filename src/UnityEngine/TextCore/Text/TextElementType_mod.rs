#[cfg(feature = "UnityEngine+TextCore+Text+TextElementType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextElementType {
    #[default]
    Character = 1u8,
    Sprite = 2u8,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextElementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextElementType =>
    "UnityEngine.TextCore.Text"."TextElementType"
);
