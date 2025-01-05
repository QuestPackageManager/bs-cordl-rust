#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct GameBuildInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::Generated::GameBuildInfo =>
    "BeatSaber.Init.Generated"."GameBuildInfo"
);
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl std::ops::Deref for crate::BeatSaber::Init::Generated::GameBuildInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl std::ops::DerefMut for crate::BeatSaber::Init::Generated::GameBuildInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl crate::BeatSaber::Init::Generated::GameBuildInfo {
    pub const kDefaultPlatformVersion: &'static str = "1390";
    pub const kPreReleaseLabel: &'static str = "";
}
#[cfg(feature = "BeatSaber+Init+Generated+GameBuildInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Init::Generated::GameBuildInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
