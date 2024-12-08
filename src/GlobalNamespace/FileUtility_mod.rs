#[cfg(feature = "FileUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct FileUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "FileUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileUtility => ""."FileUtility"
);
#[cfg(feature = "FileUtility")]
impl std::ops::Deref for crate::GlobalNamespace::FileUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileUtility")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileUtility")]
impl crate::GlobalNamespace::FileUtility {}
#[cfg(feature = "FileUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FileUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
