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
impl crate::BeatSaber::BeatAvatarSDK::AvatarRandomizer {}
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
