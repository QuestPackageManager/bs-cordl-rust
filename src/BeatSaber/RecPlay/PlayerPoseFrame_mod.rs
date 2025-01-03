#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrame")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PlayerPoseFrame {
    pub _cordl_time: f32,
    pub pose: crate::BeatSaber::RecPlay::PlayerPose,
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrame")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PlayerPoseFrame =>
    "BeatSaber.RecPlay"."PlayerPoseFrame"
);
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::RecPlay::PlayerPoseFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrame")]
impl crate::BeatSaber::RecPlay::PlayerPoseFrame {}
