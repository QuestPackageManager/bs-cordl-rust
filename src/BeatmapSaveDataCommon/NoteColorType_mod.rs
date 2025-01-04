#[cfg(feature = "BeatmapSaveDataCommon+NoteColorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoteColorType {
    #[default]
    ColorA = 0i32,
    ColorB = 1i32,
}
#[cfg(feature = "BeatmapSaveDataCommon+NoteColorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataCommon::NoteColorType =>
    "BeatmapSaveDataCommon"."NoteColorType"
);
