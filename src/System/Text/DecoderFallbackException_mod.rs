#[cfg(feature = "System+Text+DecoderFallbackException")]
#[repr(C)]
#[derive(Debug)]
pub struct DecoderFallbackException {
    __cordl_parent: crate::System::ArgumentException,
    pub _bytesUnknown: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _index: i32,
}
#[cfg(feature = "System+Text+DecoderFallbackException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::DecoderFallbackException =>
    "System.Text"."DecoderFallbackException"
);
#[cfg(feature = "System+Text+DecoderFallbackException")]
impl std::ops::Deref for crate::System::Text::DecoderFallbackException {
    type Target = crate::System::ArgumentException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+DecoderFallbackException")]
impl std::ops::DerefMut for crate::System::Text::DecoderFallbackException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+DecoderFallbackException")]
impl crate::System::Text::DecoderFallbackException {
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext2(
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_i32_1(
        message: *mut crate::System::String,
        bytesUnknown: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, bytesUnknown, index))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext2(
        &mut self,
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_i32_1(
        &mut self,
        message: *mut crate::System::String,
        bytesUnknown: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, bytesUnknown, index))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+DecoderFallbackException")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::DecoderFallbackException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}