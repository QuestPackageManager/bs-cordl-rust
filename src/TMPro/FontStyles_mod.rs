#[cfg(feature = "TMPro+FontStyles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FontStyles {
    #[default]
    Bold = 1i32,
    Highlight = 512i32,
    Italic = 2i32,
    LowerCase = 8i32,
    Normal = 0i32,
    SmallCaps = 32i32,
    Strikethrough = 64i32,
    Subscript = 256i32,
    Superscript = 128i32,
    Underline = 4i32,
    UpperCase = 16i32,
}
#[cfg(feature = "TMPro+FontStyles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::FontStyles => "TMPro"."FontStyles"
);
