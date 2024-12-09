#[cfg(feature = "Oculus+Platform+AssetFile")]
#[repr(C)]
#[derive(Debug)]
pub struct AssetFile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::AssetFile => "Oculus.Platform"
    ."AssetFile"
);
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl std::ops::Deref for crate::Oculus::Platform::AssetFile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl std::ops::DerefMut for crate::Oculus::Platform::AssetFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl crate::Oculus::Platform::AssetFile {}
#[cfg(feature = "Oculus+Platform+AssetFile")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::AssetFile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
