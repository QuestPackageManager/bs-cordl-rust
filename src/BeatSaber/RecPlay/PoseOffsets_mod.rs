#[cfg(feature = "BeatSaber+RecPlay+PoseOffsets")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PoseOffsets {
    pub room: crate::UnityEngine::Pose,
    pub leftController: crate::UnityEngine::Pose,
    pub rightController: crate::UnityEngine::Pose,
}
#[cfg(feature = "BeatSaber+RecPlay+PoseOffsets")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PoseOffsets =>
    "BeatSaber.RecPlay"."PoseOffsets"
);
#[cfg(feature = "BeatSaber+RecPlay+PoseOffsets")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::RecPlay::PoseOffsets {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PoseOffsets")]
impl crate::BeatSaber::RecPlay::PoseOffsets {
    pub fn Adjust(
        &mut self,
        pose: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::RecPlay::PlayerPose>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Adjust",
            (pose),
        )?;
        Ok(__cordl_ret.into())
    }
}
