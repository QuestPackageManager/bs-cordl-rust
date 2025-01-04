#[cfg(feature = "TMPro+TextAlignmentOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlignmentOptions {
    #[default]
    Baseline = 2050i32,
    BaselineFlush = 2064i32,
    BaselineGeoAligned = 2080i32,
    BaselineJustified = 2056i32,
    BaselineLeft = 2049i32,
    BaselineRight = 2052i32,
    Bottom = 1026i32,
    BottomFlush = 1040i32,
    BottomGeoAligned = 1056i32,
    BottomJustified = 1032i32,
    BottomLeft = 1025i32,
    BottomRight = 1028i32,
    Capline = 8194i32,
    CaplineFlush = 8208i32,
    CaplineGeoAligned = 8224i32,
    CaplineJustified = 8200i32,
    CaplineLeft = 8193i32,
    CaplineRight = 8196i32,
    Center = 514i32,
    CenterGeoAligned = 544i32,
    Converted = 65535i32,
    Flush = 528i32,
    Justified = 520i32,
    Left = 513i32,
    Midline = 4098i32,
    MidlineFlush = 4112i32,
    MidlineGeoAligned = 4128i32,
    MidlineJustified = 4104i32,
    MidlineLeft = 4097i32,
    MidlineRight = 4100i32,
    Right = 516i32,
    Top = 258i32,
    TopFlush = 272i32,
    TopGeoAligned = 288i32,
    TopJustified = 264i32,
    TopLeft = 257i32,
    TopRight = 260i32,
}
#[cfg(feature = "TMPro+TextAlignmentOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TextAlignmentOptions => "TMPro"
    ."TextAlignmentOptions"
);
