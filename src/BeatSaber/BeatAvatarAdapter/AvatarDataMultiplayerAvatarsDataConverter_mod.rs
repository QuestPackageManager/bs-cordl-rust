#[cfg(feature = "BeatSaber+BeatAvatarAdapter+AvatarDataMultiplayerAvatarsDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarDataMultiplayerAvatarsDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::BeatSaber::BeatAvatarAdapter::AvatarDataMultiplayerAvatarsDataConverter {}
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
