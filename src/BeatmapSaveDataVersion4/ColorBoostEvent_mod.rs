#[cfg(feature = "BeatmapSaveDataVersion4+ColorBoostEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ColorBoostEvent {
    pub b: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+ColorBoostEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::ColorBoostEvent =>
    "BeatmapSaveDataVersion4"."ColorBoostEvent"
);
#[cfg(feature = "BeatmapSaveDataVersion4+ColorBoostEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::ColorBoostEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+ColorBoostEvent")]
impl crate::BeatmapSaveDataVersion4::ColorBoostEvent {}
