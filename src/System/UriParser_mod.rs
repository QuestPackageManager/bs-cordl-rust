#[cfg(feature = "System+UriParser")]
#[repr(C)]
#[derive(Debug)]
pub struct UriParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Flags: crate::System::UriSyntaxFlags,
    pub m_UpdatableFlags: crate::System::UriSyntaxFlags,
    pub m_UpdatableFlagsUsed: bool,
    pub m_Port: i32,
    pub m_Scheme: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+UriParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::UriParser => "System"."UriParser"
);
#[cfg(feature = "System+UriParser")]
impl std::ops::Deref for crate::System::UriParser {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriParser")]
impl std::ops::DerefMut for crate::System::UriParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriParser")]
impl crate::System::UriParser {
    pub const NoDefaultPort: i32 = -1i32;
    pub const c_InitialTableSize: i32 = 25i32;
    pub const c_MaxCapacity: i32 = 512i32;
    #[cfg(feature = "System+UriParser+BuiltInUriParser")]
    pub type BuiltInUriParser = crate::GlobalNamespace::UriParser_BuiltInUriParser;
    #[cfg(feature = "System+UriParser+UriQuirksVersion")]
    pub type UriQuirksVersion = crate::System::UriParser_UriQuirksVersion;
    pub fn GetComponents(
        &mut self,
        uri: *mut crate::System::Uri,
        components: crate::System::UriComponents,
        format: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("GetComponents", (uri, components, format))?;
        Ok(__cordl_ret)
    }
    pub fn InFact(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InFact", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn InitializeAndValidate(
        &mut self,
        uri: *mut crate::System::Uri,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::UriFormatException,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeAndValidate", (uri, parsingError))?;
        Ok(__cordl_ret)
    }
    pub fn InternalGetComponents(
        &mut self,
        thisUri: *mut crate::System::Uri,
        uriComponents: crate::System::UriComponents,
        uriFormat: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("InternalGetComponents", (thisUri, uriComponents, uriFormat))?;
        Ok(__cordl_ret)
    }
    pub fn InternalIsBaseOf(
        &mut self,
        thisBaseUri: *mut crate::System::Uri,
        uriLink: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalIsBaseOf", (thisBaseUri, uriLink))?;
        Ok(__cordl_ret)
    }
    pub fn InternalIsWellFormedOriginalString(
        &mut self,
        thisUri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalIsWellFormedOriginalString", (thisUri))?;
        Ok(__cordl_ret)
    }
    pub fn InternalOnNewUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::UriParser> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::UriParser = __cordl_object
            .invoke("InternalOnNewUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalResolve(
        &mut self,
        thisBaseUri: *mut crate::System::Uri,
        uriLink: *mut crate::System::Uri,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::UriFormatException,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("InternalResolve", (thisBaseUri, uriLink, parsingError))?;
        Ok(__cordl_ret)
    }
    pub fn InternalValidate(
        &mut self,
        thisUri: *mut crate::System::Uri,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::UriFormatException,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalValidate", (thisUri, parsingError))?;
        Ok(__cordl_ret)
    }
    pub fn IsAllSet(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAllSet", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn IsBaseOf(
        &mut self,
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBaseOf", (baseUri, relativeUri))?;
        Ok(__cordl_ret)
    }
    pub fn IsFullMatch(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
        expected: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsFullMatch", (flags, expected))?;
        Ok(__cordl_ret)
    }
    pub fn IsWellFormedOriginalString(
        &mut self,
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsWellFormedOriginalString", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (flags))?;
        Ok(__cordl_object)
    }
    pub fn NotAny(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NotAny", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn OnNewUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::UriParser> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::UriParser = __cordl_object
            .invoke("OnNewUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn Resolve(
        &mut self,
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::Uri,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::UriFormatException,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("Resolve", (baseUri, relativeUri, parsingError))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DefaultPort", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::UriSyntaxFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::UriSyntaxFlags = __cordl_object
            .invoke("get_Flags", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSimple(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSimple", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SchemeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_SchemeName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+UriParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::UriParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+UriParser+UriQuirksVersion")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UriParser_UriQuirksVersion {
    V2 = 2i32,
    V3 = 3i32,
}
#[cfg(feature = "System+UriParser+UriQuirksVersion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UriParser_UriQuirksVersion => "System"
    ."UriParser/UriQuirksVersion"
);
