#[cfg(feature = "BeatSaber+AvatarCore+AvatarPoseData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AvatarPoseData {
    pub headPose: crate::UnityEngine::Pose,
    pub leftHandPose: crate::UnityEngine::Pose,
    pub rightHandPose: crate::UnityEngine::Pose,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarPoseData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarPoseData =>
    "BeatSaber.AvatarCore"."AvatarPoseData"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarPoseData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::BeatSaber::AvatarCore::AvatarPoseData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarPoseData")]
impl crate::BeatSaber::AvatarCore::AvatarPoseData {
    pub fn _ctor(
        &mut self,
        headPose: crate::UnityEngine::Pose,
        leftHandPose: crate::UnityEngine::Pose,
        rightHandPose: crate::UnityEngine::Pose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (headPose, leftHandPose, rightHandPose),
        )?;
        Ok(__cordl_ret.into())
    }
}
