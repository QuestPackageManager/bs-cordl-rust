#[cfg(feature = "System+Net+Http+StreamContent")]
#[repr(C)]
#[derive(Debug)]
pub struct StreamContent {
    __cordl_parent: crate::System::Net::Http::HttpContent,
    pub content: *mut crate::System::IO::Stream,
    pub bufferSize: i32,
    pub cancellationToken: crate::System::Threading::CancellationToken,
    pub startPosition: i64,
    pub contentCopied: bool,
}
#[cfg(feature = "System+Net+Http+StreamContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::StreamContent =>
    "System.Net.Http"."StreamContent"
);
#[cfg(feature = "System+Net+Http+StreamContent")]
impl std::ops::Deref for crate::System::Net::Http::StreamContent {
    type Target = crate::System::Net::Http::HttpContent;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+StreamContent")]
impl std::ops::DerefMut for crate::System::Net::Http::StreamContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+StreamContent")]
impl crate::System::Net::Http::StreamContent {
    pub fn SerializeToStreamAsync(
        &mut self,
        stream: *mut crate::System::IO::Stream,
        context: *mut crate::System::Net::TransportContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("SerializeToStreamAsync", (stream, context))?;
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
    pub fn _ctor_Stream0(
        &mut self,
        content: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        content: *mut crate::System::IO::Stream,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content, bufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CancellationToken2(
        &mut self,
        content: *mut crate::System::IO::Stream,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn TryComputeLength(
        &mut self,
        length: quest_hook::libil2cpp::ByRefMut<i64>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryComputeLength", (length))?;
        Ok(__cordl_ret)
    }
    pub fn New_Stream0(
        content: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(
        content: *mut crate::System::IO::Stream,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content, bufferSize))?;
        Ok(__cordl_object)
    }
    pub fn New_CancellationToken2(
        content: *mut crate::System::IO::Stream,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content, cancellationToken))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Http+StreamContent")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::StreamContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
