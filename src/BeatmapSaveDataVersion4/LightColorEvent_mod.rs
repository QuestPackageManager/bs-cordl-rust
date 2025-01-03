#[cfg(feature = "BeatmapSaveDataVersion4+LightColorEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LightColorEvent {
    pub p: i32,
    pub e: crate::BeatmapSaveDataCommon::EaseType,
    pub c: crate::BeatmapSaveDataCommon::EnvironmentColorType,
    pub b: f32,
    pub f: i32,
    pub sb: f32,
    pub sf: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightColorEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::LightColorEvent =>
    "BeatmapSaveDataVersion4"."LightColorEvent"
);
#[cfg(feature = "BeatmapSaveDataVersion4+LightColorEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::LightColorEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightColorEvent")]
impl crate::BeatmapSaveDataVersion4::LightColorEvent {}
