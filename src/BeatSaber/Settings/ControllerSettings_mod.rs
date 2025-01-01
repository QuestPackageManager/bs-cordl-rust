#[cfg(feature = "BeatSaber+Settings+ControllerSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ControllerSettings {
    pub position: crate::Unity::Mathematics::float3,
    pub rotation: crate::Unity::Mathematics::float3,
    pub hapticFeedback: bool,
    pub selectedProfile: crate::BeatSaber::Settings::SelectedProfile,
}
#[cfg(feature = "BeatSaber+Settings+ControllerSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::ControllerSettings =>
    "BeatSaber.Settings"."ControllerSettings"
);
#[cfg(feature = "BeatSaber+Settings+ControllerSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::ControllerSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+ControllerSettings")]
impl crate::BeatSaber::Settings::ControllerSettings {}
