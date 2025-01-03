#[cfg(feature = "Interop")]
#[repr(C)]
#[derive(Debug)]
pub struct Interop {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Interop")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Interop => ""."Interop"
);
#[cfg(feature = "Interop")]
impl std::ops::Deref for crate::GlobalNamespace::Interop {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Interop")]
impl std::ops::DerefMut for crate::GlobalNamespace::Interop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Interop")]
impl crate::GlobalNamespace::Interop {
    #[cfg(feature = "Interop+Error")]
    pub type Error = crate::GlobalNamespace::Interop_Error;
    #[cfg(feature = "Interop+ErrorInfo")]
    pub type ErrorInfo = crate::GlobalNamespace::Interop_ErrorInfo;
    #[cfg(feature = "Interop+Sys")]
    pub type Sys = crate::GlobalNamespace::Interop_Sys;
    pub fn CheckIo_i32_1(
        result: i32,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isDirectory: bool,
        errorRewriter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::GlobalNamespace::Interop_ErrorInfo,
                crate::GlobalNamespace::Interop_ErrorInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIo", (result, path, isDirectory, errorRewriter))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIo_i64_0(
        result: i64,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isDirectory: bool,
        errorRewriter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::GlobalNamespace::Interop_ErrorInfo,
                crate::GlobalNamespace::Interop_ErrorInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIo", (result, path, isDirectory, errorRewriter))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExceptionForIoErrno(
        errorInfo: crate::GlobalNamespace::Interop_ErrorInfo,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isDirectory: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetExceptionForIoErrno", (errorInfo, path, isDirectory))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIOException(
        errorInfo: crate::GlobalNamespace::Interop_ErrorInfo,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIOException", (errorInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRandomBytes(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRandomBytes", (buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowExceptionForIoErrno(
        errorInfo: crate::GlobalNamespace::Interop_ErrorInfo,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isDirectory: bool,
        errorRewriter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                crate::GlobalNamespace::Interop_ErrorInfo,
                crate::GlobalNamespace::Interop_ErrorInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ThrowExceptionForIoErrno",
                (errorInfo, path, isDirectory, errorRewriter),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Interop")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Interop {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Interop+Error")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interop_Error {
    _cordl_E2BIG = 65537i32,
    _cordl_EACCES = 65538i32,
    _cordl_EADDRINUSE = 65539i32,
    _cordl_EADDRNOTAVAIL = 65540i32,
    _cordl_EAFNOSUPPORT = 65541i32,
    _cordl_EAGAIN = 65542i32,
    _cordl_EALREADY = 65543i32,
    _cordl_EBADF = 65544i32,
    _cordl_EBADMSG = 65545i32,
    _cordl_EBUSY = 65546i32,
    _cordl_ECANCELED = 65547i32,
    _cordl_ECHILD = 65548i32,
    _cordl_ECONNABORTED = 65549i32,
    _cordl_ECONNREFUSED = 65550i32,
    _cordl_ECONNRESET = 65551i32,
    _cordl_EDEADLK = 65552i32,
    _cordl_EDESTADDRREQ = 65553i32,
    _cordl_EDOM = 65554i32,
    _cordl_EDQUOT = 65555i32,
    _cordl_EEXIST = 65556i32,
    _cordl_EFAULT = 65557i32,
    _cordl_EFBIG = 65558i32,
    _cordl_EHOSTDOWN = 65648i32,
    _cordl_EHOSTUNREACH = 65559i32,
    _cordl_EIDRM = 65560i32,
    _cordl_EILSEQ = 65561i32,
    _cordl_EINPROGRESS = 65562i32,
    _cordl_EINTR = 65563i32,
    _cordl_EINVAL = 65564i32,
    _cordl_EIO = 65565i32,
    _cordl_EISCONN = 65566i32,
    _cordl_EISDIR = 65567i32,
    _cordl_ELOOP = 65568i32,
    _cordl_EMFILE = 65569i32,
    _cordl_EMLINK = 65570i32,
    _cordl_EMSGSIZE = 65571i32,
    _cordl_EMULTIHOP = 65572i32,
    _cordl_ENAMETOOLONG = 65573i32,
    _cordl_ENETDOWN = 65574i32,
    _cordl_ENETRESET = 65575i32,
    _cordl_ENETUNREACH = 65576i32,
    _cordl_ENFILE = 65577i32,
    _cordl_ENOBUFS = 65578i32,
    _cordl_ENODATA = 65649i32,
    _cordl_ENODEV = 65580i32,
    _cordl_ENOENT = 65581i32,
    _cordl_ENOEXEC = 65582i32,
    _cordl_ENOLCK = 65583i32,
    _cordl_ENOLINK = 65584i32,
    _cordl_ENOMEM = 65585i32,
    _cordl_ENOMSG = 65586i32,
    _cordl_ENOPROTOOPT = 65587i32,
    _cordl_ENOSPC = 65588i32,
    _cordl_ENOSYS = 65591i32,
    _cordl_ENOTCONN = 65592i32,
    _cordl_ENOTDIR = 65593i32,
    _cordl_ENOTEMPTY = 65594i32,
    _cordl_ENOTSOCK = 65596i32,
    _cordl_ENOTSUP = 65597i32,
    _cordl_ENOTTY = 65598i32,
    _cordl_ENXIO = 65599i32,
    _cordl_EOVERFLOW = 65600i32,
    _cordl_EPERM = 65602i32,
    _cordl_EPFNOSUPPORT = 65632i32,
    _cordl_EPIPE = 65603i32,
    _cordl_EPROTO = 65604i32,
    _cordl_EPROTONOSUPPORT = 65605i32,
    _cordl_EPROTOTYPE = 65606i32,
    _cordl_ERANGE = 65607i32,
    _cordl_EROFS = 65608i32,
    _cordl_ESHUTDOWN = 65644i32,
    _cordl_ESOCKTNOSUPPORT = 65630i32,
    _cordl_ESPIPE = 65609i32,
    _cordl_ESRCH = 65610i32,
    _cordl_ESTALE = 65611i32,
    _cordl_ETIMEDOUT = 65613i32,
    _cordl_ETXTBSY = 65614i32,
    _cordl_EXDEV = 65615i32,
    SUCCESS = 0i32,
}
#[cfg(feature = "Interop+Error")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Interop_Error => ""
    ."Interop/Error"
);
#[cfg(feature = "Interop+ErrorInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Interop_ErrorInfo {
    pub _error: crate::GlobalNamespace::Interop_Error,
    pub _rawErrno: i32,
}
#[cfg(feature = "Interop+ErrorInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Interop_ErrorInfo => ""
    ."Interop/ErrorInfo"
);
#[cfg(feature = "Interop+ErrorInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Interop_ErrorInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Interop+ErrorInfo")]
impl crate::GlobalNamespace::Interop_ErrorInfo {
    pub fn GetErrorMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetErrorMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Interop_Error1(
        &mut self,
        error: crate::GlobalNamespace::Interop_Error,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (error),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        _cordl_errno: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_errno),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Interop_Error> {
        let __cordl_ret: crate::GlobalNamespace::Interop_Error = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Error",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RawErrno(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_RawErrno",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Interop+Sys")]
#[repr(C)]
#[derive(Debug)]
pub struct Interop_Sys {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Interop+Sys")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Interop_Sys => ""."Interop/Sys"
);
#[cfg(feature = "Interop+Sys")]
impl std::ops::Deref for crate::GlobalNamespace::Interop_Sys {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Interop+Sys")]
impl std::ops::DerefMut for crate::GlobalNamespace::Interop_Sys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Interop+Sys")]
impl crate::GlobalNamespace::Interop_Sys {
    #[cfg(feature = "Interop+Sys+DirectoryEntry")]
    pub type DirectoryEntry = crate::GlobalNamespace::Sys_Interop_DirectoryEntry;
    #[cfg(feature = "Interop+Sys+FileStatus")]
    pub type FileStatus = crate::GlobalNamespace::Sys_Interop_FileStatus;
    #[cfg(feature = "Interop+Sys+FileStatusFlags")]
    pub type FileStatusFlags = crate::GlobalNamespace::Sys_Interop_FileStatusFlags;
    #[cfg(feature = "Interop+Sys+NodeType")]
    pub type NodeType = crate::GlobalNamespace::Sys_Interop_NodeType;
    #[cfg(feature = "Interop+Sys+Permissions")]
    pub type Permissions = crate::GlobalNamespace::Sys_Interop_Permissions;
    pub fn CloseDir(dir: crate::System::IntPtr) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloseDir", (dir))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertErrorPalToPlatform(
        error: crate::GlobalNamespace::Interop_Error,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertErrorPalToPlatform", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertErrorPlatformToPal(
        platformErrno: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::Interop_Error> {
        let __cordl_ret: crate::GlobalNamespace::Interop_Error = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertErrorPlatformToPal", (platformErrno))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFile(
        source: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        >,
        destination: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyFile", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoubleToString(
        value: f64,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLength: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DoubleToString", (value, format, buffer, bufferLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEGid() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEGid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEUid() -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEUid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastErrorInfo() -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::Interop_ErrorInfo,
    > {
        let __cordl_ret: crate::GlobalNamespace::Interop_ErrorInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLastErrorInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNonCryptographicallySecureRandomBytes(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNonCryptographicallySecureRandomBytes", (buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReadDirRBufferSize() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReadDirRBufferSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LChflagsCanSetHiddenFlag() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LChflagsCanSetHiddenFlag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LStat_ByRefMut1(
        path: quest_hook::libil2cpp::ByRefMut<u8>,
        output: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Sys_Interop_FileStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LStat", (path, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn LStat_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        output: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Sys_Interop_FileStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LStat", (path, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn LStat_ReadOnlySpan_1_2(
        path: crate::System::ReadOnlySpan_1<char>,
        output: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Sys_Interop_FileStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LStat", (path, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn Link(
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        link: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Link", (source, link))?;
        Ok(__cordl_ret.into())
    }
    pub fn MkDir(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MkDir", (path, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenDir(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OpenDir", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadDirR(
        dir: crate::System::IntPtr,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
        outputEntry: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Sys_Interop_DirectoryEntry,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadDirR", (dir, buffer, bufferSize, outputEntry))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadLink_Il2CppArray_i32_0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadLink", (path, buffer, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadLink_Il2CppString1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ReadLink", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rename(
        oldPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rename", (oldPath, newPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn RmDir(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RmDir", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stat_ByRefMut1(
        path: quest_hook::libil2cpp::ByRefMut<u8>,
        output: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Sys_Interop_FileStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Stat", (path, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stat_Il2CppString0(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        output: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Sys_Interop_FileStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Stat", (path, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stat_ReadOnlySpan_1_2(
        path: crate::System::ReadOnlySpan_1<char>,
        output: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::Sys_Interop_FileStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Stat", (path, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn StrError(
        platformErrno: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StrError", (platformErrno))?;
        Ok(__cordl_ret.into())
    }
    pub fn StrErrorR(
        platformErrno: i32,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StrErrorR", (platformErrno, buffer, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn Symlink(
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        linkPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Symlink", (target, linkPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unlink(
        pathname: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unlink", (pathname))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Interop+Sys")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Interop_Sys {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Interop+Sys+DirectoryEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Sys_Interop_DirectoryEntry {
    pub Name: *mut quest_hook::libil2cpp::Il2CppObject,
    pub NameLength: i32,
    pub InodeType: crate::GlobalNamespace::Sys_Interop_NodeType,
}
#[cfg(feature = "Interop+Sys+DirectoryEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Sys_Interop_DirectoryEntry =>
    ""."Interop/Sys/DirectoryEntry"
);
#[cfg(feature = "Interop+Sys+DirectoryEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Sys_Interop_DirectoryEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Interop+Sys+DirectoryEntry")]
impl crate::GlobalNamespace::Sys_Interop_DirectoryEntry {
    pub fn GetName(
        &mut self,
        buffer: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<char>> {
        let __cordl_ret: crate::System::ReadOnlySpan_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetName",
            (buffer),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Interop+Sys+FileStatus")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Sys_Interop_FileStatus {
    pub Flags: crate::GlobalNamespace::Sys_Interop_FileStatusFlags,
    pub Mode: i32,
    pub Uid: u32,
    pub Gid: u32,
    pub Size: i64,
    pub ATime: i64,
    pub ATimeNsec: i64,
    pub MTime: i64,
    pub MTimeNsec: i64,
    pub CTime: i64,
    pub CTimeNsec: i64,
    pub BirthTime: i64,
    pub BirthTimeNsec: i64,
    pub Dev: i64,
    pub Ino: i64,
    pub UserFlags: u32,
}
#[cfg(feature = "Interop+Sys+FileStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Sys_Interop_FileStatus => ""
    ."Interop/Sys/FileStatus"
);
#[cfg(feature = "Interop+Sys+FileStatus")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::Sys_Interop_FileStatus {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Interop+Sys+FileStatus")]
impl crate::GlobalNamespace::Sys_Interop_FileStatus {}
#[cfg(feature = "Interop+Sys+FileStatusFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys_Interop_FileStatusFlags {
    HasBirthTime = 1i32,
    None = 0i32,
}
#[cfg(feature = "Interop+Sys+FileStatusFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Sys_Interop_FileStatusFlags =>
    ""."Interop/Sys/FileStatusFlags"
);
#[cfg(feature = "Interop+Sys+NodeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys_Interop_NodeType {
    DT_BLK = 6i32,
    DT_CHR = 2i32,
    DT_DIR = 4i32,
    DT_FIFO = 1i32,
    DT_LNK = 10i32,
    DT_REG = 8i32,
    DT_SOCK = 12i32,
    DT_UNKNOWN = 0i32,
    DT_WHT = 14i32,
}
#[cfg(feature = "Interop+Sys+NodeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Sys_Interop_NodeType => ""
    ."Interop/Sys/NodeType"
);
#[cfg(feature = "Interop+Sys+Permissions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sys_Interop_Permissions {
    Mask = 511i32,
    S_IRGRP = 32i32,
    S_IROTH = 4i32,
    S_IRUSR = 256i32,
    S_IRWXG = 56i32,
    S_IRWXO = 7i32,
    S_IRWXU = 448i32,
    S_IWGRP = 16i32,
    S_IWOTH = 2i32,
    S_IWUSR = 128i32,
    S_IXGRP = 8i32,
    S_IXOTH = 1i32,
    S_IXUSR = 64i32,
}
#[cfg(feature = "Interop+Sys+Permissions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Sys_Interop_Permissions => ""
    ."Interop/Sys/Permissions"
);
