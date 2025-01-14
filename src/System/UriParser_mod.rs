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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::UriParser>,
                1usize,
            >("FindOrFetchAsUnknownV1Syntax")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FindOrFetchAsUnknownV1Syntax", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = unsafe {
            method.invoke_unchecked((), (lwrCaseScheme))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    crate::System::UriComponents,
                    crate::System::UriFormat,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("GetComponents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetComponents", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (uri, components, format)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSyntax(
        lwrCaseScheme: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::System::UriParser>,
                1usize,
            >("GetSyntax")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSyntax", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = unsafe {
            method.invoke_unchecked((), (lwrCaseScheme))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InFact(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::UriSyntaxFlags), bool, 1usize>("InFact")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InFact", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (flags)) };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeAndValidate(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("InitializeAndValidate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InitializeAndValidate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (uri, parsingError))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    crate::System::UriComponents,
                    crate::System::UriFormat,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("InternalGetComponents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalGetComponents", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method.invoke_unchecked(self, (thisUri, uriComponents, uriFormat))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsBaseOf(
        &mut self,
        thisBaseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        uriLink: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                ),
                bool,
                2usize,
            >("InternalIsBaseOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalIsBaseOf", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (thisBaseUri, uriLink))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalIsWellFormedOriginalString(
        &mut self,
        thisUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                bool,
                1usize,
            >("InternalIsWellFormedOriginalString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalIsWellFormedOriginalString", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (thisUri)) };
        Ok(__cordl_ret.into())
    }
    pub fn InternalOnNewUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::UriParser>,
                0usize,
            >("InternalOnNewUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalOnNewUri", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("InternalResolve")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalResolve", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method.invoke_unchecked(self, (thisBaseUri, uriLink, parsingError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalValidate(
        &mut self,
        thisUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        parsingError: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("InternalValidate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InternalValidate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (thisUri, parsingError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAllSet(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::UriSyntaxFlags), bool, 1usize>("IsAllSet")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsAllSet", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (flags)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsBaseOf(
        &mut self,
        baseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        relativeUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                ),
                bool,
                2usize,
            >("IsBaseOf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsBaseOf", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (baseUri, relativeUri))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsFullMatch(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
        expected: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::UriSyntaxFlags, crate::System::UriSyntaxFlags),
                bool,
                2usize,
            >("IsFullMatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsFullMatch", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (flags, expected))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsWellFormedOriginalString(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                bool,
                1usize,
            >("IsWellFormedOriginalString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsWellFormedOriginalString", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (uri)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::System::UriSyntaxFlags), bool, 1usize>("NotAny")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "NotAny", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (flags)) };
        Ok(__cordl_ret.into())
    }
    pub fn OnNewUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::UriParser>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::UriParser>,
                0usize,
            >("OnNewUri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnNewUri", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::UriParser> = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::Gc<crate::System::Uri>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<crate::System::UriFormatException>,
                    >,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                3usize,
            >("Resolve")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Resolve", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe {
            method.invoke_unchecked(self, (baseUri, relativeUri, parsingError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        flags: crate::System::UriSyntaxFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::UriSyntaxFlags),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (flags))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultPort(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_DefaultPort")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_DefaultPort", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_Flags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::UriSyntaxFlags> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::UriSyntaxFlags, 0usize>("get_Flags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Flags", 0usize
                )
            });
        let __cordl_ret: crate::System::UriSyntaxFlags = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSimple(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsSimple")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSimple", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_SchemeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_SchemeName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_SchemeName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_ShouldUseLegacyV2Quirks() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_ShouldUseLegacyV2Quirks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_ShouldUseLegacyV2Quirks", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
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
    const CLASS_NAME: &'static str = "UriParser/UriQuirksVersion";
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
#[cfg(feature = "System+UriParser+UriQuirksVersion")]
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
#[cfg(feature = "System+UriParser+UriQuirksVersion")]
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
#[cfg(feature = "System+UriParser+UriQuirksVersion")]
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
#[cfg(feature = "System+UriParser+UriQuirksVersion")]
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
