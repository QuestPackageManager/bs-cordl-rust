#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
#[repr(C)]
#[derive(Debug)]
pub struct SupportedPlatforms {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms =>
    "BGLib.MetaRemoteAssets.Platform"."SupportedPlatforms"
);
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl std::ops::Deref for crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl std::ops::DerefMut
for crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    pub const Android: &'static str = "ANDROID";
    pub const Playstation: &'static str = "PLAYSTATION";
    pub const Windows: &'static str = "WIN_64";
}
#[cfg(feature = "BGLib+MetaRemoteAssets+Platform+SupportedPlatforms")]
impl quest_hook::libil2cpp::ObjectType
for crate::BGLib::MetaRemoteAssets::Platform::SupportedPlatforms {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
