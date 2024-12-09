#[cfg(feature = "System+Text+Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Encoder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _fallback: *mut crate::System::Text::EncoderFallback,
    pub _fallbackBuffer: *mut crate::System::Text::EncoderFallbackBuffer,
}
#[cfg(feature = "System+Text+Encoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::Encoder => "System.Text"."Encoder"
);
#[cfg(feature = "System+Text+Encoder")]
impl std::ops::Deref for crate::System::Text::Encoder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoder")]
impl std::ops::DerefMut for crate::System::Text::Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoder")]
impl crate::System::Text::Encoder {
    pub fn Convert_Il2CppArray_i32_Il2CppArray_i32_i32__cordl_bool_ByRefMut_ByRefMut0(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        charIndex: i32,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteIndex: i32,
        byteCount: i32,
        flush: bool,
        charsUsed: quest_hook::libil2cpp::ByRefMut<i32>,
        bytesUsed: quest_hook::libil2cpp::ByRefMut<i32>,
        completed: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Convert",
                (
                    chars,
                    charIndex,
                    charCount,
                    bytes,
                    byteIndex,
                    byteCount,
                    flush,
                    charsUsed,
                    bytesUsed,
                    completed,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Convert_Il2CppObject_Il2CppObject_i32__cordl_bool_ByRefMut_ByRefMut1(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        flush: bool,
        charsUsed: quest_hook::libil2cpp::ByRefMut<i32>,
        bytesUsed: quest_hook::libil2cpp::ByRefMut<i32>,
        completed: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Convert",
                (
                    chars,
                    charCount,
                    bytes,
                    byteCount,
                    flush,
                    charsUsed,
                    bytesUsed,
                    completed,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetByteCount_Il2CppArray_i32__cordl_bool0(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetByteCount", (chars, index, count, flush))?;
        Ok(__cordl_ret)
    }
    pub fn GetByteCount_Il2CppObject__cordl_bool1(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        count: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetByteCount", (chars, count, flush))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_Il2CppArray_i32_Il2CppArray_i32__cordl_bool0(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        charIndex: i32,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteIndex: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBytes", (chars, charIndex, charCount, bytes, byteIndex, flush))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_Il2CppObject_Il2CppObject_i32__cordl_bool1(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBytes", (chars, charCount, bytes, byteCount, flush))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Fallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::EncoderFallback> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::EncoderFallback = __cordl_object
            .invoke("get_Fallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FallbackBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::EncoderFallbackBuffer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::EncoderFallbackBuffer = __cordl_object
            .invoke("get_FallbackBuffer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_InternalHasFallbackBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_InternalHasFallbackBuffer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+Encoder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
