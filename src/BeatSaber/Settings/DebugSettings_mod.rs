#[cfg(feature = "BeatSaber+Settings+DebugSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DebugSettings {
    pub showBeatmapLevelVersion: bool,
}
#[cfg(feature = "BeatSaber+Settings+DebugSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::DebugSettings =>
    "BeatSaber.Settings"."DebugSettings"
);
#[cfg(feature = "BeatSaber+Settings+DebugSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::DebugSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+DebugSettings")]
impl crate::BeatSaber::Settings::DebugSettings {}
