#[cfg(feature = "BeatmapSaveDataVersion4+BasicEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BasicEvent {
    pub t: crate::BeatmapSaveDataCommon::BeatmapEventType,
    pub i: i32,
    pub f: f32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+BasicEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::BasicEvent =>
    "BeatmapSaveDataVersion4"."BasicEvent"
);
#[cfg(feature = "BeatmapSaveDataVersion4+BasicEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::BasicEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BasicEvent")]
impl crate::BeatmapSaveDataVersion4::BasicEvent {}
