#[cfg(feature = "BeatmapSaveDataCommon+RotationDirection")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RotationDirection {
    #[default]
    Automatic = 0i32,
    Clockwise = 1i32,
    Counterclockwise = 2i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+RotationDirection")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::RotationDirection =>
    "BeatmapSaveDataCommon"."RotationDirection"
);
