#[cfg(feature = "System+Text+UTF8Encoding+UTF8Decoder")]
#[repr(C)]
#[derive(Debug)]
pub struct UTF8Encoding_UTF8Decoder {
    __cordl_parent: crate::System::Text::DecoderNLS,
    pub bits: i32,
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Decoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::UTF8Encoding_UTF8Decoder =>
    "System.Text"."UTF8Encoding/UTF8Decoder"
);
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Decoder")]
impl std::ops::Deref for crate::System::Text::UTF8Encoding_UTF8Decoder {
    type Target = crate::System::Text::DecoderNLS;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Decoder")]
impl std::ops::DerefMut for crate::System::Text::UTF8Encoding_UTF8Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Decoder")]
impl crate::System::Text::UTF8Encoding_UTF8Decoder {
    pub fn New(
        encoding: *mut crate::System::Text::UTF8Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoding))?;
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
        encoding: *mut crate::System::Text::UTF8Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasState", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Decoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::UTF8Encoding_UTF8Decoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct UTF8Encoding_UTF8Encoder {
    __cordl_parent: crate::System::Text::EncoderNLS,
    pub surrogateChar: i32,
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Encoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::UTF8Encoding_UTF8Encoder =>
    "System.Text"."UTF8Encoding/UTF8Encoder"
);
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Encoder")]
impl std::ops::Deref for crate::System::Text::UTF8Encoding_UTF8Encoder {
    type Target = crate::System::Text::EncoderNLS;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Encoder")]
impl std::ops::DerefMut for crate::System::Text::UTF8Encoding_UTF8Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Encoder")]
impl crate::System::Text::UTF8Encoding_UTF8Encoder {
    pub fn New(
        encoding: *mut crate::System::Text::UTF8Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoding))?;
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
        encoding: *mut crate::System::Text::UTF8Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasState(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasState", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+UTF8Encoding+UTF8Encoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::UTF8Encoding_UTF8Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+UTF8Encoding")]
#[repr(C)]
#[derive(Debug)]
pub struct UTF8Encoding {
    __cordl_parent: crate::System::Text::Encoding,
    pub _emitUTF8Identifier: bool,
    pub _isThrowException: bool,
}
#[cfg(feature = "System+Text+UTF8Encoding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::UTF8Encoding => "System.Text"
    ."UTF8Encoding"
);
#[cfg(feature = "System+Text+UTF8Encoding")]
impl std::ops::Deref for crate::System::Text::UTF8Encoding {
    type Target = crate::System::Text::Encoding;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding")]
impl std::ops::DerefMut for crate::System::Text::UTF8Encoding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+UTF8Encoding")]
impl crate::System::Text::UTF8Encoding {
    #[cfg(feature = "System+Text+UTF8Encoding+UTF8Decoder")]
    pub type UTF8Decoder = crate::System::Text::UTF8Encoding_UTF8Decoder;
    #[cfg(feature = "System+Text+UTF8Encoding+UTF8Encoder")]
    pub type UTF8Encoder = crate::System::Text::UTF8Encoding_UTF8Encoder;
    #[cfg(feature = "System+Text+UTF8Encoding+UTF8EncodingSealed")]
    pub type UTF8EncodingSealed = crate::GlobalNamespace::UTF8Encoding_UTF8EncodingSealed;
    pub fn Equals(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FallbackInvalidByteSequence_ByRefMut_ByRefMut0(
        &mut self,
        pSrc: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
        ch: i32,
        fallback: *mut crate::System::Text::DecoderFallbackBuffer,
        pTarget: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("FallbackInvalidByteSequence", (pSrc, ch, fallback, pTarget))?;
        Ok(__cordl_ret)
    }
    pub fn FallbackInvalidByteSequence_Il2CppObject1(
        &mut self,
        pSrc: *mut quest_hook::libil2cpp::Il2CppObject,
        ch: i32,
        fallback: *mut crate::System::Text::DecoderFallbackBuffer,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FallbackInvalidByteSequence", (pSrc, ch, fallback))?;
        Ok(__cordl_ret)
    }
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
    pub fn GetByteCount_Il2CppObject_i32_EncoderNLS3(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        count: i32,
        baseEncoder: *mut crate::System::Text::EncoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetByteCount", (chars, count, baseEncoder))?;
        Ok(__cordl_ret)
    }
    pub fn GetByteCount_String1(
        &mut self,
        chars: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetByteCount", (chars))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytesUnknown(
        &mut self,
        pSrc: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
        ch: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBytesUnknown", (pSrc, ch))?;
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
    pub fn GetBytes_Il2CppObject_Il2CppObject_i32_EncoderNLS3(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        baseEncoder: *mut crate::System::Text::EncoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBytes", (chars, charCount, bytes, byteCount, baseEncoder))?;
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
    pub fn GetCharCount_Il2CppObject_DecoderNLS2(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        count: i32,
        baseDecoder: *mut crate::System::Text::DecoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCharCount", (bytes, count, baseDecoder))?;
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
    pub fn GetChars_Il2CppObject_Il2CppObject_i32_DecoderNLS2(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        baseDecoder: *mut crate::System::Text::DecoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChars", (bytes, byteCount, chars, charCount, baseDecoder))?;
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMaxByteCount(
        &mut self,
        charCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMaxByteCount", (charCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetMaxCharCount(
        &mut self,
        byteCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMaxCharCount", (byteCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetPreamble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetPreamble", ())?;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        encoderShouldEmitUTF8Identifier: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoderShouldEmitUTF8Identifier))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool__cordl_bool2(
        encoderShouldEmitUTF8Identifier: bool,
        throwOnInvalidBytes: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (encoderShouldEmitUTF8Identifier, throwOnInvalidBytes),
            )?;
        Ok(__cordl_object)
    }
    pub fn SetDefaultFallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultFallbacks", ())?;
        Ok(__cordl_ret)
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
    pub fn _ctor__cordl_bool1(
        &mut self,
        encoderShouldEmitUTF8Identifier: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoderShouldEmitUTF8Identifier))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool__cordl_bool2(
        &mut self,
        encoderShouldEmitUTF8Identifier: bool,
        throwOnInvalidBytes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoderShouldEmitUTF8Identifier, throwOnInvalidBytes))?;
        Ok(__cordl_ret)
    }
    pub fn get_Preamble(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ReadOnlySpan_1<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ReadOnlySpan_1<u8> = __cordl_object
            .invoke("get_Preamble", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+UTF8Encoding")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::UTF8Encoding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
