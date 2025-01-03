#[cfg(feature = "BeatmapSaveDataVersion4+ColorNote")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ColorNote {
    pub x: i32,
    pub y: i32,
    pub a: i32,
    pub c: crate::BeatmapSaveDataCommon::NoteColorType,
    pub d: crate::BeatmapSaveDataCommon::NoteCutDirection,
}
#[cfg(feature = "BeatmapSaveDataVersion4+ColorNote")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::ColorNote =>
    "BeatmapSaveDataVersion4"."ColorNote"
);
#[cfg(feature = "BeatmapSaveDataVersion4+ColorNote")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::ColorNote {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ColorNote")]
impl crate::BeatmapSaveDataVersion4::ColorNote {}
