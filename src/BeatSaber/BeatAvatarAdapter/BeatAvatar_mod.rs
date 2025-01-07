#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatar")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatar {
    __cordl_parent: crate::BeatSaber::AvatarCore::Avatar,
    pub _avatarVisualController: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController,
    >,
    pub _avatarPoseController: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::BeatAvatarSDK::BeatAvatarPoseController,
    >,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatar")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatar {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarAdapter";
    const CLASS_NAME: &'static str = "BeatAvatar";
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
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatar")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarAdapter::BeatAvatar {
    type Target = crate::BeatSaber::AvatarCore::Avatar;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatar")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarAdapter::BeatAvatar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatar")]
impl crate::BeatSaber::BeatAvatarAdapter::BeatAvatar {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetLightColor(
        &mut self,
        lightColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLightColor", (lightColor))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarFromOptionalData(
        &mut self,
        data: crate::BeatSaber::AvatarCore::OptionalAvatarData,
        playbackDelaySeconds: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarFromOptionalData", (data, playbackDelaySeconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarFromPose(
        &mut self,
        currentPose: crate::BeatSaber::AvatarCore::AvatarPoseData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarFromPose", (currentPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarFromVisualData(
        &mut self,
        visualData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarFromVisualData", (visualData))?;
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
    pub fn get_bodyCenterWorldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_bodyCenterWorldPosition", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatar")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
