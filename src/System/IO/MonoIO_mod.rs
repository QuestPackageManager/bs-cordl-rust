#[cfg(feature = "System+IO+MonoIO")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoIO {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IO+MonoIO")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::MonoIO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "MonoIO";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+IO+MonoIO")]
impl std::ops::Deref for crate::System::IO::MonoIO {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+MonoIO")]
impl std::ops::DerefMut for crate::System::IO::MonoIO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+MonoIO")]
impl crate::System::IO::MonoIO {
    pub fn Cancel(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cancel", (safeHandle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cancel_internal(
        handle: crate::System::IntPtr,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cancel_internal", (handle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        handle: crate::System::IntPtr,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Close", (handle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePipe(
        read_handle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        write_handle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePipe", (read_handle, write_handle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn DumpHandles() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DumpHandles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DuplicateHandle(
        source_process_handle: crate::System::IntPtr,
        source_handle: crate::System::IntPtr,
        target_process_handle: crate::System::IntPtr,
        target_handle: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        access: i32,
        inherit: i32,
        options: i32,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DuplicateHandle",
                (
                    source_process_handle,
                    source_handle,
                    target_process_handle,
                    target_handle,
                    access,
                    inherit,
                    options,
                    error,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentDirectory(
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCurrentDirectory", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetException_Il2CppString_MonoIOError1(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        error: crate::System::IO::MonoIOError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetException", (path, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetException_MonoIOError0(
        error: crate::System::IO::MonoIOError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetException", (error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFileType_IntPtr0(
        handle: crate::System::IntPtr,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::MonoFileType> {
        let __cordl_ret: crate::System::IO::MonoFileType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFileType", (handle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFileType_SafeHandle1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IO::MonoFileType> {
        let __cordl_ret: crate::System::IO::MonoFileType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFileType", (safeHandle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLength_IntPtr0(
        handle: crate::System::IntPtr,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLength", (handle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLength_SafeHandle1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLength", (safeHandle, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Open_Il2CppObject0(
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        options: crate::System::IO::FileOptions,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Open", (filename, mode, access, share, options, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Open_Il2CppString1(
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        options: crate::System::IO::FileOptions,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Open", (filename, mode, access, share, options, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read_IntPtr0(
        handle: crate::System::IntPtr,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dest_offset: i32,
        count: i32,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read", (handle, dest, dest_offset, count, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Read_SafeHandle1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dest_offset: i32,
        count: i32,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Read", (safeHandle, dest, dest_offset, count, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemapPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newPath: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemapPath", (path, newPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn Seek_IntPtr0(
        handle: crate::System::IntPtr,
        offset: i64,
        origin: crate::System::IO::SeekOrigin,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Seek", (handle, offset, origin, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Seek_SafeHandle1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        offset: i64,
        origin: crate::System::IO::SeekOrigin,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Seek", (safeHandle, offset, origin, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLength_IntPtr0(
        handle: crate::System::IntPtr,
        length: i64,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLength", (handle, length, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLength_SafeHandle1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        length: i64,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLength", (safeHandle, length, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_IntPtr_ByRef0(
        handle: crate::System::IntPtr,
        src: quest_hook::libil2cpp::ByRef<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        src_offset: i32,
        count: i32,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (handle, src, src_offset, count, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write_SafeHandle_Il2CppArray1(
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        src_offset: i32,
        count: i32,
        error: quest_hook::libil2cpp::ByRefMut<crate::System::IO::MonoIOError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Write", (safeHandle, src, src_offset, count, error))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AltDirectorySeparatorChar() -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_AltDirectorySeparatorChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConsoleError() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ConsoleError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConsoleInput() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ConsoleInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConsoleOutput() -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ConsoleOutput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DirectorySeparatorChar() -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DirectorySeparatorChar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PathSeparator() -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PathSeparator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_VolumeSeparatorChar() -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_VolumeSeparatorChar", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+MonoIO")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::MonoIO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
