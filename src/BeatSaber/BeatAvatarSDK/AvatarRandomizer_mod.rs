#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarRandomizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::AvatarRandomizer =>
    "BeatSaber.BeatAvatarSDK"."AvatarRandomizer"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    pub fn RandomizeAll(
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
        avatarPartsModel: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RandomizeAll", (avatarData, avatarPartsModel))?;
        Ok(__cordl_ret.into())
    }
    pub fn RandomizeColors(
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RandomizeColors", (avatarData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RandomizeModels(
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
        avatarPartsModel: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RandomizeModels", (avatarData, avatarPartsModel))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarRandomizer")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
