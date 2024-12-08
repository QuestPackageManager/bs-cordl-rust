#[cfg(feature = "System+IO+Compression+DeflateStreamNative")]
#[repr(C)]
#[derive(Debug)]
pub struct DeflateStreamNative {
    __cordl_parent: crate::System::Object,
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
    type Target = crate::System::Object;
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
        _cordl_where: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckResult", (result, _cordl_where))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReadZStream(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ReadZStream", (buffer, length))?;
        Ok(__cordl_ret)
    }
    pub fn UnmanagedRead(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("UnmanagedRead", (buffer, length))?;
        Ok(__cordl_ret)
    }
    pub fn UnmanagedWrite(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("UnmanagedWrite", (buffer, length))?;
        Ok(__cordl_ret)
    }
    pub fn WriteZStream(
        &mut self,
        buffer: crate::System::IntPtr,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteZStream", (buffer, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReleaseHandle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleaseHandle", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsInvalid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInvalid", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
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
