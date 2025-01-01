#[cfg(feature = "BeatSaber+Settings+WindowSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WindowSettings {
    pub fullscreen: bool,
    pub resolution: crate::Unity::Mathematics::int2,
}
#[cfg(feature = "BeatSaber+Settings+WindowSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::WindowSettings =>
    "BeatSaber.Settings"."WindowSettings"
);
#[cfg(feature = "BeatSaber+Settings+WindowSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::WindowSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+WindowSettings")]
impl crate::BeatSaber::Settings::WindowSettings {}
