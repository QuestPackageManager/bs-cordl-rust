#[cfg(feature = "FileSystemHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "FileSystemHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FileSystemHelper => ""."FileSystemHelper"
);
#[cfg(feature = "FileSystemHelper")]
impl std::ops::Deref for FileSystemHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemHelper")]
impl std::ops::DerefMut for FileSystemHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemHelper")]
impl FileSystemHelper {}
#[cfg(feature = "FileSystemHelper")]
impl quest_hook::libil2cpp::ObjectType for FileSystemHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
