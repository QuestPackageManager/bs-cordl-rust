#[cfg(feature = "System+Net+WebUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct WebUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+WebUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::WebUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "WebUtility";
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
#[cfg(feature = "System+Net+WebUtility")]
impl std::ops::Deref for crate::System::Net::WebUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility")]
impl std::ops::DerefMut for crate::System::Net::WebUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility")]
impl crate::System::Net::WebUtility {
    #[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
    pub type UrlDecoder = crate::System::Net::WebUtility_UrlDecoder;
    pub fn GetNextUnicodeScalarValueFromUtf16Surrogate(
        pch: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        charsRemaining: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetNextUnicodeScalarValueFromUtf16Surrogate",
                (pch, charsRemaining),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HexToInt(h: char) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HexToInt", (h))?;
        Ok(__cordl_ret.into())
    }
    pub fn HtmlEncode_Il2CppString0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HtmlEncode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn HtmlEncode_TextWriter1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HtmlEncode", (value, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfHtmlEncodingChars(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startPos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOfHtmlEncodingChars", (s, startPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn UrlDecode(
        encodedValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UrlDecode", (encodedValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn UrlDecodeInternal(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UrlDecodeInternal", (value, encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HtmlEncodeConformance() -> quest_hook::libil2cpp::Result<
        crate::System::Net::Configuration::UnicodeEncodingConformance,
    > {
        let __cordl_ret: crate::System::Net::Configuration::UnicodeEncodingConformance = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HtmlEncodeConformance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
#[repr(C)]
#[derive(Debug)]
pub struct WebUtility_UrlDecoder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bufferSize: i32,
    pub _numChars: i32,
    pub _charBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub _numBytes: i32,
    pub _byteBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub _encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::WebUtility_UrlDecoder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "UrlDecoder";
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
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl std::ops::Deref for crate::System::Net::WebUtility_UrlDecoder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl std::ops::DerefMut for crate::System::Net::WebUtility_UrlDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl crate::System::Net::WebUtility_UrlDecoder {
    pub fn AddByte(
        &mut self,
        b: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddByte", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddChar(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddChar", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn FlushBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FlushBytes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bufferSize: i32,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bufferSize, encoding))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bufferSize: i32,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bufferSize, encoding))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+WebUtility+UrlDecoder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::WebUtility_UrlDecoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
