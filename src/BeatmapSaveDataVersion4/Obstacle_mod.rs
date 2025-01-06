#[cfg(feature = "BeatmapSaveDataVersion4+Obstacle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Obstacle {
    pub x: i32,
    pub y: i32,
    pub d: f32,
    pub w: i32,
    pub h: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+Obstacle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::Obstacle =>
    "BeatmapSaveDataVersion4"."Obstacle"
);
#[cfg(feature = "BeatmapSaveDataVersion4+Obstacle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::Obstacle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+Obstacle")]
impl crate::BeatmapSaveDataVersion4::Obstacle {}
