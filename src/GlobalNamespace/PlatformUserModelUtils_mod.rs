#[cfg(feature = "PlatformUserModelUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformUserModelUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlatformUserModelUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlatformUserModelUtils => ""
    ."PlatformUserModelUtils"
);
#[cfg(feature = "PlatformUserModelUtils")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformUserModelUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserModelUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformUserModelUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformUserModelUtils")]
impl crate::GlobalNamespace::PlatformUserModelUtils {
    pub const kMinimalTokenLength: i32 = 64i32;
}
#[cfg(feature = "PlatformUserModelUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformUserModelUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
