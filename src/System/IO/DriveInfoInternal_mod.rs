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
impl crate::System::IO::DriveInfoInternal {
    pub fn GetLogicalDrives() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLogicalDrives", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+DriveInfoInternal")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::DriveInfoInternal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
