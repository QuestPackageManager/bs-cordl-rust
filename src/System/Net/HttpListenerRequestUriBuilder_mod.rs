#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpListenerRequestUriBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub rawUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cookedUriScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cookedUriHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cookedUriPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cookedUriQuery: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub requestUriString: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub rawOctets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<u8>,
    >,
    pub rawPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub requestUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpListenerRequestUriBuilder =>
    "System.Net"."HttpListenerRequestUriBuilder"
);
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder")]
impl std::ops::Deref for crate::System::Net::HttpListenerRequestUriBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+EncodingType")]
    pub type EncodingType = crate::System::Net::HttpListenerRequestUriBuilder_EncodingType;
    #[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+ParsingResult")]
    pub type ParsingResult = crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult;
    pub fn AddPercentEncodedOctetToRawOctetsList(
        &mut self,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
        escapedCharacter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "AddPercentEncodedOctetToRawOctetsList",
                (encoding, escapedCharacter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSlashToAsteriskOnlyPath(
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddSlashToAsteriskOnlyPath", (path))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendOctetsPercentEncoded(
        target: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        octets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendOctetsPercentEncoded", (target, octets))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendUnicodeCodePointValuePercentEncoded(
        &mut self,
        codePoint: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AppendUnicodeCodePointValuePercentEncoded", (codePoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildRequestUriUsingCookedPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildRequestUriUsingCookedPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildRequestUriUsingRawPath_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildRequestUriUsingRawPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildRequestUriUsingRawPath_Encoding1(
        &mut self,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult = __cordl_object
            .invoke("BuildRequestUriUsingRawPath", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn EmptyDecodeAndAppendRawOctetsList(
        &mut self,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("EmptyDecodeAndAppendRawOctetsList", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEncoding(
        _cordl_type: crate::System::Net::HttpListenerRequestUriBuilder_EncodingType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEncoding", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOctetsAsString(
        octets: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOctetsAsString", (octets))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPath(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPath", (uriString))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestUri(
        rawUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriQuery: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetRequestUri",
                (rawUri, cookedUriScheme, cookedUriHost, cookedUriPath, cookedUriQuery),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LogWarning(
        &mut self,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LogWarning", (methodName, message, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        rawUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriQuery: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (rawUri, cookedUriScheme, cookedUriHost, cookedUriPath, cookedUriQuery),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn ParseRawPath(
        &mut self,
        encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::HttpListenerRequestUriBuilder_ParsingResult = __cordl_object
            .invoke("ParseRawPath", (encoding))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rawUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cookedUriQuery: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (rawUri, cookedUriScheme, cookedUriHost, cookedUriPath, cookedUriQuery),
            )?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+EncodingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpListenerRequestUriBuilder_EncodingType {
    #[default]
    Primary = 0i32,
    Secondary = 1i32,
}
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+EncodingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::HttpListenerRequestUriBuilder_EncodingType => "System.Net"
    ."HttpListenerRequestUriBuilder/EncodingType"
);
#[cfg(feature = "System+Net+HttpListenerRequestUriBuilder+ParsingResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpListenerRequestUriBuilder_ParsingResult {
    #[default]
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
