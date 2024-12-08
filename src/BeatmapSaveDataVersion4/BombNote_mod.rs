#[cfg(feature = "BeatmapSaveDataVersion4+BombNote")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BombNote {
    pub x: i32,
    pub y: i32,
}
#[cfg(feature = "BeatmapSaveDataVersion4+BombNote")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::BombNote =>
    "BeatmapSaveDataVersion4"."BombNote"
);
#[cfg(feature = "BeatmapSaveDataVersion4+BombNote")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatmapSaveDataVersion4::BombNote {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+BombNote")]
impl crate::BeatmapSaveDataVersion4::BombNote {}
