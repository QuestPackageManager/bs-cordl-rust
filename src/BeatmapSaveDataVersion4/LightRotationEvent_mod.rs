#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LightRotationEvent {
    pub p: i32,
    pub e: crate::BeatmapSaveDataCommon::EaseType,
    pub l: i32,
    pub r: f32,
    pub d: crate::BeatmapSaveDataCommon::RotationDirection,
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::LightRotationEvent =>
    "BeatmapSaveDataVersion4"."LightRotationEvent"
);
#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::LightRotationEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightRotationEvent")]
impl crate::BeatmapSaveDataVersion4::LightRotationEvent {}
