#[cfg(feature = "System+IO+FileStatus")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FileStatus {
    pub _fileStatus: crate::GlobalNamespace::Sys_Interop_FileStatus,
    pub _fileStatusInitialized: i32,
    pub _InitiallyDirectory_k__BackingField: bool,
    pub _isDirectory: bool,
    pub _exists: bool,
}
#[cfg(feature = "System+IO+FileStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileStatus => "System.IO"
    ."FileStatus"
);
#[cfg(feature = "System+IO+FileStatus")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::IO::FileStatus {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+IO+FileStatus")]
impl crate::System::IO::FileStatus {
    pub fn EnsureStatInitialized(
        &mut self,
        path: crate::System::ReadOnlySpan_1<char>,
        continueOnError: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EnsureStatInitialized",
            (path, continueOnError),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes(
        &mut self,
        path: crate::System::ReadOnlySpan_1<char>,
        fileName: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::FileAttributes> {
        let __cordl_ret: crate::System::IO::FileAttributes = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetAttributes",
            (path, fileName),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExists(
        &mut self,
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetExists",
            (path),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastWriteTime(
        &mut self,
        path: crate::System::ReadOnlySpan_1<char>,
        continueOnError: bool,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLastWriteTime",
            (path, continueOnError),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLength(
        &mut self,
        path: crate::System::ReadOnlySpan_1<char>,
        continueOnError: bool,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLength",
            (path, continueOnError),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        status: quest_hook::libil2cpp::ByRefMut<crate::System::IO::FileStatus>,
        isDirectory: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", (status, isDirectory))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsReadOnly(
        &mut self,
        path: crate::System::ReadOnlySpan_1<char>,
        continueOnError: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsReadOnly",
            (path, continueOnError),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Refresh(
        &mut self,
        path: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Refresh",
            (path),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnixTimeToDateTimeOffset(
        &mut self,
        seconds: i64,
        nanoseconds: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UnixTimeToDateTimeOffset",
            (seconds, nanoseconds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InitiallyDirectory(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InitiallyDirectory",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InitiallyDirectory(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_InitiallyDirectory",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
