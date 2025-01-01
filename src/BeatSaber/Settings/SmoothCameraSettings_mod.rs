#[cfg(feature = "BeatSaber+Settings+SmoothCameraSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SmoothCameraSettings {
    pub enabled: bool,
    pub fov: f32,
    pub smoothPosition: f32,
    pub smoothRotation: f32,
    pub thirdPersonEnabled: bool,
    pub thirdPersonPosition: crate::Unity::Mathematics::float3,
    pub thirdPersonRotation: crate::Unity::Mathematics::float3,
}
#[cfg(feature = "BeatSaber+Settings+SmoothCameraSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SmoothCameraSettings =>
    "BeatSaber.Settings"."SmoothCameraSettings"
);
#[cfg(feature = "BeatSaber+Settings+SmoothCameraSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::Settings::SmoothCameraSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+Settings+SmoothCameraSettings")]
impl crate::BeatSaber::Settings::SmoothCameraSettings {}
