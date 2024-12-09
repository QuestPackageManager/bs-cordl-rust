#[cfg(feature = "System+Text+Encoding")]
#[repr(C)]
#[derive(Debug)]
pub struct Encoding {
    __cordl_parent: crate::System::Object,
    pub m_codePage: i32,
    pub dataItem: *mut crate::System::Globalization::CodePageDataItem,
    pub m_deserializedFromEverett: bool,
    pub m_isReadOnly: bool,
    pub encoderFallback: *mut crate::System::Text::EncoderFallback,
    pub decoderFallback: *mut crate::System::Text::DecoderFallback,
}
#[cfg(feature = "System+Text+Encoding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::Encoding => "System.Text"
    ."Encoding"
);
#[cfg(feature = "System+Text+Encoding")]
impl std::ops::Deref for crate::System::Text::Encoding {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding")]
impl std::ops::DerefMut for crate::System::Text::Encoding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding")]
impl crate::System::Text::Encoding {
    #[cfg(feature = "System+Text+Encoding+DefaultDecoder")]
    pub type DefaultDecoder = crate::System::Text::Encoding_DefaultDecoder;
    #[cfg(feature = "System+Text+Encoding+DefaultEncoder")]
    pub type DefaultEncoder = crate::System::Text::Encoding_DefaultEncoder;
    #[cfg(feature = "System+Text+Encoding+EncodingByteBuffer")]
    pub type EncodingByteBuffer = crate::System::Text::Encoding_EncodingByteBuffer;
    #[cfg(feature = "System+Text+Encoding+EncodingCharBuffer")]
    pub type EncodingCharBuffer = crate::System::Text::Encoding_EncodingCharBuffer;
    pub fn Clone(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Clone", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeEncoding(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeserializeEncoding", (info, context))?;
        Ok(__cordl_ret)
    }
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
    pub fn GetBestFitBytesToUnicodeData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("GetBestFitBytesToUnicodeData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBestFitUnicodeToBytesData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("GetBestFitUnicodeToBytesData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetByteCount_Il2CppArray_i32_i32_1(
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
        encoder: *mut crate::System::Text::EncoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetByteCount", (chars, count, encoder))?;
        Ok(__cordl_ret)
    }
    pub fn GetByteCount_String0(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetByteCount", (s))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_Il2CppArray0(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBytes", (chars))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_Il2CppArray_i32_i32_1(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBytes", (chars, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_Il2CppArray_i32_i32_Il2CppArray_i32_2(
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
    pub fn GetBytes_Il2CppObject_i32_Il2CppObject_i32_6(
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
    pub fn GetBytes_Il2CppObject_i32_Il2CppObject_i32_EncoderNLS5(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        encoder: *mut crate::System::Text::EncoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBytes", (chars, charCount, bytes, byteCount, encoder))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_ReadOnlySpan_1_Span_1_7(
        &mut self,
        chars: crate::System::ReadOnlySpan_1<char>,
        bytes: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetBytes", (chars, bytes))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_String3(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBytes", (s))?;
        Ok(__cordl_ret)
    }
    pub fn GetBytes_String_i32_i32_Il2CppArray_i32_4(
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
        decoder: *mut crate::System::Text::DecoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCharCount", (bytes, count, decoder))?;
        Ok(__cordl_ret)
    }
    pub fn GetChars_Il2CppArray_i32_i32_0(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("GetChars", (bytes, index, count))?;
        Ok(__cordl_ret)
    }
    pub fn GetChars_Il2CppArray_i32_i32_Il2CppArray_i32_1(
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
    pub fn GetChars_Il2CppObject_i32_Il2CppObject_i32_2(
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
    pub fn GetChars_Il2CppObject_i32_Il2CppObject_i32_DecoderNLS3(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        decoder: *mut crate::System::Text::DecoderNLS,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChars", (bytes, byteCount, chars, charCount, decoder))?;
        Ok(__cordl_ret)
    }
    pub fn GetChars_ReadOnlySpan_1_Span_1_4(
        &mut self,
        bytes: crate::System::ReadOnlySpan_1<u8>,
        chars: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetChars", (bytes, chars))?;
        Ok(__cordl_ret)
    }
    pub fn GetDataItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetDataItem", ())?;
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
    pub fn GetString_Il2CppArray2(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (bytes))?;
        Ok(__cordl_ret)
    }
    pub fn GetString_Il2CppArray_i32_i32_3(
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
    pub fn GetString_Il2CppObject_i32_0(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (bytes, byteCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetString_ReadOnlySpan_1_1(
        &mut self,
        bytes: crate::System::ReadOnlySpan_1<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (bytes))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(codePage: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (codePage))?;
        Ok(__cordl_object)
    }
    pub fn OnDeserialized_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDeserialized_StreamingContext1(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn OnDeserializing_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserializing", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDeserializing_StreamingContext1(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserializing", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn OnSerializing(
        &mut self,
        ctx: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerializing", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeEncoding(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeEncoding", (info, context))?;
        Ok(__cordl_ret)
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
    pub fn ThrowBytesOverflow_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowBytesOverflow", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThrowBytesOverflow_EncoderNLS__cordl_bool1(
        &mut self,
        encoder: *mut crate::System::Text::EncoderNLS,
        nothingEncoded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowBytesOverflow", (encoder, nothingEncoded))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowCharsOverflow_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowCharsOverflow", ())?;
        Ok(__cordl_ret)
    }
    pub fn ThrowCharsOverflow_DecoderNLS__cordl_bool1(
        &mut self,
        decoder: *mut crate::System::Text::DecoderNLS,
        nothingDecoded: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ThrowCharsOverflow", (decoder, nothingDecoded))?;
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
    pub fn _ctor_i32_1(
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
    pub fn get_CodePage(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CodePage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DecoderFallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::DecoderFallback> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::DecoderFallback = __cordl_object
            .invoke("get_DecoderFallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncoderFallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::EncoderFallback> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::EncoderFallback = __cordl_object
            .invoke("get_EncoderFallback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncodingName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_EncodingName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsReadOnly", ())?;
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
    pub fn get_WebName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_WebName", ())?;
        Ok(__cordl_ret)
    }
    pub fn setReadOnly(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("setReadOnly", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DecoderFallback(
        &mut self,
        value: *mut crate::System::Text::DecoderFallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DecoderFallback", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_EncoderFallback(
        &mut self,
        value: *mut crate::System::Text::EncoderFallback,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EncoderFallback", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+Encoding")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::Encoding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultDecoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Encoding_DefaultDecoder {
    __cordl_parent: crate::System::Text::Decoder,
    pub m_encoding: *mut crate::System::Text::Encoding,
    pub m_hasInitializedEncoding: bool,
}
#[cfg(feature = "System+Text+Encoding+DefaultDecoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::Encoding_DefaultDecoder =>
    "System.Text"."Encoding/DefaultDecoder"
);
#[cfg(feature = "System+Text+Encoding+DefaultDecoder")]
impl std::ops::Deref for crate::System::Text::Encoding_DefaultDecoder {
    type Target = crate::System::Text::Decoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultDecoder")]
impl std::ops::DerefMut for crate::System::Text::Encoding_DefaultDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultDecoder")]
impl crate::System::Text::Encoding_DefaultDecoder {
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
    pub fn GetCharCount_Il2CppArray_i32__cordl_bool1(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
        count: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCharCount", (bytes, index, count, flush))?;
        Ok(__cordl_ret)
    }
    pub fn GetCharCount_Il2CppObject__cordl_bool2(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        count: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCharCount", (bytes, count, flush))?;
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
    pub fn GetChars_Il2CppArray_i32_Il2CppArray_i32__cordl_bool1(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        byteIndex: i32,
        byteCount: i32,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        charIndex: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChars", (bytes, byteIndex, byteCount, chars, charIndex, flush))?;
        Ok(__cordl_ret)
    }
    pub fn GetChars_Il2CppObject_Il2CppObject_i32__cordl_bool2(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
        chars: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        flush: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChars", (bytes, byteCount, chars, charCount, flush))?;
        Ok(__cordl_ret)
    }
    pub fn GetRealObject(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetRealObject", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New_Encoding0(
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoding))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext1(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Encoding0(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultDecoder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::Encoding_DefaultDecoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultEncoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Encoding_DefaultEncoder {
    __cordl_parent: crate::System::Text::Encoder,
    pub m_encoding: *mut crate::System::Text::Encoding,
    pub m_hasInitializedEncoding: bool,
    pub charLeftOver: char,
}
#[cfg(feature = "System+Text+Encoding+DefaultEncoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::Encoding_DefaultEncoder =>
    "System.Text"."Encoding/DefaultEncoder"
);
#[cfg(feature = "System+Text+Encoding+DefaultEncoder")]
impl std::ops::Deref for crate::System::Text::Encoding_DefaultEncoder {
    type Target = crate::System::Text::Encoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultEncoder")]
impl std::ops::DerefMut for crate::System::Text::Encoding_DefaultEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultEncoder")]
impl crate::System::Text::Encoding_DefaultEncoder {
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
    pub fn GetRealObject(
        &mut self,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetRealObject", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New_Encoding0(
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encoding))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext1(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Encoding0(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+Encoding+DefaultEncoder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::Encoding_DefaultEncoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingByteBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct Encoding_EncodingByteBuffer {
    __cordl_parent: crate::System::Object,
    pub bytes: *mut quest_hook::libil2cpp::Il2CppObject,
    pub byteStart: *mut quest_hook::libil2cpp::Il2CppObject,
    pub byteEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    pub chars: *mut quest_hook::libil2cpp::Il2CppObject,
    pub charStart: *mut quest_hook::libil2cpp::Il2CppObject,
    pub charEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    pub byteCountResult: i32,
    pub enc: *mut crate::System::Text::Encoding,
    pub encoder: *mut crate::System::Text::EncoderNLS,
    pub fallbackBuffer: *mut crate::System::Text::EncoderFallbackBuffer,
}
#[cfg(feature = "System+Text+Encoding+EncodingByteBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::Encoding_EncodingByteBuffer =>
    "System.Text"."Encoding/EncodingByteBuffer"
);
#[cfg(feature = "System+Text+Encoding+EncodingByteBuffer")]
impl std::ops::Deref for crate::System::Text::Encoding_EncodingByteBuffer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingByteBuffer")]
impl std::ops::DerefMut for crate::System::Text::Encoding_EncodingByteBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingByteBuffer")]
impl crate::System::Text::Encoding_EncodingByteBuffer {
    pub fn AddByte_i32_0(
        &mut self,
        b: u8,
        moreBytesExpected: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddByte", (b, moreBytesExpected))?;
        Ok(__cordl_ret)
    }
    pub fn AddByte_u8_1(&mut self, b1: u8) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AddByte", (b1))?;
        Ok(__cordl_ret)
    }
    pub fn AddByte_u8_2(
        &mut self,
        b1: u8,
        b2: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AddByte", (b1, b2))?;
        Ok(__cordl_ret)
    }
    pub fn AddByte_u8_i32_3(
        &mut self,
        b1: u8,
        b2: u8,
        moreBytesExpected: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddByte", (b1, b2, moreBytesExpected))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextChar(&mut self) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("GetNextChar", ())?;
        Ok(__cordl_ret)
    }
    pub fn MovePrevious(
        &mut self,
        bThrow: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MovePrevious", (bThrow))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        inEncoding: *mut crate::System::Text::Encoding,
        inEncoder: *mut crate::System::Text::EncoderNLS,
        inByteStart: *mut quest_hook::libil2cpp::Il2CppObject,
        inByteCount: i32,
        inCharStart: *mut quest_hook::libil2cpp::Il2CppObject,
        inCharCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    inEncoding,
                    inEncoder,
                    inByteStart,
                    inByteCount,
                    inCharStart,
                    inCharCount,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        inEncoding: *mut crate::System::Text::Encoding,
        inEncoder: *mut crate::System::Text::EncoderNLS,
        inByteStart: *mut quest_hook::libil2cpp::Il2CppObject,
        inByteCount: i32,
        inCharStart: *mut quest_hook::libil2cpp::Il2CppObject,
        inCharCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    inEncoding,
                    inEncoder,
                    inByteStart,
                    inByteCount,
                    inCharStart,
                    inCharCount,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CharsUsed(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CharsUsed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MoreData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_MoreData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingByteBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::Encoding_EncodingByteBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingCharBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct Encoding_EncodingCharBuffer {
    __cordl_parent: crate::System::Object,
    pub chars: *mut quest_hook::libil2cpp::Il2CppObject,
    pub charStart: *mut quest_hook::libil2cpp::Il2CppObject,
    pub charEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    pub charCountResult: i32,
    pub enc: *mut crate::System::Text::Encoding,
    pub decoder: *mut crate::System::Text::DecoderNLS,
    pub byteStart: *mut quest_hook::libil2cpp::Il2CppObject,
    pub byteEnd: *mut quest_hook::libil2cpp::Il2CppObject,
    pub bytes: *mut quest_hook::libil2cpp::Il2CppObject,
    pub fallbackBuffer: *mut crate::System::Text::DecoderFallbackBuffer,
}
#[cfg(feature = "System+Text+Encoding+EncodingCharBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::Encoding_EncodingCharBuffer =>
    "System.Text"."Encoding/EncodingCharBuffer"
);
#[cfg(feature = "System+Text+Encoding+EncodingCharBuffer")]
impl std::ops::Deref for crate::System::Text::Encoding_EncodingCharBuffer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingCharBuffer")]
impl std::ops::DerefMut for crate::System::Text::Encoding_EncodingCharBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingCharBuffer")]
impl crate::System::Text::Encoding_EncodingCharBuffer {
    pub fn AddChar__cordl_char1(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AddChar", (ch))?;
        Ok(__cordl_ret)
    }
    pub fn AddChar_i32_0(
        &mut self,
        ch: char,
        numBytes: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AddChar", (ch, numBytes))?;
        Ok(__cordl_ret)
    }
    pub fn AdjustBytes(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdjustBytes", (count))?;
        Ok(__cordl_ret)
    }
    pub fn Fallback_Il2CppArray1(
        &mut self,
        byteBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Fallback", (byteBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn Fallback_u8_0(
        &mut self,
        fallbackByte: u8,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Fallback", (fallbackByte))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextByte(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("GetNextByte", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        enc: *mut crate::System::Text::Encoding,
        decoder: *mut crate::System::Text::DecoderNLS,
        charStart: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        byteStart: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (enc, decoder, charStart, charCount, byteStart, byteCount),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        enc: *mut crate::System::Text::Encoding,
        decoder: *mut crate::System::Text::DecoderNLS,
        charStart: *mut quest_hook::libil2cpp::Il2CppObject,
        charCount: i32,
        byteStart: *mut quest_hook::libil2cpp::Il2CppObject,
        byteCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (enc, decoder, charStart, charCount, byteStart, byteCount),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_BytesUsed(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BytesUsed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MoreData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_MoreData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+Encoding+EncodingCharBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::Encoding_EncodingCharBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
