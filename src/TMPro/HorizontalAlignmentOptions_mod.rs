#[cfg(feature = "TMPro+HorizontalAlignmentOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlignmentOptions {
    Center = 2i32,
    Flush = 16i32,
    Geometry = 32i32,
    Justified = 8i32,
    Left = 1i32,
    Right = 4i32,
}
#[cfg(feature = "TMPro+HorizontalAlignmentOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::HorizontalAlignmentOptions => "TMPro"
    ."HorizontalAlignmentOptions"
);
