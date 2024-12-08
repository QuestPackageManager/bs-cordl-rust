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
    __cordl_parent: crate::System::Object,
    pub Path: *mut crate::System::String,
    pub Query: *mut crate::System::String,
    pub Fragment: *mut crate::System::String,
    pub AbsoluteUri: *mut crate::System::String,
    pub Hash: i32,
    pub RemoteUrl: *mut crate::System::String,
}
#[cfg(feature = "System+Uri+MoreInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Uri_MoreInfo => "System"."Uri/MoreInfo"
);
#[cfg(feature = "System+Uri+MoreInfo")]
impl std::ops::Deref for crate::System::Uri_MoreInfo {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[derive(Debug, Clone)]
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
#[cfg(feature = "System+Uri")]
#[repr(C)]
#[derive(Debug)]
pub struct Uri {
    __cordl_parent: crate::System::Object,
    pub m_String: *mut crate::System::String,
    pub m_originalUnicodeString: *mut crate::System::String,
    pub m_Syntax: *mut crate::System::UriParser,
    pub m_DnsSafeHost: *mut crate::System::String,
    pub m_Flags: crate::System::Uri_Flags,
    pub m_Info: *mut crate::System::Uri_UriInfo,
    pub m_iriParsing: bool,
}
#[cfg(feature = "System+Uri")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Uri => "System"."Uri"
);
#[cfg(feature = "System+Uri")]
impl std::ops::Deref for crate::System::Uri {
    type Target = crate::System::Object;
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
    pub const c_DummyChar: char = "\\u{ffff}";
    pub const c_EOL: char = "\\u{fffe}";
    pub const c_Max16BitUtf8SequenceLength: i32 = 12i32;
    pub const c_MaxUriBufferSize: i32 = 65520i32;
    pub const c_MaxUriSchemeName: i32 = 1024i32;
    #[cfg(feature = "System+Uri+Offset")]
    pub type Offset = crate::System::Uri_Offset;
    #[cfg(feature = "System+Uri+MoreInfo")]
    pub type MoreInfo = crate::System::Uri_MoreInfo;
    #[cfg(feature = "System+Uri+UriInfo")]
    pub type UriInfo = crate::System::Uri_UriInfo;
    #[cfg(feature = "System+Uri+Check")]
    pub type Check = crate::System::Uri_Check;
    #[cfg(feature = "System+Uri+Flags")]
    pub type Flags = crate::System::Uri_Flags;
    pub fn CreateHostString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateHostString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAbsoluteUri(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAbsoluteUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsUnc(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUnc", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserEscaped(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UserEscaped", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateAbsolutePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_PrivateAbsolutePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDefaultPort(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDefaultPort", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn CheckCanonical(
        &mut self,
        str: *mut quest_hook::libil2cpp::Il2CppObject,
        idx: quest_hook::libil2cpp::ByRefMut<u16>,
        end: u16,
        delim: char,
    ) -> quest_hook::libil2cpp::Result<crate::System::Uri_Check> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Uri_Check = __cordl_object
            .invoke("CheckCanonical", (str, idx, end, delim))?;
        Ok(__cordl_ret)
    }
    pub fn IsIntranet(
        &mut self,
        schemeHost: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsIntranet", (schemeHost))?;
        Ok(__cordl_ret)
    }
    pub fn get_Scheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Scheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OriginalString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_OriginalString", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckForEscapedUnreserved(
        &mut self,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckForEscapedUnreserved", (data))?;
        Ok(__cordl_ret)
    }
    pub fn ParseMinimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::UriFormatException> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::UriFormatException = __cordl_object
            .invoke("ParseMinimal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserDrivenParsing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UserDrivenParsing", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsBaseOf(
        &mut self,
        uri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBaseOf", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn GetParts(
        &mut self,
        uriParts: crate::System::UriComponents,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetParts", (uriParts, formatAs))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForConfigLoad(
        &mut self,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckForConfigLoad", (data))?;
        Ok(__cordl_ret)
    }
    pub fn InternalIsWellFormedOriginalString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalIsWellFormedOriginalString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DnsSafeHost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DnsSafeHost", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SecuredPathIndex(&mut self) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("get_SecuredPathIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckAuthorityHelper(
        &mut self,
        pString: *mut quest_hook::libil2cpp::Il2CppObject,
        idx: u16,
        length: u16,
        err: quest_hook::libil2cpp::ByRefMut<crate::System::ParsingError>,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        syntax: *mut crate::System::UriParser,
        newHost: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object
            .invoke(
                "CheckAuthorityHelper",
                (pString, idx, length, err, flags, syntax, newHost),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Segments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Segments", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRelativeSerializationString(
        &mut self,
        format: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetRelativeSerializationString", (format))?;
        Ok(__cordl_ret)
    }
    pub fn NotAny(
        &mut self,
        flags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("NotAny", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsNotAbsoluteUri(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsNotAbsoluteUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReCreateParts(
        &mut self,
        parts: crate::System::UriComponents,
        nonCanonical: u16,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReCreateParts", (parts, nonCanonical, formatAs))?;
        Ok(__cordl_ret)
    }
    pub fn get_Fragment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Fragment", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetComponentsHelper(
        &mut self,
        uriComponents: crate::System::UriComponents,
        uriFormat: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetComponentsHelper", (uriComponents, uriFormat))?;
        Ok(__cordl_ret)
    }
    pub fn InFact(
        &mut self,
        flags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InFact", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn get_Query(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Query", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsBaseOfHelper(
        &mut self,
        uriLink: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsBaseOfHelper", (uriLink))?;
        Ok(__cordl_ret)
    }
    pub fn IsWellFormedOriginalString(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsWellFormedOriginalString", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetComponents(
        &mut self,
        components: crate::System::UriComponents,
        format: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetComponents", (components, format))?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
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
        Ok(__cordl_ret)
    }
    pub fn get_IsLoopback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsLoopback", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_AbsoluteUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AbsoluteUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Syntax(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::UriParser> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::UriParser = __cordl_object
            .invoke("get_Syntax", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllowIdnStatic(
        &mut self,
        syntax: *mut crate::System::UriParser,
        flags: crate::System::Uri_Flags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AllowIdnStatic", (syntax, flags))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsDosPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDosPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        uriString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uriString))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_UriKind1(
        &mut self,
        uriString: *mut crate::System::String,
        uriKind: crate::System::UriKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uriString, uriKind))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Uri_String2(
        &mut self,
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Uri_Uri3(
        &mut self,
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext4(
        &mut self,
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Uri_Flags_UriParser_String5(
        &mut self,
        flags: crate::System::Uri_Flags,
        uriParser: *mut crate::System::UriParser,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (flags, uriParser, uri))?;
        Ok(__cordl_ret)
    }
    pub fn get_AbsolutePath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AbsolutePath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Authority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Authority", ())?;
        Ok(__cordl_ret)
    }
    pub fn PrivateParseMinimalIri(
        &mut self,
        newHost: *mut crate::System::String,
        idx: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrivateParseMinimalIri", (newHost, idx))?;
        Ok(__cordl_ret)
    }
    pub fn CreateThisFromUri(
        &mut self,
        otherUri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateThisFromUri", (otherUri))?;
        Ok(__cordl_ret)
    }
    pub fn SetUserDrivenParsing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUserDrivenParsing", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsUncOrDosPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUncOrDosPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCanonicalPath(
        &mut self,
        dest: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = __cordl_object
            .invoke("GetCanonicalPath", (dest, pos, formatAs))?;
        Ok(__cordl_ret)
    }
    pub fn CheckForUnicode(
        &mut self,
        data: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CheckForUnicode", (data))?;
        Ok(__cordl_ret)
    }
    pub fn CheckAuthorityHelperHandleAnyHostIri(
        &mut self,
        pString: *mut quest_hook::libil2cpp::Il2CppObject,
        startInput: i32,
        end: i32,
        iriParsing: bool,
        hasUnicode: bool,
        syntax: *mut crate::System::UriParser,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        newHost: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
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
        Ok(__cordl_ret)
    }
    pub fn get_HostNameType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::UriHostNameType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::UriHostNameType = __cordl_object
            .invoke("get_HostNameType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Host(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Host", ())?;
        Ok(__cordl_ret)
    }
    pub fn Equals(
        &mut self,
        comparand: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (comparand))?;
        Ok(__cordl_ret)
    }
    pub fn GetLocalPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetLocalPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnsureUriInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri_UriInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri_UriInfo = __cordl_object
            .invoke("EnsureUriInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsFile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFile", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_LocalPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HostType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Uri_Flags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Uri_Flags = __cordl_object
            .invoke("get_HostType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PathAndQuery(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_PathAndQuery", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OriginalStringSwitched(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_OriginalStringSwitched", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsImplicitFile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsImplicitFile", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowIdn(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowIdn", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnsureParseRemaining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureParseRemaining", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (serializationInfo, streamingContext))?;
        Ok(__cordl_ret)
    }
    pub fn GetUnescapedParts(
        &mut self,
        uriParts: crate::System::UriComponents,
        formatAs: crate::System::UriFormat,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetUnescapedParts", (uriParts, formatAs))?;
        Ok(__cordl_ret)
    }
    pub fn GetUriPartsFromUserString(
        &mut self,
        uriParts: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetUriPartsFromUserString", (uriParts))?;
        Ok(__cordl_ret)
    }
    pub fn FindEndOfComponent_String0(
        &mut self,
        input: *mut crate::System::String,
        idx: quest_hook::libil2cpp::ByRefMut<u16>,
        end: u16,
        delim: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindEndOfComponent", (input, idx, end, delim))?;
        Ok(__cordl_ret)
    }
    pub fn FindEndOfComponent_Il2CppObject1(
        &mut self,
        str: *mut quest_hook::libil2cpp::Il2CppObject,
        idx: quest_hook::libil2cpp::ByRefMut<u16>,
        end: u16,
        delim: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FindEndOfComponent", (str, idx, end, delim))?;
        Ok(__cordl_ret)
    }
    pub fn GetEscapedParts(
        &mut self,
        uriParts: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetEscapedParts", (uriParts))?;
        Ok(__cordl_ret)
    }
    pub fn EscapeUnescapeIri(
        &mut self,
        input: *mut crate::System::String,
        start: i32,
        end: i32,
        component: crate::System::UriComponents,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("EscapeUnescapeIri", (input, start, end, component))?;
        Ok(__cordl_ret)
    }
    pub fn CreateUri(
        &mut self,
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::String,
        dontEscape: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateUri", (baseUri, relativeUri, dontEscape))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsUncPath(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsUncPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateThis(
        &mut self,
        uri: *mut crate::System::String,
        dontEscape: bool,
        uriKind: crate::System::UriKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateThis", (uri, dontEscape, uriKind))?;
        Ok(__cordl_ret)
    }
    pub fn CheckAuthorityHelperHandleDnsIri(
        &mut self,
        pString: *mut quest_hook::libil2cpp::Il2CppObject,
        start: u16,
        end: i32,
        startInput: i32,
        iriParsing: bool,
        hasUnicode: bool,
        syntax: *mut crate::System::UriParser,
        userInfoString: *mut crate::System::String,
        flags: quest_hook::libil2cpp::ByRefMut<crate::System::Uri_Flags>,
        justNormalized: quest_hook::libil2cpp::ByRefMut<bool>,
        newHost: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
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
        Ok(__cordl_ret)
    }
    pub fn PrivateParseMinimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ParsingError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ParsingError = __cordl_object
            .invoke("PrivateParseMinimal", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHostViaCustomSyntax(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetHostViaCustomSyntax", ())?;
        Ok(__cordl_ret)
    }
    pub fn ParseRemaining(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ParseRemaining", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UserInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_UserInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Port(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Port", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasAuthority(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasAuthority", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_String0(
        uriString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uriString))?;
        Ok(__cordl_object)
    }
    pub fn New_String_UriKind1(
        uriString: *mut crate::System::String,
        uriKind: crate::System::UriKind,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uriString, uriKind))?;
        Ok(__cordl_object)
    }
    pub fn New_Uri_String2(
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_object)
    }
    pub fn New_Uri_Uri3(
        baseUri: *mut crate::System::Uri,
        relativeUri: *mut crate::System::Uri,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (baseUri, relativeUri))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext4(
        serializationInfo: *mut crate::System::Runtime::Serialization::SerializationInfo,
        streamingContext: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializationInfo, streamingContext))?;
        Ok(__cordl_object)
    }
    pub fn New_Uri_Flags_UriParser_String5(
        flags: crate::System::Uri_Flags,
        uriParser: *mut crate::System::UriParser,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (flags, uriParser, uri))?;
        Ok(__cordl_object)
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
#[cfg(feature = "System+Uri+UriInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct Uri_UriInfo {
    __cordl_parent: crate::System::Object,
    pub Host: *mut crate::System::String,
    pub ScopeId: *mut crate::System::String,
    pub String: *mut crate::System::String,
    pub Offset: crate::System::Uri_Offset,
    pub DnsSafeHost: *mut crate::System::String,
    pub MoreInfo: *mut crate::System::Uri_MoreInfo,
}
#[cfg(feature = "System+Uri+UriInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Uri_UriInfo => "System"."Uri/UriInfo"
);
#[cfg(feature = "System+Uri+UriInfo")]
impl std::ops::Deref for crate::System::Uri_UriInfo {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
