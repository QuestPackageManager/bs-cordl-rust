#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrames")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PlayerPoseFrames {
    pub head: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatSaber::RecPlay::PoseFrame,
    >,
    pub leftHand: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatSaber::RecPlay::PoseFrame,
    >,
    pub rightHand: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatSaber::RecPlay::PoseFrame,
    >,
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrames")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PlayerPoseFrames =>
    "BeatSaber.RecPlay"."PlayerPoseFrames"
);
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrames")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::RecPlay::PlayerPoseFrames {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseFrames")]
impl crate::BeatSaber::RecPlay::PlayerPoseFrames {}
