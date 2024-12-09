#[cfg(feature = "FileHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct FileHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "FileHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileHelpers => ""."FileHelpers"
);
#[cfg(feature = "FileHelpers")]
impl std::ops::Deref for crate::GlobalNamespace::FileHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileHelpers")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileHelpers")]
impl crate::GlobalNamespace::FileHelpers {
    pub const kProtocolInfix: &'static str = "://";
}
#[cfg(feature = "FileHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FileHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
