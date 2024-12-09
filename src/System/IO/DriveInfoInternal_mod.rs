#[cfg(feature = "System+IO+DriveInfoInternal")]
#[repr(C)]
#[derive(Debug)]
pub struct DriveInfoInternal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+DriveInfoInternal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::DriveInfoInternal => "System.IO"
    ."DriveInfoInternal"
);
#[cfg(feature = "System+IO+DriveInfoInternal")]
impl std::ops::Deref for crate::System::IO::DriveInfoInternal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+DriveInfoInternal")]
impl std::ops::DerefMut for crate::System::IO::DriveInfoInternal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+DriveInfoInternal")]
impl crate::System::IO::DriveInfoInternal {}
#[cfg(feature = "System+IO+DriveInfoInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::DriveInfoInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
