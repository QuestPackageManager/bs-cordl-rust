#[cfg(feature = "System+IO+FileStream")]
#[repr(C)]
#[derive(Debug)]
pub struct FileStream {
    __cordl_parent: crate::System::IO::Stream,
    pub buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub safeHandle: quest_hook::libil2cpp::Gc<
        crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
    >,
    pub isExposed: bool,
    pub append_startpos: i64,
    pub access: crate::System::IO::FileAccess,
    pub owner: bool,
    pub _cordl_async: bool,
    pub canseek: bool,
    pub anonymous: bool,
    pub buf_dirty: bool,
    pub buf_size: i32,
    pub buf_length: i32,
    pub buf_offset: i32,
    pub buf_start: i64,
}
#[cfg(feature = "System+IO+FileStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileStream => "System.IO"
    ."FileStream"
);
#[cfg(feature = "System+IO+FileStream")]
impl std::ops::Deref for crate::System::IO::FileStream {
    type Target = crate::System::IO::Stream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileStream")]
impl std::ops::DerefMut for crate::System::IO::FileStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileStream")]
impl crate::System::IO::FileStream {
    #[cfg(feature = "System+IO+FileStream+ReadDelegate")]
    pub type ReadDelegate = crate::System::IO::FileStream_ReadDelegate;
    #[cfg(feature = "System+IO+FileStream+WriteDelegate")]
    pub type WriteDelegate = crate::System::IO::FileStream_WriteDelegate;
    pub fn BeginRead(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        numBytes: i32,
        userCallback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        stateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginRead", (array, offset, numBytes, userCallback, stateObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn BeginWrite(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        numBytes: i32,
        userCallback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        stateObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginWrite", (array, offset, numBytes, userCallback, stateObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndRead(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndRead", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndWrite(
        &mut self,
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndWrite", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExposeHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExposeHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("FlushAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushBufferIfDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushBufferIfDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSecureFileName_Il2CppString0(
        &mut self,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetSecureFileName", (filename))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSecureFileName__cordl_bool1(
        &mut self,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        full: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetSecureFileName", (filename, full))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        >,
        access: crate::System::IO::FileAccess,
        ownsHandle: bool,
        bufferSize: i32,
        isAsync: bool,
        isConsoleWrapper: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (safeHandle, access, ownsHandle, bufferSize, isAsync, isConsoleWrapper),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InitBuffer(
        &mut self,
        _cordl_size: i32,
        isZeroSize: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitBuffer", (_cordl_size, isZeroSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString_FileMode_FileAccess2(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, mode, access))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_FileMode_FileAccess_FileShare3(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, mode, access, share))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_FileMode_FileAccess_FileShare_i32_4(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, mode, access, share, bufferSize))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_FileMode_FileAccess_FileShare_i32_FileOptions6(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        options: crate::System::IO::FileOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, mode, access, share, bufferSize, options))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_FileMode_FileAccess_FileShare_i32__cordl_bool5(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        useAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (path, mode, access, share, bufferSize, useAsync))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_FileMode_FileAccess_FileShare_i32__cordl_bool_FileOptions10(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        anonymous: bool,
        options: crate::System::IO::FileOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (path, mode, access, share, bufferSize, anonymous, options),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_FileMode_FileAccess_FileShare_i32__cordl_bool__cordl_bool9(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        isAsync: bool,
        anonymous: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (path, mode, access, share, bufferSize, isAsync, anonymous),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_IntPtr_FileAccess__cordl_bool_i32_0(
        handle: crate::System::IntPtr,
        access: crate::System::IO::FileAccess,
        ownsHandle: bool,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handle, access, ownsHandle, bufferSize))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IntPtr_FileAccess__cordl_bool_i32__cordl_bool__cordl_bool1(
        handle: crate::System::IntPtr,
        access: crate::System::IO::FileAccess,
        ownsHandle: bool,
        bufferSize: i32,
        isAsync: bool,
        isConsoleWrapper: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (handle, access, ownsHandle, bufferSize, isAsync, isConsoleWrapper),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_SafeFileHandle_FileAccess7(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        >,
        access: crate::System::IO::FileAccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handle, access))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SafeFileHandle_FileAccess_i32__cordl_bool8(
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        >,
        access: crate::System::IO::FileAccess,
        bufferSize: i32,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handle, access, bufferSize, isAsync))?;
        Ok(__cordl_object.into())
    }
    pub fn Read(
        &mut self,
        array: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (array, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsync(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = __cordl_object
            .invoke("ReadAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadByte(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadByte", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadData(
        &mut self,
        safeHandle: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::InteropServices::SafeHandle,
        >,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadData", (safeHandle, buf, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadInternal(
        &mut self,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadInternal", (dest, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadSegment(
        &mut self,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        dest_offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ReadSegment", (dest, dest_offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefillBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefillBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Seek(
        &mut self,
        offset: i64,
        origin: crate::System::IO::SeekOrigin,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("Seek", (offset, origin))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLength(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLength", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (array, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object
            .invoke("WriteAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteByte(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteByte", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteInternal(
        &mut self,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteInternal", (src, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteSegment(
        &mut self,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        src_offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("WriteSegment", (src, src_offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_FileMode_FileAccess2(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, mode, access))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_FileMode_FileAccess_FileShare3(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, mode, access, share))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_FileMode_FileAccess_FileShare_i32_4(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, mode, access, share, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_FileMode_FileAccess_FileShare_i32_FileOptions6(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        options: crate::System::IO::FileOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, mode, access, share, bufferSize, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_FileMode_FileAccess_FileShare_i32__cordl_bool5(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        useAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (path, mode, access, share, bufferSize, useAsync))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_FileMode_FileAccess_FileShare_i32__cordl_bool_FileOptions10(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        anonymous: bool,
        options: crate::System::IO::FileOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (path, mode, access, share, bufferSize, anonymous, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_FileMode_FileAccess_FileShare_i32__cordl_bool__cordl_bool9(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        share: crate::System::IO::FileShare,
        bufferSize: i32,
        isAsync: bool,
        anonymous: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (path, mode, access, share, bufferSize, isAsync, anonymous),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IntPtr_FileAccess__cordl_bool_i32_0(
        &mut self,
        handle: crate::System::IntPtr,
        access: crate::System::IO::FileAccess,
        ownsHandle: bool,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handle, access, ownsHandle, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IntPtr_FileAccess__cordl_bool_i32__cordl_bool__cordl_bool1(
        &mut self,
        handle: crate::System::IntPtr,
        access: crate::System::IO::FileAccess,
        ownsHandle: bool,
        bufferSize: i32,
        isAsync: bool,
        isConsoleWrapper: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (handle, access, ownsHandle, bufferSize, isAsync, isConsoleWrapper),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SafeFileHandle_FileAccess7(
        &mut self,
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        >,
        access: crate::System::IO::FileAccess,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handle, access))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SafeFileHandle_FileAccess_i32__cordl_bool8(
        &mut self,
        handle: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        >,
        access: crate::System::IO::FileAccess,
        bufferSize: i32,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handle, access, bufferSize, isAsync))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanRead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanRead", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanSeek(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanSeek", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanWrite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanWrite", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SafeFileHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Microsoft::Win32::SafeHandles::SafeFileHandle>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Microsoft::Win32::SafeHandles::SafeFileHandle,
        > = __cordl_object.invoke("get_SafeFileHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Position(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Position", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+FileStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::FileStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+FileStream+ReadDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct FileStream_ReadDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+IO+FileStream+ReadDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileStream_ReadDelegate =>
    "System.IO"."FileStream/ReadDelegate"
);
#[cfg(feature = "System+IO+FileStream+ReadDelegate")]
impl std::ops::Deref for crate::System::IO::FileStream_ReadDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileStream+ReadDelegate")]
impl std::ops::DerefMut for crate::System::IO::FileStream_ReadDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileStream+ReadDelegate")]
impl crate::System::IO::FileStream_ReadDelegate {
    pub fn BeginInvoke(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (buffer, offset, count, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Invoke", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+FileStream+ReadDelegate")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::FileStream_ReadDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+FileStream+WriteDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct FileStream_WriteDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+IO+FileStream+WriteDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileStream_WriteDelegate =>
    "System.IO"."FileStream/WriteDelegate"
);
#[cfg(feature = "System+IO+FileStream+WriteDelegate")]
impl std::ops::Deref for crate::System::IO::FileStream_WriteDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileStream+WriteDelegate")]
impl std::ops::DerefMut for crate::System::IO::FileStream_WriteDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+FileStream+WriteDelegate")]
impl crate::System::IO::FileStream_WriteDelegate {
    pub fn BeginInvoke(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (buffer, offset, count, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+FileStream+WriteDelegate")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::FileStream_WriteDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
