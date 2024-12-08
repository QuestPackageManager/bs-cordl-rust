#[cfg(feature = "System+Text+EncodingNLS")]
#[repr(C)]
#[derive(Debug)]
pub struct EncodingNLS {
    __cordl_parent: crate::System::Text::Encoding,
}
#[cfg(feature = "System+Text+EncodingNLS")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::EncodingNLS => "System.Text"
    ."EncodingNLS"
);
#[cfg(feature = "System+Text+EncodingNLS")]
impl std::ops::Deref for crate::System::Text::EncodingNLS {
    type Target = crate::System::Text::Encoding;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingNLS")]
impl std::ops::DerefMut for crate::System::Text::EncodingNLS {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+EncodingNLS")]
impl crate::System::Text::EncodingNLS {
    pub fn GetByteCount_Il2CppArray_i32_i32_0(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetByteCount", (chars, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn GetByteCount_Il2CppObject_i32_2(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetByteCount", (chars, count))?;
        Ok(__cordl_ret)
    }
    pub fn GetByteCount_String1(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetByteCount", (s))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_Il2CppArray_i32_Il2CppArray_i32_1(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        charIndex: i32,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBytes", (chars, charIndex, charCount, bytes, byteIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_Il2CppObject_Il2CppObject_i32_2(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBytes", (chars, charCount, bytes, byteCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_String_i32_Il2CppArray_i32_0(
        &mut self,
        s: *mut crate::System::String,
        charIndex: i32,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBytes", (s, charIndex, charCount, bytes, byteIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetCharCount_Il2CppArray_i32_0(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCharCount", (bytes, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn GetCharCount_Il2CppObject1(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetCharCount", (bytes, count))?;
        Ok(__cordl_ret)
    }
    pub fn GetChars_Il2CppArray_i32_Il2CppArray_i32_0(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteIndex: i32,
        byteCount: i32,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        charIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChars", (bytes, byteIndex, byteCount, chars, charIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetChars_Il2CppObject_Il2CppObject_i32_1(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChars", (bytes, byteCount, chars, charCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetDecoder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Decoder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Decoder = __cordl_object
            .invoke("GetDecoder", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEncoder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoder = __cordl_object
            .invoke("GetEncoder", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (bytes, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn New(codePage: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (codePage))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        codePage: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (codePage))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+EncodingNLS")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::EncodingNLS {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}