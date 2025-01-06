#[cfg(feature = "System+IO+FileSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+FileSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileSystem => "System.IO"
    ."FileSystem"
);
#[cfg(feature = "System+IO+FileSystem")]
impl std::ops::Deref for crate::System::IO::FileSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::System::IO::FileSystem {
    pub fn CopyDanglingSymlink(
        sourceFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyDanglingSymlink", (sourceFullPath, destFullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFile(
        sourceFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        overwrite: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyFile", (sourceFullPath, destFullPath, overwrite))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDirectory(
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDirectory", (fullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeleteFile(
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeleteFile", (fullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn DirectoryExists_ByRefMut1(
        fullPath: crate::System::ReadOnlySpan_1<char>,
        errorInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Interop_ErrorInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DirectoryExists", (fullPath, errorInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn DirectoryExists_ReadOnlySpan_1_0(
        fullPath: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DirectoryExists", (fullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FileExists_ReadOnlySpan_1_0(
        fullPath: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FileExists", (fullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FileExists_i32_ByRefMut1(
        fullPath: crate::System::ReadOnlySpan_1<char>,
        fileType: i32,
        errorInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Interop_ErrorInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FileExists", (fullPath, fileType, errorInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes(
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::FileAttributes> {
        let __cordl_ret: crate::System::IO::FileAttributes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (fullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLogicalDrives() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLogicalDrives", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LinkOrCopyFile(
        sourceFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LinkOrCopyFile", (sourceFullPath, destFullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveFile(
        sourceFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveFile", (sourceFullPath, destFullPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDirectory(
        fullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        recursive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveDirectory", (fullPath, recursive))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDirectoryInternal(
        directory: quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo>,
        recursive: bool,
        throwOnTopLevelDirectoryNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RemoveDirectoryInternal",
                (directory, recursive, throwOnTopLevelDirectoryNotFound),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReplaceFile(
        sourceFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destFullPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destBackupFullPath: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        ignoreMetadataErrors: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ReplaceFile",
                (sourceFullPath, destFullPath, destBackupFullPath, ignoreMetadataErrors),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldIgnoreDirectory(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldIgnoreDirectory", (name))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+FileSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::FileSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
