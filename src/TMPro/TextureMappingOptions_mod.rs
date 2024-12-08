#[cfg(feature = "TMPro+TextureMappingOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureMappingOptions {
    Character = 0i32,
    Line = 1i32,
    MatchAspect = 3i32,
    Paragraph = 2i32,
}
#[cfg(feature = "TMPro+TextureMappingOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TextureMappingOptions => "TMPro"
    ."TextureMappingOptions"
);
