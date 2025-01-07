#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncStreamReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    pub decoder: quest_hook::libil2cpp::Gc<crate::System::Text::Decoder>,
    pub byteBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub charBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub cancelOperation: bool,
    pub eofEvent: quest_hook::libil2cpp::Gc<crate::System::Threading::ManualResetEvent>,
    pub syncObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub asyncReadResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
}
#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Diagnostics::AsyncStreamReader {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics";
    const CLASS_NAME: &'static str = "AsyncStreamReader";
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
#[cfg(feature = "System+Diagnostics+AsyncStreamReader")]
impl std::ops::Deref for crate::System::Diagnostics::AsyncStreamReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", ())?;
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
