#[cfg(feature = "BeatSaber+Settings+RoomSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RoomSettings {
    pub center: crate::Unity::Mathematics::float3,
    pub rotation: f32,
}
#[cfg(feature = "BeatSaber+Settings+RoomSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::RoomSettings =>
    "BeatSaber.Settings"."RoomSettings"
);
#[cfg(feature = "BeatSaber+Settings+RoomSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::RoomSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+RoomSettings")]
impl crate::BeatSaber::Settings::RoomSettings {}
