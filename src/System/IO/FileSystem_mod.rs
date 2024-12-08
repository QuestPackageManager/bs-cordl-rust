#[cfg(feature = "System+IO+FileSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystem {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+IO+FileSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileSystem => "System.IO"
    ."FileSystem"
);
#[cfg(feature = "System+IO+FileSystem")]
impl std::ops::Deref for crate::System::IO::FileSystem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileSystem")]
impl std::ops::DerefMut for crate::System::IO::FileSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileSystem")]
impl crate::System::IO::FileSystem {}
#[cfg(feature = "System+IO+FileSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::FileSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}