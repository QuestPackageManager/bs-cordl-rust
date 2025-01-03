#[cfg(feature = "System+IO+Compression+DeflateStreamNative")]
#[repr(C)]
#[derive(Debug)]
pub struct DeflateStreamNative {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub feeder: *mut crate::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite,
    pub base_stream: *mut crate::System::IO::Stream,
    pub z_stream: *mut crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle,
    pub data: crate::System::Runtime::InteropServices::GCHandle,
    pub disposed: bool,
    pub io_buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub last_error: *mut crate::System::Exception,
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::Compression::DeflateStreamNative =>
    "System.IO.Compression"."DeflateStreamNative"
);
#[cfg(feature = "System+IO+Compression+DeflateStreamNative")]
impl std::ops::Deref for crate::System::IO::Compression::DeflateStreamNative {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative")]
impl std::ops::DerefMut for crate::System::IO::Compression::DeflateStreamNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative")]
impl crate::System::IO::Compression::DeflateStreamNative {
    #[cfg(feature = "System+IO+Compression+DeflateStreamNative+SafeDeflateStreamHandle")]
    pub type SafeDeflateStreamHandle = crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle;
    #[cfg(feature = "System+IO+Compression+DeflateStreamNative+UnmanagedReadOrWrite")]
    pub type UnmanagedReadOrWrite = crate::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite;
    pub fn CheckResult(
        &mut self,
        result: i32,
        _cordl_where: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckResult", (result, _cordl_where))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseZStream(
        stream: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CloseZStream", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        compressedStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        mode: crate::System::IO::Compression::CompressionMode,
        gzip: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Compression::DeflateStreamNative>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::Compression::DeflateStreamNative,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (compressedStream, mode, gzip))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateZStream(
        compress: crate::System::IO::Compression::CompressionMode,
        gzip: bool,
        feeder: quest_hook::libil2cpp::Gc<
            crate::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite,
        >,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateZStream", (compress, gzip, feeder, data))?;
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
    pub fn Flush_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Flush_DeflateStreamNative_SafeDeflateStreamHandle1(
        stream: quest_hook::libil2cpp::Gc<
            crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Flush", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadZStream_DeflateStreamNative_SafeDeflateStreamHandle_IntPtr_i32_1(
        stream: quest_hook::libil2cpp::Gc<
            crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle,
        >,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadZStream", (stream, buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadZStream_IntPtr_i32_0(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadZStream", (buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnmanagedRead_IntPtr0(
        buffer: crate::System::IntPtr,
        length: i32,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnmanagedRead", (buffer, length, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnmanagedRead_IntPtr_i32_1(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("UnmanagedRead", (buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnmanagedWrite_IntPtr0(
        buffer: crate::System::IntPtr,
        length: i32,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnmanagedWrite", (buffer, length, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnmanagedWrite_IntPtr_i32_1(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("UnmanagedWrite", (buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteZStream_DeflateStreamNative_SafeDeflateStreamHandle_IntPtr_i32_1(
        stream: quest_hook::libil2cpp::Gc<
            crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle,
        >,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteZStream", (stream, buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteZStream_IntPtr_i32_0(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteZStream", (buffer, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::IO::Compression::DeflateStreamNative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+SafeDeflateStreamHandle")]
#[repr(C)]
#[derive(Debug)]
pub struct DeflateStreamNative_SafeDeflateStreamHandle {
    __cordl_parent: crate::System::Runtime::InteropServices::SafeHandle,
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+SafeDeflateStreamHandle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle =>
    "System.IO.Compression"."DeflateStreamNative/SafeDeflateStreamHandle"
);
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+SafeDeflateStreamHandle")]
impl std::ops::Deref
for crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle {
    type Target = crate::System::Runtime::InteropServices::SafeHandle;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+SafeDeflateStreamHandle")]
impl std::ops::DerefMut
for crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+SafeDeflateStreamHandle")]
impl crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReleaseHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsInvalid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInvalid", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+SafeDeflateStreamHandle")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::IO::Compression::DeflateStreamNative_SafeDeflateStreamHandle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+UnmanagedReadOrWrite")]
#[repr(C)]
#[derive(Debug)]
pub struct DeflateStreamNative_UnmanagedReadOrWrite {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+UnmanagedReadOrWrite")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite =>
    "System.IO.Compression"."DeflateStreamNative/UnmanagedReadOrWrite"
);
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+UnmanagedReadOrWrite")]
impl std::ops::Deref
for crate::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+UnmanagedReadOrWrite")]
impl std::ops::DerefMut
for crate::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+UnmanagedReadOrWrite")]
impl crate::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite {
    pub fn Invoke(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
        data: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Invoke", (buffer, length, data))?;
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
#[cfg(feature = "System+IO+Compression+DeflateStreamNative+UnmanagedReadOrWrite")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::IO::Compression::DeflateStreamNative_UnmanagedReadOrWrite {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
