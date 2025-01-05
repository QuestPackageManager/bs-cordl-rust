#[cfg(feature = "FileSystemHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "FileSystemHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileSystemHelper => ""
    ."FileSystemHelper"
);
#[cfg(feature = "FileSystemHelper")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileSystemHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemHelper")]
impl crate::GlobalNamespace::FileSystemHelper {
    pub fn FindFirstExistedParentPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindFirstExistedParentPath", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasWritePermission(
        accessControlList: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::FileSystemSecurity,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasWritePermission", (accessControlList))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasWritePermissionOnDirectory(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasWritePermissionOnDirectory", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasWritePermissionOnFile(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasWritePermissionOnFile", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFileWritable(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsFileWritable", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileSystemHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FileSystemHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
