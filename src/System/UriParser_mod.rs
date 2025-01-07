#[cfg(feature = "System+UriParser")]
#[repr(C)]
#[derive(Debug)]
pub struct UriParser {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Flags: crate::System::UriSyntaxFlags,
    pub m_UpdatableFlags: crate::System::UriSyntaxFlags,
    pub m_UpdatableFlagsUsed: bool,
    pub m_Port: i32,
    pub m_Scheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+UriParser")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::UriParser {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "UriParser";
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
    pub fn FindOrFetchAsUnknownV1Syntax(
        lwrCaseScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindOrFetchAsUnknownV1Syntax", (lwrCaseScheme))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetComponents(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
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
        > = __cordl_object.invoke("GetComponents", (uri, components, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSyntax(
        lwrCaseScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSyntax", (lwrCaseScheme))?;
        Ok(__cordl_ret.into())
    }
    pub fn InFact(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InFact", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAndValidate(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeAndValidate", (uri, parsingError))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalGetComponents(
        &mut self,
        thisUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
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
        > = __cordl_object
            .invoke("InternalGetComponents", (thisUri, uriComponents, uriFormat))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsBaseOf(
        &mut self,
        thisBaseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        uriLink: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalIsBaseOf", (thisBaseUri, uriLink))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsWellFormedOriginalString(
        &mut self,
        thisUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalIsWellFormedOriginalString", (thisUri))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalOnNewUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = __cordl_object
            .invoke("InternalOnNewUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalResolve(
        &mut self,
        thisBaseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        uriLink: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object
            .invoke("InternalResolve", (thisBaseUri, uriLink, parsingError))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalValidate(
        &mut self,
        thisUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalValidate", (thisUri, parsingError))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAllSet(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsAllSet", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBaseOf(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBaseOf", (baseUri, relativeUri))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn IsWellFormedOriginalString(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsWellFormedOriginalString", (uri))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (flags))?;
        Ok(__cordl_object.into())
    }
    pub fn NotAny(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NotAny", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnNewUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = __cordl_object
            .invoke("OnNewUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Resolve(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Resolve", (baseUri, relativeUri, parsingError))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_DefaultPort", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::UriSyntaxFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::UriSyntaxFlags = __cordl_object
            .invoke("get_Flags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSimple(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSimple", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_SchemeName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldUseLegacyV2Quirks() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ShouldUseLegacyV2Quirks", ())?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UriParser_UriQuirksVersion {
    #[default]
    V2 = 2i32,
    V3 = 3i32,
}
#[cfg(feature = "System+UriParser+UriQuirksVersion")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::UriParser_UriQuirksVersion {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "UriQuirksVersion";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::UriParser_UriQuirksVersion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::UriParser_UriQuirksVersion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::UriParser_UriQuirksVersion {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::System::UriParser_UriQuirksVersion {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
