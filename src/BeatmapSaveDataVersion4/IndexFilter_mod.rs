#[cfg(feature = "BeatmapSaveDataVersion4+IndexFilter")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IndexFilter {
    pub f: crate::BeatmapSaveDataCommon::IndexFilterType,
    pub p: i32,
    pub t: i32,
    pub r: i32,
    pub c: i32,
    pub n: crate::BeatmapSaveDataCommon::IndexFilterRandomType,
    pub s: i32,
    pub l: f32,
    pub d: crate::BeatmapSaveDataCommon::IndexFilterLimitAlsoAffectsType,
}
#[cfg(feature = "BeatmapSaveDataVersion4+IndexFilter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::IndexFilter =>
    "BeatmapSaveDataVersion4"."IndexFilter"
);
#[cfg(feature = "BeatmapSaveDataVersion4+IndexFilter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::IndexFilter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+IndexFilter")]
impl crate::BeatmapSaveDataVersion4::IndexFilter {}
