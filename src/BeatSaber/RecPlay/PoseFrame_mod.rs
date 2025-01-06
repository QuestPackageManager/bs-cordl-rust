#[cfg(feature = "BeatSaber+RecPlay+PoseFrame")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PoseFrame {
    pub _cordl_time: f32,
    pub pose: crate::UnityEngine::Pose,
}
#[cfg(feature = "BeatSaber+RecPlay+PoseFrame")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PoseFrame =>
    "BeatSaber.RecPlay"."PoseFrame"
);
#[cfg(feature = "BeatSaber+RecPlay+PoseFrame")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::RecPlay::PoseFrame {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseFrame")]
impl crate::BeatSaber::RecPlay::PoseFrame {}
