#[cfg(feature = "BeatSaber+Settings+AudioSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AudioSettings {
    pub volume: f32,
    pub ambientVolumeScale: f32,
    pub latency: f32,
    pub overrideLatency: bool,
}
#[cfg(feature = "BeatSaber+Settings+AudioSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::AudioSettings =>
    "BeatSaber.Settings"."AudioSettings"
);
#[cfg(feature = "BeatSaber+Settings+AudioSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::AudioSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+AudioSettings")]
impl crate::BeatSaber::Settings::AudioSettings {}
