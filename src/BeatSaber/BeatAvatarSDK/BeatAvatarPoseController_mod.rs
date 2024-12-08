#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarPoseController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _headTransform: *mut crate::UnityEngine::Transform,
    pub _leftHandTransform: *mut crate::UnityEngine::Transform,
    pub _rightHandTransform: *mut crate::UnityEngine::Transform,
    pub _bodyTransform: *mut crate::UnityEngine::Transform,
    pub _headBodyOffset: *mut crate::BeatSaber::BeatAvatarSDK::HeadBodyOffsetSO,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController => "BeatSaber.BeatAvatarSDK"
    ."BeatAvatarPoseController"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
impl crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateTransforms(
        &mut self,
        headPosition: crate::UnityEngine::Vector3,
        leftHandPosition: crate::UnityEngine::Vector3,
        rightHandPosition: crate::UnityEngine::Vector3,
        headRotation: crate::UnityEngine::Quaternion,
        leftHandRotation: crate::UnityEngine::Quaternion,
        rightHandRotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UpdateTransforms",
                (
                    headPosition,
                    leftHandPosition,
                    rightHandPosition,
                    headRotation,
                    leftHandRotation,
                    rightHandRotation,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateBodyPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBodyPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyWorldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_bodyWorldPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
