#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct IAvatarPoseRestriction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "IAvatarPoseRestriction";
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
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseRestriction")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Quaternion,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            crate::UnityEngine::Vector3,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("RestrictPose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RestrictPose", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        headRotation,
                        headPosition,
                        leftHandPosition,
                        rightHandPosition,
                        newHeadPosition,
                        newLeftHandPosition,
                        newRightHandPosition,
                    ),
                )?
        };
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
