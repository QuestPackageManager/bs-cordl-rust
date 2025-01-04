#[cfg(feature = "UnityEngine+TextCore+Text+TextProcessingElementType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextProcessingElementType {
    #[default]
    TextCharacterElement = 1i32,
    TextMarkupElement = 2i32,
    Undefined = 0i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextProcessingElementType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::TextCore::Text::TextProcessingElementType =>
    "UnityEngine.TextCore.Text"."TextProcessingElementType"
);
