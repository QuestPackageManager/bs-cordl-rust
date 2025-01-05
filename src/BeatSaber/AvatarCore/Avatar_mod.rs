#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
#[repr(C)]
#[derive(Debug)]
pub struct Avatar {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub visualDataProvider: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider,
    >,
    pub poseDataProvider: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider,
    >,
    pub optionalDataProvider: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::Avatar =>
    "BeatSaber.AvatarCore"."Avatar"
);
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::Avatar {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::Avatar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
impl crate::BeatSaber::AvatarCore::Avatar {
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
    pub fn SetOptionalDataProvider(
        &mut self,
        optionalDataProvider: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOptionalDataProvider", (optionalDataProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPoseDataProvider(
        &mut self,
        poseDataProvider: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPoseDataProvider", (poseDataProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVisualDataProvider(
        &mut self,
        visualDataProvider: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVisualDataProvider", (visualDataProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarFromOptionalDataList(
        &mut self,
        optionalData: quest_hook::libil2cpp::Gc<
            u32,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarFromOptionalDataList", (optionalData))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarFromOptionalData_OptionalAvatarData0(
        &mut self,
        data: crate::BeatSaber::AvatarCore::OptionalAvatarData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarFromOptionalData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarFromOptionalData_f32_1(
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
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::AvatarCore::Avatar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
