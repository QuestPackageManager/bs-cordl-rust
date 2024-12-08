#[cfg(feature = "TMPro+VerticalAlignmentOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignmentOptions {
    Baseline = 2048i32,
    Bottom = 1024i32,
    Capline = 8192i32,
    Geometry = 4096i32,
    Middle = 512i32,
    Top = 256i32,
}
#[cfg(feature = "TMPro+VerticalAlignmentOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::VerticalAlignmentOptions => "TMPro"
    ."VerticalAlignmentOptions"
);
