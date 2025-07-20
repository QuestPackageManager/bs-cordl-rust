#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarPoseController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _headTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _leftHandTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _rightHandTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _bodyTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _headBodyOffset: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::HeadBodyOffsetSO,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarPoseController")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarSDK";
    const CLASS_NAME: &'static str = "BeatAvatarPoseController";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateBodyPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateBodyPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateBodyPosition", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Quaternion,
                    crate::UnityEngine::Quaternion,
                    crate::UnityEngine::Quaternion,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("UpdateTransforms")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateTransforms", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        headPosition,
                        leftHandPosition,
                        rightHandPosition,
                        headRotation,
                        leftHandRotation,
                        rightHandRotation,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_bodyWorldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector3,
                0usize,
            >("get_bodyWorldPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController as
                    quest_hook::libil2cpp::Type > ::class(), "get_bodyWorldPosition",
                    0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
