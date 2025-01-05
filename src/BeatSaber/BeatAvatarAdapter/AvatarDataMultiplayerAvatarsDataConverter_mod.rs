#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarDataMultiplayerAvatarsDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter =>
    "BeatSaber.BeatAvatarAdapter"."AvatarDataMultiplayerAvatarsDataConverter"
);
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
impl crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter {
    pub fn CreateAvatarData(
        multiplayerAvatarsData: crate::GlobalNamespace::MultiplayerAvatarData,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::BeatAvatarSDK::AvatarData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateAvatarData", (multiplayerAvatarsData))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMultiplayerAvatarsData(
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarData> {
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarData = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMultiplayerAvatarsData", (avatarData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
