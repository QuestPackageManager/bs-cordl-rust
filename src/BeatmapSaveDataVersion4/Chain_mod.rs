#[cfg(feature = "BeatmapSaveDataVersion4+Chain")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Chain {
    pub tx: i32,
    pub ty: i32,
    pub c: i32,
    pub s: f32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+Chain")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::Chain =>
    "BeatmapSaveDataVersion4"."Chain"
);
#[cfg(feature = "BeatmapSaveDataVersion4+Chain")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::Chain {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+Chain")]
impl crate::BeatmapSaveDataVersion4::Chain {}
