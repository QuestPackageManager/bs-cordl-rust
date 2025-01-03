#[cfg(feature = "BeatSaber+Settings+PerformanceToolSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PerformanceToolSettings {
    pub recordFps: bool,
    pub showFps: bool,
    pub showMemory: bool,
    pub saveScreenshot: bool,
}
#[cfg(feature = "BeatSaber+Settings+PerformanceToolSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::PerformanceToolSettings =>
    "BeatSaber.Settings"."PerformanceToolSettings"
);
#[cfg(feature = "BeatSaber+Settings+PerformanceToolSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::PerformanceToolSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+PerformanceToolSettings")]
impl crate::BeatSaber::Settings::PerformanceToolSettings {}
