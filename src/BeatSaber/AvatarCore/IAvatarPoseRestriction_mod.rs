#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct IAvatarPoseRestriction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::IAvatarPoseRestriction =>
    "BeatSaber.AvatarCore"."IAvatarPoseRestriction"
);
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
impl crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
    pub fn RestrictPose(
        &mut self,
        headRotation: crate::UnityEngine::Quaternion,
        headPosition: crate::UnityEngine::Vector3,
        leftHandPosition: crate::UnityEngine::Vector3,
        rightHandPosition: crate::UnityEngine::Vector3,
        newHeadPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        newLeftHandPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector3,
        >,
        newRightHandPosition: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Vector3,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RestrictPose",
                (
                    headRotation,
                    headPosition,
                    leftHandPosition,
                    rightHandPosition,
                    newHeadPosition,
                    newLeftHandPosition,
                    newRightHandPosition,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
