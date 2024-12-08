#[cfg(feature = "FileHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct FileHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "FileHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FileHelpers => ""."FileHelpers"
);
#[cfg(feature = "FileHelpers")]
impl std::ops::Deref for FileHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileHelpers")]
impl std::ops::DerefMut for FileHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileHelpers")]
impl FileHelpers {
    pub const kProtocolInfix: &'static str = "://";
}
#[cfg(feature = "FileHelpers")]
impl quest_hook::libil2cpp::ObjectType for FileHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
