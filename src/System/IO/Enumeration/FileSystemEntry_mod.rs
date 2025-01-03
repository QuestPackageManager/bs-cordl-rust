#[cfg(feature = "System+IO+Enumeration+FileSystemEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    pub _directoryEntry: crate::GlobalNamespace::Sys_Interop_DirectoryEntry,
    pub _status: crate::System::IO::FileStatus,
    pub _pathBuffer: crate::System::Span_1<char>,
    pub _fullPath: crate::System::ReadOnlySpan_1<char>,
    pub _fileName: crate::System::ReadOnlySpan_1<char>,
    pub _fileNameBuffer: crate::System::IO::Enumeration::FileSystemEntry___fileNameBuffer_e__FixedBuffer,
    pub _initialAttributes: crate::System::IO::FileAttributes,
    pub _Directory_k__BackingField: crate::System::ReadOnlySpan_1<char>,
    pub _RootDirectory_k__BackingField: crate::System::ReadOnlySpan_1<char>,
    pub _OriginalRootDirectory_k__BackingField: crate::System::ReadOnlySpan_1<char>,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Enumeration::FileSystemEntry =>
    "System.IO.Enumeration"."FileSystemEntry"
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::IO::Enumeration::FileSystemEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEntry")]
impl crate::System::IO::Enumeration::FileSystemEntry {
    #[cfg(
        feature = "System+IO+Enumeration+FileSystemEntry+__fileNameBuffer_e__FixedBuffer"
    )]
    pub type __fileNameBuffer_e__FixedBuffer = crate::System::IO::Enumeration::FileSystemEntry___fileNameBuffer_e__FixedBuffer;
    pub fn Initialize(
        entry: quest_hook::libil2cpp::ByRefMut<
            crate::System::IO::Enumeration::FileSystemEntry,
        >,
        directoryEntry: crate::GlobalNamespace::Sys_Interop_DirectoryEntry,
        directory: crate::System::ReadOnlySpan_1<char>,
        rootDirectory: crate::System::ReadOnlySpan_1<char>,
        originalRootDirectory: crate::System::ReadOnlySpan_1<char>,
        pathBuffer: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::FileAttributes> {
        let __cordl_ret: crate::System::IO::FileAttributes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Initialize",
                (
                    entry,
                    directoryEntry,
                    directory,
                    rootDirectory,
                    originalRootDirectory,
                    pathBuffer,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFileSystemInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::FileSystemInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::FileSystemInfo> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToFileSystemInfo",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToFullPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToFullPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSpecifiedFullPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToSpecifiedFullPath",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::FileAttributes> {
        let __cordl_ret: crate::System::IO::FileAttributes = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Attributes",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Directory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Directory",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FileName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_FileName",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FullPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_FullPath",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDirectory(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsDirectory",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OriginalRootDirectory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_OriginalRootDirectory",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RootDirectory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RootDirectory",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Directory(
        &mut self,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Directory",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_OriginalRootDirectory(
        &mut self,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_OriginalRootDirectory",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RootDirectory(
        &mut self,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_RootDirectory",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEntry+__fileNameBuffer_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FileSystemEntry___fileNameBuffer_e__FixedBuffer {
    pub FixedElementField: char,
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEntry+__fileNameBuffer_e__FixedBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Enumeration::FileSystemEntry___fileNameBuffer_e__FixedBuffer =>
    "System.IO.Enumeration"."FileSystemEntry/<_fileNameBuffer>e__FixedBuffer"
);
#[cfg(feature = "System+IO+Enumeration+FileSystemEntry+__fileNameBuffer_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::IO::Enumeration::FileSystemEntry___fileNameBuffer_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+IO+Enumeration+FileSystemEntry+__fileNameBuffer_e__FixedBuffer")]
impl crate::System::IO::Enumeration::FileSystemEntry___fileNameBuffer_e__FixedBuffer {}
