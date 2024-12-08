#[cfg(feature = "System+IO+PinnedBufferMemoryStream")]
#[repr(C)]
#[derive(Debug)]
pub struct PinnedBufferMemoryStream {
    __cordl_parent: crate::System::IO::UnmanagedMemoryStream,
    pub _array: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _pinningHandle: crate::System::Runtime::InteropServices::GCHandle,
}
#[cfg(feature = "System+IO+PinnedBufferMemoryStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IO::PinnedBufferMemoryStream =>
    "System.IO"."PinnedBufferMemoryStream"
);
#[cfg(feature = "System+IO+PinnedBufferMemoryStream")]
impl std::ops::Deref for crate::System::IO::PinnedBufferMemoryStream {
    type Target = crate::System::IO::UnmanagedMemoryStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+PinnedBufferMemoryStream")]
impl std::ops::DerefMut for crate::System::IO::PinnedBufferMemoryStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IO+PinnedBufferMemoryStream")]
impl crate::System::IO::PinnedBufferMemoryStream {
    pub fn Read(
        &mut self,
        buffer: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        buffer: crate::System::ReadOnlySpan_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (array))?;
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
    pub fn New(
        array: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (array))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+IO+PinnedBufferMemoryStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IO::PinnedBufferMemoryStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
