#[cfg(feature = "BeatmapSaveDataVersion4+SpawnRotation")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SpawnRotation {
    pub t: crate::BeatmapSaveDataCommon::ExecutionTime,
    pub r: f32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+SpawnRotation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::SpawnRotation =>
    "BeatmapSaveDataVersion4"."SpawnRotation"
);
#[cfg(feature = "BeatmapSaveDataVersion4+SpawnRotation")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::SpawnRotation {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+SpawnRotation")]
impl crate::BeatmapSaveDataVersion4::SpawnRotation {}
