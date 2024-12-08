#[cfg(feature = "System+Text+EncoderFallbackException")]
#[repr(C)]
#[derive(Debug)]
pub struct EncoderFallbackException {
    __cordl_parent: crate::System::ArgumentException,
    pub _charUnknown: char,
    pub _charUnknownHigh: char,
    pub _charUnknownLow: char,
    pub _index: i32,
}
#[cfg(feature = "System+Text+EncoderFallbackException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::EncoderFallbackException =>
    "System.Text"."EncoderFallbackException"
);
#[cfg(feature = "System+Text+EncoderFallbackException")]
impl std::ops::Deref for crate::System::Text::EncoderFallbackException {
    type Target = crate::System::ArgumentException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncoderFallbackException")]
impl std::ops::DerefMut for crate::System::Text::EncoderFallbackException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncoderFallbackException")]
impl crate::System::Text::EncoderFallbackException {
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
    pub fn _ctor_String__cordl_char_i32_1(
        &mut self,
        message: *mut crate::System::String,
        charUnknown: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, charUnknown, index))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String__cordl_char__cordl_char_i32_2(
        &mut self,
        message: *mut crate::System::String,
        charUnknownHigh: char,
        charUnknownLow: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, charUnknownHigh, charUnknownLow, index))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext3(
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String__cordl_char_i32_1(
        message: *mut crate::System::String,
        charUnknown: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, charUnknown, index))?;
        Ok(__cordl_object)
    }
    pub fn New_String__cordl_char__cordl_char_i32_2(
        message: *mut crate::System::String,
        charUnknownHigh: char,
        charUnknownLow: char,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, charUnknownHigh, charUnknownLow, index))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext3(
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Text+EncoderFallbackException")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::EncoderFallbackException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
