#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+EncodingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpListenerRequestUriBuilder_EncodingType {
    Primary = 0i32,
    Secondary = 1i32,
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+EncodingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::HttpListenerRequestUriBuilder_EncodingType => "System.Net"
    ."HttpListenerRequestUriBuilder/EncodingType"
);
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpListenerRequestUriBuilder {
    __cordl_parent: crate::System::Object,
    pub rawUri: *mut crate::System::String,
    pub cookedUriScheme: *mut crate::System::String,
    pub cookedUriHost: *mut crate::System::String,
    pub cookedUriPath: *mut crate::System::String,
    pub cookedUriQuery: *mut crate::System::String,
    pub requestUriString: *mut crate::System::Text::StringBuilder,
    pub rawOctets: *mut crate::System::Collections::Generic::List_1<u8>,
    pub rawPath: *mut crate::System::String,
    pub requestUri: *mut crate::System::Uri,
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpListenerRequestUriBuilder =>
    "System.Net"."HttpListenerRequestUriBuilder"
);
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
impl std::ops::Deref for crate::System::Net::HttpListenerRequestUriBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
impl std::ops::DerefMut for crate::System::Net::HttpListenerRequestUriBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
impl crate::System::Net::HttpListenerRequestUriBuilder {
    #[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+ParsingResult")]
    pub type ParsingResult = crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult;
    #[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+EncodingType")]
    pub type EncodingType = crate::System::Net::HttpListenerRequestUriBuilder_EncodingType;
    pub fn LogWarning(
        &mut self,
        methodName: *mut crate::System::String,
        message: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogWarning", (methodName, message, args))?;
        Ok(__cordl_ret)
    }
    pub fn BuildRequestUriUsingRawPath_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildRequestUriUsingRawPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn BuildRequestUriUsingRawPath_Encoding1(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult = __cordl_object
            .invoke("BuildRequestUriUsingRawPath", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn ParseRawPath(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult = __cordl_object
            .invoke("ParseRawPath", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn AppendUnicodeCodePointValuePercentEncoded(
        &mut self,
        codePoint: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AppendUnicodeCodePointValuePercentEncoded", (codePoint))?;
        Ok(__cordl_ret)
    }
    pub fn AddPercentEncodedOctetToRawOctetsList(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
        escapedCharacter: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "AddPercentEncodedOctetToRawOctetsList",
                (encoding, escapedCharacter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EmptyDecodeAndAppendRawOctetsList(
        &mut self,
        encoding: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EmptyDecodeAndAppendRawOctetsList", (encoding))?;
        Ok(__cordl_ret)
    }
    pub fn Build(&mut self) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        rawUri: *mut crate::System::String,
        cookedUriScheme: *mut crate::System::String,
        cookedUriHost: *mut crate::System::String,
        cookedUriPath: *mut crate::System::String,
        cookedUriQuery: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (rawUri, cookedUriScheme, cookedUriHost, cookedUriPath, cookedUriQuery),
            )?;
        Ok(__cordl_ret)
    }
    pub fn BuildRequestUriUsingCookedPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildRequestUriUsingCookedPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        rawUri: *mut crate::System::String,
        cookedUriScheme: *mut crate::System::String,
        cookedUriHost: *mut crate::System::String,
        cookedUriPath: *mut crate::System::String,
        cookedUriQuery: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (rawUri, cookedUriScheme, cookedUriHost, cookedUriPath, cookedUriQuery),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::HttpListenerRequestUriBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+ParsingResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpListenerRequestUriBuilder_ParsingResult {
    EncodingError = 2i32,
    InvalidString = 1i32,
    Success = 0i32,
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+ParsingResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::HttpListenerRequestUriBuilder_ParsingResult => "System.Net"
    ."HttpListenerRequestUriBuilder/ParsingResult"
);
