#[cfg(feature = "BeatmapSaveDataVersion4+Arc")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Arc {
    pub m: f32,
    pub tm: f32,
    pub a: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
}
#[cfg(feature = "BeatmapSaveDataVersion4+Arc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::Arc =>
    "BeatmapSaveDataVersion4"."Arc"
);
#[cfg(feature = "BeatmapSaveDataVersion4+Arc")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::BeatmapSaveDataVersion4::Arc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+Arc")]
impl crate::BeatmapSaveDataVersion4::Arc {}
