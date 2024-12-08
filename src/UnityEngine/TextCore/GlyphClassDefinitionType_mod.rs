#[cfg(feature = "UnityEngine+TextCore+GlyphClassDefinitionType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlyphClassDefinitionType {
    Base = 1i32,
    Component = 4i32,
    Ligature = 2i32,
    Mark = 3i32,
    Undefined = 0i32,
}
#[cfg(feature = "UnityEngine+TextCore+GlyphClassDefinitionType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::GlyphClassDefinitionType
    => "UnityEngine.TextCore"."GlyphClassDefinitionType"
);
