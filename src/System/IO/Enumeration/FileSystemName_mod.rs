#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemName {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Enumeration::FileSystemName =>
    "System.IO.Enumeration"."FileSystemName"
);
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl std::ops::Deref for crate::System::IO::Enumeration::FileSystemName {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl std::ops::DerefMut for crate::System::IO::Enumeration::FileSystemName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl crate::System::IO::Enumeration::FileSystemName {}
#[cfg(feature = "System+IO+Enumeration+FileSystemName")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::IO::Enumeration::FileSystemName {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
