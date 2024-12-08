#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncStreamReader {
    __cordl_parent: crate::System::Object,
    pub stream: *mut crate::System::IO::Stream,
    pub encoding: *mut crate::System::Text::Encoding,
    pub decoder: *mut crate::System::Text::Decoder,
    pub byteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub charBuffer: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub cancelOperation: bool,
    pub eofEvent: *mut crate::System::Threading::ManualResetEvent,
    pub syncObject: *mut crate::System::Object,
    pub asyncReadResult: *mut crate::System::IAsyncResult,
}
#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::AsyncStreamReader =>
    "System.Diagnostics"."AsyncStreamReader"
);
#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
impl std::ops::Deref for crate::System::Diagnostics::AsyncStreamReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
impl std::ops::DerefMut for crate::System::Diagnostics::AsyncStreamReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
impl crate::System::Diagnostics::AsyncStreamReader {
    pub fn CancelOperation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelOperation", ())?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
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
}
#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::AsyncStreamReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
