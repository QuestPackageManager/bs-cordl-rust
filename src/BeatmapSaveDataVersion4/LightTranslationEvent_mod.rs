#[cfg(feature = "BeatmapSaveDataVersion4+LightTranslationEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LightTranslationEvent {
    pub p: i32,
    pub e: crate::BeatmapSaveDataCommon::EaseType,
    pub t: f32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightTranslationEvent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::LightTranslationEvent
    => "BeatmapSaveDataVersion4"."LightTranslationEvent"
);
#[cfg(feature = "BeatmapSaveDataVersion4+LightTranslationEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::LightTranslationEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightTranslationEvent")]
impl crate::BeatmapSaveDataVersion4::LightTranslationEvent {}
