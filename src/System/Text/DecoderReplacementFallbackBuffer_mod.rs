#[cfg(feature = "System+Text+DecoderReplacementFallbackBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct DecoderReplacementFallbackBuffer {
    __cordl_parent: crate::System::Text::DecoderFallbackBuffer,
    pub _strDefault: *mut quest_hook::libil2cpp::Il2CppString,
    pub _fallbackCount: i32,
    pub _fallbackIndex: i32,
}
#[cfg(feature = "System+Text+DecoderReplacementFallbackBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::DecoderReplacementFallbackBuffer
    => "System.Text"."DecoderReplacementFallbackBuffer"
);
#[cfg(feature = "System+Text+DecoderReplacementFallbackBuffer")]
impl std::ops::Deref for crate::System::Text::DecoderReplacementFallbackBuffer {
    type Target = crate::System::Text::DecoderFallbackBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+DecoderReplacementFallbackBuffer")]
impl std::ops::DerefMut for crate::System::Text::DecoderReplacementFallbackBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+DecoderReplacementFallbackBuffer")]
impl crate::System::Text::DecoderReplacementFallbackBuffer {
    pub fn Fallback(
        &mut self,
        bytesUnknown: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Fallback", (bytesUnknown, index))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("GetNextChar", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalFallback(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        pBytes: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalFallback", (bytes, pBytes))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        fallback: *mut crate::System::Text::DecoderReplacementFallback,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fallback))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        fallback: *mut crate::System::Text::DecoderReplacementFallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fallback))?;
        Ok(__cordl_ret)
    }
    pub fn get_Remaining(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Remaining", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+DecoderReplacementFallbackBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::DecoderReplacementFallbackBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
