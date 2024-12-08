#[cfg(feature = "UnityEngine+TextCore+Text+TextAlignment")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    BaselineCenter = 2050i32,
    BaselineFlush = 2064i32,
    BaselineGeoAligned = 2080i32,
    BaselineJustified = 2056i32,
    BaselineLeft = 2049i32,
    BaselineRight = 2052i32,
    BottomCenter = 1026i32,
    BottomFlush = 1040i32,
    BottomGeoAligned = 1056i32,
    BottomJustified = 1032i32,
    BottomLeft = 1025i32,
    BottomRight = 1028i32,
    CaplineCenter = 8194i32,
    CaplineFlush = 8208i32,
    CaplineGeoAligned = 8224i32,
    CaplineJustified = 8200i32,
    CaplineLeft = 8193i32,
    CaplineRight = 8196i32,
    MiddleCenter = 514i32,
    MiddleFlush = 528i32,
    MiddleGeoAligned = 544i32,
    MiddleJustified = 520i32,
    MiddleLeft = 513i32,
    MiddleRight = 516i32,
    MidlineCenter = 4098i32,
    MidlineFlush = 4112i32,
    MidlineGeoAligned = 4128i32,
    MidlineJustified = 4104i32,
    MidlineLeft = 4097i32,
    MidlineRight = 4100i32,
    TopCenter = 258i32,
    TopFlush = 272i32,
    TopGeoAligned = 288i32,
    TopJustified = 264i32,
    TopLeft = 257i32,
    TopRight = 260i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextAlignment")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextAlignment =>
    "UnityEngine.TextCore.Text"."TextAlignment"
);
