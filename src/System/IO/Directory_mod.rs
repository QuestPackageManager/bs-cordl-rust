#[cfg(feature = "System+IO+Directory")]
#[repr(C)]
#[derive(Debug)]
pub struct Directory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+Directory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Directory => "System.IO"."Directory"
);
#[cfg(feature = "System+IO+Directory")]
impl std::ops::Deref for crate::System::IO::Directory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Directory")]
impl std::ops::DerefMut for crate::System::IO::Directory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Directory")]
impl crate::System::IO::Directory {
    pub fn CreateDirectory(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateDirectory", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumerateDirectories_EnumerationOptions1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enumerationOptions: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnumerateDirectories", (path, searchPattern, enumerationOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumerateDirectories_SearchOption0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchOption: crate::System::IO::SearchOption,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnumerateDirectories", (path, searchPattern, searchOption))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumerateFileSystemEntries_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnumerateFileSystemEntries", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnumerateFileSystemEntries_Il2CppString_EnumerationOptions1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enumerationOptions: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnumerateFileSystemEntries",
                (path, searchPattern, enumerationOptions),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Exists(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Exists", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAccessControl_AccessControlSections0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        includeSections: crate::System::Security::AccessControl::AccessControlSections,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DirectorySecurity,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DirectorySecurity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAccessControl", (path, includeSections))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAccessControl_Il2CppString1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DirectorySecurity,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::AccessControl::DirectorySecurity,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAccessControl", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentDirectory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentDirectory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFiles_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetFiles", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFiles_Il2CppString1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFiles", (path, searchPattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFiles_Il2CppString_EnumerationOptions3(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        enumerationOptions: quest_hook::libil2cpp::Gc<
            crate::System::IO::EnumerationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFiles", (path, searchPattern, enumerationOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFiles_Il2CppString_SearchOption2(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchOption: crate::System::IO::SearchOption,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFiles", (path, searchPattern, searchOption))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn GetParent(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::DirectoryInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParent", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsecureGetCurrentDirectory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsecureGetCurrentDirectory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalEnumeratePaths(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchPattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchTarget: crate::System::IO::SearchTarget,
        options: quest_hook::libil2cpp::Gc<crate::System::IO::EnumerationOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InternalEnumeratePaths",
                (path, searchPattern, searchTarget, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetDirectoryRoot(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalGetDirectoryRoot", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Directory")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::Directory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
