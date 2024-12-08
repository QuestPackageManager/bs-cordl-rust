#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
#[repr(C)]
#[derive(Debug)]
pub struct Avatar {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub visualDataProvider: *mut crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider,
    pub poseDataProvider: *mut crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider,
    pub optionalDataProvider: *mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
}
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::Avatar =>
    "BeatSaber.AvatarCore"."Avatar"
);
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::Avatar {
    type Target = crate::UnityEngine::MonoBehaviour;
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
    pub fn UpdateAvatarFromVisualData(
        &mut self,
        visualData: MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarFromVisualData", (visualData))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetOptionalDataProvider(
        &mut self,
        optionalDataProvider: *mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOptionalDataProvider", (optionalDataProvider))?;
        Ok(__cordl_ret)
    }
    pub fn get_bodyCenterWorldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_bodyCenterWorldPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAvatarFromOptionalDataList(
        &mut self,
        optionalData: *mut crate::System::Collections::Generic::Dictionary_2<
            u32,
            crate::BeatSaber::AvatarCore::OptionalAvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarFromOptionalDataList", (optionalData))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetPoseDataProvider(
        &mut self,
        poseDataProvider: *mut crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPoseDataProvider", (poseDataProvider))?;
        Ok(__cordl_ret)
    }
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
    pub fn SetVisualDataProvider(
        &mut self,
        visualDataProvider: *mut crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVisualDataProvider", (visualDataProvider))?;
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
#[cfg(feature = "BeatSaber+AvatarCore+Avatar")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::AvatarCore::Avatar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
