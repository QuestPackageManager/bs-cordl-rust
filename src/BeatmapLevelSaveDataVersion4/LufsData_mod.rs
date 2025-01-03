#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LufsData {
    pub si: i32,
    pub ei: i32,
    pub l: f32,
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapLevelSaveDataVersion4::LufsData =>
    "BeatmapLevelSaveDataVersion4"."LufsData"
);
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapLevelSaveDataVersion4::LufsData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelSaveDataVersion4+LufsData")]
impl crate::BeatmapLevelSaveDataVersion4::LufsData {}
