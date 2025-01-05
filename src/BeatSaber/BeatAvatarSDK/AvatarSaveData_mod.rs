#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub headTopId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub glassesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub facialHairId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub handsId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub clothesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub skinColorId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub mouthId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub eyesId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub headTopPrimaryColor: crate::UnityEngine::Color,
    pub headTopSecondaryColor: crate::UnityEngine::Color,
    pub glassesColor: crate::UnityEngine::Color,
    pub facialHairColor: crate::UnityEngine::Color,
    pub handsColor: crate::UnityEngine::Color,
    pub clothesPrimaryColor: crate::UnityEngine::Color,
    pub clothesSecondaryColor: crate::UnityEngine::Color,
    pub clothesDetailColor: crate::UnityEngine::Color,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::AvatarSaveData =>
    "BeatSaber.BeatAvatarSDK"."AvatarSaveData"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
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
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
