#[cfg(feature = "System+Text+EncodingProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Text+EncodingProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::EncodingProvider => "System.Text"
    ."EncodingProvider"
);
#[cfg(feature = "System+Text+EncodingProvider")]
impl std::ops::Deref for crate::System::Text::EncodingProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingProvider")]
impl std::ops::DerefMut for crate::System::Text::EncodingProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingProvider")]
impl crate::System::Text::EncodingProvider {
    pub fn GetEncoding_Il2CppString0(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("GetEncoding", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoding_i32_1(
        &mut self,
        codepage: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("GetEncoding", (codepage))?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoding_i32_EncoderFallback_DecoderFallback2(
        &mut self,
        codepage: i32,
        encoderFallback: *mut crate::System::Text::EncoderFallback,
        decoderFallback: *mut crate::System::Text::DecoderFallback,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("GetEncoding", (codepage, encoderFallback, decoderFallback))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+EncodingProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::EncodingProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
