#[cfg(feature = "System+Uri")]
#[repr(C)]
#[derive(Debug)]
pub struct Uri {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_String: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_originalUnicodeString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_Syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    pub m_DnsSafeHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Flags: crate::System::Uri_Flags,
    pub m_Info: quest_hook::libil2cpp::Gc<crate::System::Uri_UriInfo>,
    pub m_iriParsing: bool,
}
#[cfg(feature = "System+Uri")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Uri => "System"."Uri"
);
#[cfg(feature = "System+Uri")]
impl std::ops::Deref for crate::System::Uri {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Uri")]
impl std::ops::DerefMut for crate::System::Uri {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Uri")]
impl crate::System::Uri {
    pub const c_DummyChar: char = '\u{ffff}';
    pub const c_EOL: char = '\u{fffe}';
    pub const c_Max16BitUtf8SequenceLength: i32 = 12i32;
    pub const c_MaxUriBufferSize: i32 = 65520i32;
    pub const c_MaxUriSchemeName: i32 = 1024i32;
    #[cfg(feature = "System+Uri+Check")]
    pub type Check = crate::System::Uri_Check;
    #[cfg(feature = "System+Uri+Flags")]
    pub type Flags = crate::System::Uri_Flags;
    #[cfg(feature = "System+Uri+MoreInfo")]
    pub type MoreInfo = crate::System::Uri_MoreInfo;
    #[cfg(feature = "System+Uri+Offset")]
    pub type Offset = crate::System::Uri_Offset;
    #[cfg(feature = "System+Uri+UriInfo")]
    pub type UriInfo = crate::System::Uri_UriInfo;
    pub fn AllowIdnStatic(
        &mut self,
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        flags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AllowIdnStatic", (syntax, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateCaseInsensitiveHashCode(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateCaseInsensitiveHashCode", (text))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAuthorityHelper(
        &mut self,
        pString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        idx: u16,
        length: u16,
        err: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingError>,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        newHost: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object
            .invoke(
                "CheckAuthorityHelper",
                (pString, idx, length, err, flags, syntax, newHost),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAuthorityHelperHandleAnyHostIri(
        &mut self,
        pString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        startInput: i32,
        end: i32,
        iriParsing: bool,
        hasUnicode: bool,
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        newHost: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        err: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CheckAuthorityHelperHandleAnyHostIri",
                (
                    pString,
                    startInput,
                    end,
                    iriParsing,
                    hasUnicode,
                    syntax,
                    flags,
                    newHost,
                    err,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckAuthorityHelperHandleDnsIri(
        &mut self,
        pString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: u16,
        end: i32,
        startInput: i32,
        iriParsing: bool,
        hasUnicode: bool,
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        userInfoString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        justNormalized: quest_hook::libil2cpp::ByRefMut<bool>,
        newHost: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        err: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CheckAuthorityHelperHandleDnsIri",
                (
                    pString,
                    start,
                    end,
                    startInput,
                    iriParsing,
                    hasUnicode,
                    syntax,
                    userInfoString,
                    flags,
                    justNormalized,
                    newHost,
                    err,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCanonical(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        idx: quest_hook::libil2cpp::ByRefMut<u16>,
        end: u16,
        delim: char,
    ) -> quest_hook::libil2cpp::Result<crate::System::Uri_Check> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Uri_Check = __cordl_object
            .invoke("CheckCanonical", (str, idx, end, delim))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForColonInFirstPathSegment(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckForColonInFirstPathSegment", (uriString))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForConfigLoad(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckForConfigLoad", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForEscapedUnreserved(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckForEscapedUnreserved", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForUnicode(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckForUnicode", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckKnownSchemes(
        lptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        nChars: u16,
        syntax: quest_hook::libil2cpp::ByRefMut<*mut crate::System::UriParser>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckKnownSchemes", (lptr, nChars, syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckSchemeName(
        schemeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckSchemeName", (schemeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckSchemeSyntax(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: u16,
        syntax: quest_hook::libil2cpp::ByRefMut<*mut crate::System::UriParser>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParsingError> {
        let __cordl_ret: crate::System::ParsingError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckSchemeSyntax", (ptr, length, syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineUri(
        basePart: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativePart: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        uriFormat: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineUri", (basePart, relativePart, uriFormat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compress(
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: u16,
        destLength: quest_hook::libil2cpp::ByRefMut<i32>,
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compress", (dest, start, destLength, syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHelper(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dontEscape: bool,
        uriKind: crate::System::UriKind,
        e: quest_hook::libil2cpp::ByRefMut<*mut crate::System::UriFormatException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHelper", (uriString, dontEscape, uriKind, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHostString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateHostString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHostStringHelper(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: u16,
        end: u16,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        scopeId: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateHostStringHelper", (str, idx, end, flags, scopeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateThis(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dontEscape: bool,
        uriKind: crate::System::UriKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateThis", (uri, dontEscape, uriKind))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateThisFromUri(
        &mut self,
        otherUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateThisFromUri", (otherUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUri(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dontEscape: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateUri", (baseUri, relativeUri, dontEscape))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateUriInfo(
        &mut self,
        cF: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateUriInfo", (cF))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureHostString(
        &mut self,
        allowDnsOptimization: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureHostString", (allowDnsOptimization))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureParseRemaining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureParseRemaining", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureUriInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Uri_UriInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri_UriInfo> = __cordl_object
            .invoke("EnsureUriInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        comparand: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn EscapeUnescapeIri(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        component: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("EscapeUnescapeIri", (input, start, end, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn EscapeUriString(
        stringToEscape: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EscapeUriString", (stringToEscape))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindEndOfComponent_Il2CppObject1(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        idx: quest_hook::libil2cpp::ByRefMut<u16>,
        end: u16,
        delim: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindEndOfComponent", (str, idx, end, delim))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindEndOfComponent_Il2CppString0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: quest_hook::libil2cpp::ByRefMut<u16>,
        end: u16,
        delim: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindEndOfComponent", (input, idx, end, delim))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromHex(digit: char) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromHex", (digit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCanonicalPath(
        &mut self,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = __cordl_object.invoke("GetCanonicalPath", (dest, pos, formatAs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCombinedString(
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        dontEscape: bool,
        result: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParsingError> {
        let __cordl_ret: crate::System::ParsingError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCombinedString", (baseUri, relativeStr, dontEscape, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponents(
        &mut self,
        components: crate::System::UriComponents,
        format: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetComponents", (components, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponentsHelper(
        &mut self,
        uriComponents: crate::System::UriComponents,
        uriFormat: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetComponentsHelper", (uriComponents, uriFormat))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEscapedParts(
        &mut self,
        uriParts: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetEscapedParts", (uriParts))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetException(
        err: crate::System::ParsingError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriFormatException> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetException", (err))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHostViaCustomSyntax(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetHostViaCustomSyntax", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLocalPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (serializationInfo, streamingContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParts(
        &mut self,
        uriParts: crate::System::UriComponents,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetParts", (uriParts, formatAs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRelativeSerializationString(
        &mut self,
        format: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetRelativeSerializationString", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnescapedParts(
        &mut self,
        uriParts: crate::System::UriComponents,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetUnescapedParts", (uriParts, formatAs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUriPartsFromUserString(
        &mut self,
        uriParts: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetUriPartsFromUserString", (uriParts))?;
        Ok(__cordl_ret.into())
    }
    pub fn InFact(
        &mut self,
        flags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InFact", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeUri(
        &mut self,
        err: crate::System::ParsingError,
        uriKind: crate::System::UriKind,
        e: quest_hook::libil2cpp::ByRefMut<*mut crate::System::UriFormatException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeUri", (err, uriKind, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeUriConfig() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitializeUriConfig", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalEscapeString(
        rawString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InternalEscapeString", (rawString))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsWellFormedOriginalString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalIsWellFormedOriginalString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IriParsingStatic(
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IriParsingStatic", (syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAsciiLetter(character: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAsciiLetter", (character))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAsciiLetterOrDigit(character: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAsciiLetterOrDigit", (character))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBaseOf(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBaseOf", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBaseOfHelper(
        &mut self,
        uriLink: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBaseOfHelper", (uriLink))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBidiControlCharacter(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsBidiControlCharacter", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenDelim(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenDelim", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHexDigit(character: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHexDigit", (character))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIntranet(
        &mut self,
        schemeHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsIntranet", (schemeHost))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLWS(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLWS", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWellFormedOriginalString(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsWellFormedOriginalString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWellFormedUriString(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        uriKind: crate::System::UriKind,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWellFormedUriString", (uriString, uriKind))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString0(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uriString))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_UriKind1(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        uriKind: crate::System::UriKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uriString, uriKind))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext4(
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Uri_Flags_UriParser_Il2CppString5(
        flags: crate::System::Uri_Flags,
        uriParser: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (flags, uriParser, uri))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Uri_Il2CppString2(
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Uri_Uri3(
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_object.into())
    }
    pub fn NotAny(
        &mut self,
        flags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NotAny", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseMinimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriFormatException> = __cordl_object
            .invoke("ParseMinimal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseRemaining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseRemaining", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseScheme(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        syntax: quest_hook::libil2cpp::ByRefMut<*mut crate::System::UriParser>,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParsingError> {
        let __cordl_ret: crate::System::ParsingError = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseScheme", (uriString, flags, syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseSchemeCheckImplicitFile(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: u16,
        err: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingError>,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        syntax: quest_hook::libil2cpp::ByRefMut<*mut crate::System::UriParser>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ParseSchemeCheckImplicitFile",
                (uriString, length, err, flags, syntax),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn PrivateParseMinimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParsingError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ParsingError = __cordl_object
            .invoke("PrivateParseMinimal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PrivateParseMinimalIri(
        &mut self,
        newHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrivateParseMinimalIri", (newHost, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReCreateParts(
        &mut self,
        parts: crate::System::UriComponents,
        nonCanonical: u16,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReCreateParts", (parts, nonCanonical, formatAs))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveHelper(
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        newUriString: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        userEscaped: quest_hook::libil2cpp::ByRefMut<bool>,
        e: quest_hook::libil2cpp::ByRefMut<*mut crate::System::UriFormatException>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ResolveHelper",
                (baseUri, relativeUri, newUriString, userEscaped, e),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUserDrivenParsing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUserDrivenParsing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StaticInFact(
        allFlags: crate::System::Uri_Flags,
        checkFlags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StaticInFact", (allFlags, checkFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn StaticIsFile(
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StaticIsFile", (syntax))?;
        Ok(__cordl_ret.into())
    }
    pub fn StaticNotAny(
        allFlags: crate::System::Uri_Flags,
        checkFlags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StaticNotAny", (allFlags, checkFlags))?;
        Ok(__cordl_ret.into())
    }
    pub fn StripBidiControlCharacter(
        strToClean: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StripBidiControlCharacter", (strToClean, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (serializationInfo, streamingContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCreate_Il2CppString_UriKind0(
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        uriKind: crate::System::UriKind,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCreate", (uriString, uriKind, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCreate_Uri_Il2CppString1(
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCreate", (baseUri, relativeUri, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCreate_Uri_Uri2(
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCreate", (baseUri, relativeUri, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnescapeDataString(
        stringToUnescape: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnescapeDataString", (stringToUnescape))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnescapeOnly(
        pch: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: quest_hook::libil2cpp::ByRefMut<i32>,
        ch1: char,
        ch2: char,
        ch3: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UnescapeOnly", (pch, start, end, ch1, ch2, ch3))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uriString))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_UriKind1(
        &mut self,
        uriString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        uriKind: crate::System::UriKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uriString, uriKind))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext4(
        &mut self,
        serializationInfo: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Uri_Flags_UriParser_Il2CppString5(
        &mut self,
        flags: crate::System::Uri_Flags,
        uriParser: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (flags, uriParser, uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Uri_Il2CppString2(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Uri_Uri3(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AbsolutePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AbsolutePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AbsoluteUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AbsoluteUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllowIdn(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowIdn", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Authority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Authority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DnsSafeHost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DnsSafeHost", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Fragment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Fragment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasAuthority(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasAuthority", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Host(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Host", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HostNameType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::UriHostNameType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::UriHostNameType = __cordl_object
            .invoke("get_HostNameType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HostType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Uri_Flags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Uri_Flags = __cordl_object
            .invoke("get_HostType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InitializeLock() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_InitializeLock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAbsoluteUri(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAbsoluteUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDefaultPort(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefaultPort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDosPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDosPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsImplicitFile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsImplicitFile", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsLoopback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsLoopback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsNotAbsoluteUri(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNotAbsoluteUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUnc(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUnc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUncOrDosPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUncOrDosPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUncPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUncPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_LocalPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OriginalString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_OriginalString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OriginalStringSwitched(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OriginalStringSwitched", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PathAndQuery(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_PathAndQuery", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Port(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Port", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrivateAbsolutePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_PrivateAbsolutePath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Query(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Query", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Scheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SecuredPathIndex(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("get_SecuredPathIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Segments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_Segments", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Syntax(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = __cordl_object
            .invoke("get_Syntax", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserDrivenParsing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UserDrivenParsing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserEscaped(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UserEscaped", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UserInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_UserInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        uri1: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        uri2: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (uri1, uri2))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        uri1: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        uri2: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (uri1, uri2))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Uri")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Uri {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Uri")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable> for crate::System::Uri {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Uri")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable> for crate::System::Uri {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Uri+Check")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Uri_Check {
    BackslashInPath = 16i32,
    DisplayCanonical = 2i32,
    DotSlashAttn = 4i32,
    DotSlashEscaped = 128i32,
    EscapedCanonical = 1i32,
    FoundNonAscii = 8i32,
    None = 0i32,
    NotIriCanonical = 64i32,
    ReservedFound = 32i32,
}
#[cfg(feature = "System+Uri+Check")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Uri_Check => "System"."Uri/Check"
);
#[cfg(feature = "System+Uri+Flags")]
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Uri_Flags {
    AllUriInfoSet = 2147483648u64,
    AuthorityFound = 1048576u64,
    BackslashInPath = 32768u64,
    BasicHostType = 327680u64,
    CannotDisplayCanonical = 127u64,
    CanonicalDnsHost = 33554432u64,
    CompressedSlashes = 17592186044416u64,
    DnsHostType = 196608u64,
    DosPath = 134217728u64,
    E_CannotDisplayCanonical = 8064u64,
    E_FragmentNotCanonical = 4096u64,
    E_HostNotCanonical = 256u64,
    E_PathNotCanonical = 1024u64,
    E_PortNotCanonical = 512u64,
    E_QueryNotCanonical = 2048u64,
    E_UserNotCanonical = 128u64,
    ErrorOrParsingRecursion = 67108864u64,
    FirstSlashAbsent = 16384u64,
    FragmentIriCanonical = 4398046511104u64,
    FragmentNotCanonical = 64u64,
    HasUnicode = 8589934592u64,
    HasUserInfo = 2097152u64,
    HostNotCanonical = 4u64,
    HostNotParsed = 0u64,
    HostTypeMask = 458752u64,
    HostUnicodeNormalized = 17179869184u64,
    IPv4HostType = 131072u64,
    IPv6HostType = 65536u64,
    IdnHost = 4294967296u64,
    ImplicitFile = 536870912u64,
    IndexMask = 65535u64,
    IntranetUri = 137438953472u64,
    IriCanonical = 8246337208320u64,
    LoopbackHost = 4194304u64,
    MinimalUriInfoSet = 1073741824u64,
    NotDefaultPort = 8388608u64,
    PathIriCanonical = 1099511627776u64,
    PathNotCanonical = 16u64,
    PortNotCanonical = 8u64,
    QueryIriCanonical = 2199023255552u64,
    QueryNotCanonical = 32u64,
    RestUnicodeNormalized = 34359738368u64,
    SchemeNotCanonical = 1u64,
    ShouldBeCompressed = 8192u64,
    UncHostType = 262144u64,
    UncPath = 268435456u64,
    UnicodeHost = 68719476736u64,
    UnusedHostType = 393216u64,
    UseOrigUncdStrOffset = 274877906944u64,
    UserDrivenParsing = 16777216u64,
    UserEscaped = 524288u64,
    UserIriCanonical = 549755813888u64,
    UserNotCanonical = 2u64,
}
#[cfg(feature = "System+Uri+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Uri_Flags => "System"."Uri/Flags"
);
#[cfg(feature = "System+Uri+MoreInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct Uri_MoreInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Query: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Fragment: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub AbsoluteUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Hash: i32,
    pub RemoteUrl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Uri+MoreInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Uri_MoreInfo => "System"."Uri/MoreInfo"
);
#[cfg(feature = "System+Uri+MoreInfo")]
impl std::ops::Deref for crate::System::Uri_MoreInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Uri+MoreInfo")]
impl std::ops::DerefMut for crate::System::Uri_MoreInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Uri+MoreInfo")]
impl crate::System::Uri_MoreInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Uri+MoreInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Uri_MoreInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Uri+Offset")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Uri_Offset {
    pub Scheme: u16,
    pub User: u16,
    pub Host: u16,
    pub PortValue: u16,
    pub Path: u16,
    pub Query: u16,
    pub Fragment: u16,
    pub End: u16,
}
#[cfg(feature = "System+Uri+Offset")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Uri_Offset => "System"."Uri/Offset"
);
#[cfg(feature = "System+Uri+Offset")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Uri_Offset {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Uri+Offset")]
impl crate::System::Uri_Offset {}
#[cfg(feature = "System+Uri+UriInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct Uri_UriInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Host: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ScopeId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub String: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Offset: crate::System::Uri_Offset,
    pub DnsSafeHost: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub MoreInfo: quest_hook::libil2cpp::Gc<crate::System::Uri_MoreInfo>,
}
#[cfg(feature = "System+Uri+UriInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Uri_UriInfo => "System"."Uri/UriInfo"
);
#[cfg(feature = "System+Uri+UriInfo")]
impl std::ops::Deref for crate::System::Uri_UriInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Uri+UriInfo")]
impl std::ops::DerefMut for crate::System::Uri_UriInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Uri+UriInfo")]
impl crate::System::Uri_UriInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Uri+UriInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Uri_UriInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
