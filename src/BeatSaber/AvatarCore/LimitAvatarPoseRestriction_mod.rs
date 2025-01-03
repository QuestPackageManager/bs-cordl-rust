#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
#[repr(C)]
#[derive(Debug)]
pub struct LimitAvatarPoseRestriction {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _parameters: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction_Parameters,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::LimitAvatarPoseRestriction => "BeatSaber.AvatarCore"
    ."LimitAvatarPoseRestriction"
);
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
impl crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction {
    #[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction+Parameters")]
    pub type Parameters = crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction_Parameters;
    pub fn LimitHandPositionRelativeToHead(
        &mut self,
        handPosition: crate::UnityEngine::Vector3,
        headCenter: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("LimitHandPositionRelativeToHead", (handPosition, headCenter))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
impl AsRef<crate::BeatSaber::AvatarCore::IAvatarPoseRestriction>
for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction {
    fn as_ref(&self) -> &crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction")]
impl AsMut<crate::BeatSaber::AvatarCore::IAvatarPoseRestriction>
for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction {
    fn as_mut(&mut self) -> &mut crate::BeatSaber::AvatarCore::IAvatarPoseRestriction {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction+Parameters")]
#[repr(C)]
#[derive(Debug)]
pub struct LimitAvatarPoseRestriction_Parameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub maxHeadSquareDistanceFromCenter: f32,
    pub minHeadYPos: f32,
    pub maxHeadYPos: f32,
    pub minHandXZSquareDistanceFromHeadCenter: f32,
    pub maxHandXZSquareDistanceFromHeadCenter: f32,
    pub minHandYDistanceFromHeadCenter: f32,
    pub maxHandYDistanceFromHeadCenter: f32,
    pub forceHeadPosition: bool,
    pub centerHeadOffset: crate::UnityEngine::Vector3,
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction+Parameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::LimitAvatarPoseRestriction_Parameters =>
    "BeatSaber.AvatarCore"."LimitAvatarPoseRestriction/Parameters"
);
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction+Parameters")]
impl std::ops::Deref
for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction_Parameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction+Parameters")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction_Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction+Parameters")]
impl crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction_Parameters {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+LimitAvatarPoseRestriction+Parameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::LimitAvatarPoseRestriction_Parameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
