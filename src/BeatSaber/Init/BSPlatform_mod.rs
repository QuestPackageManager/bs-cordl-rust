#[cfg(feature = "BeatSaber+Init+BSPlatform")]
#[repr(C)]
#[derive(Debug)]
pub struct BSPlatform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatSaber+Init+BSPlatform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Init::BSPlatform => "BeatSaber.Init"
    ."BSPlatform"
);
#[cfg(feature = "BeatSaber+Init+BSPlatform")]
impl std::ops::Deref for crate::BeatSaber::Init::BSPlatform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSPlatform")]
impl std::ops::DerefMut for crate::BeatSaber::Init::BSPlatform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Init+BSPlatform")]
impl crate::BeatSaber::Init::BSPlatform {}
#[cfg(feature = "BeatSaber+Init+BSPlatform")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Init::BSPlatform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
