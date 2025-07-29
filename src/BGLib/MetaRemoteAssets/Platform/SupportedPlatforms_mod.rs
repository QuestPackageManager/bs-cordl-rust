#[cfg(feature = "cordl_class_BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
#[repr(C)]
#[derive(Debug)]
pub struct SupportedPlatforms {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BGLib.MetaRemoteAssets.Platform";
    const CLASS_NAME: &'static str = "SupportedPlatforms";
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
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl std::ops::Deref for crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl std::ops::DerefMut
for crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    pub const Android: &'static str = "ANDROID";
    pub const Playstation: &'static str = "PLAYSTATION";
    pub const Windows: &'static str = "WIN_64";
}
#[cfg(feature = "cordl_class_BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
