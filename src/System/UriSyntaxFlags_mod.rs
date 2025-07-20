#[cfg(feature = "System+UriSyntaxFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UriSyntaxFlags {
    #[default]
    AllowAnInternetHost = 3584i32,
    AllowAnyOtherHost = 4096i32,
    AllowDOSPath = 1048576i32,
    AllowDnsHost = 512i32,
    AllowEmptyHost = 128i32,
    AllowIPv4Host = 1024i32,
    AllowIPv6Host = 2048i32,
    AllowIdn = 67108864i32,
    AllowIriParsing = 268435456i32,
    AllowUncHost = 256i32,
    BuiltInSyntax = 262144i32,
    CanonicalizeAsFilePath = 16777216i32,
    CompressPath = 8388608i32,
    ConvertPathSlashes = 4194304i32,
    FileLikeUri = 8192i32,
    MailToLikeUri = 16384i32,
    MayHaveFragment = 64i32,
    MayHavePath = 16i32,
    MayHavePort = 8i32,
    MayHaveQuery = 32i32,
    MayHaveUserInfo = 4i32,
    MustHaveAuthority = 1i32,
    None = 0i32,
    OptionalAuthority = 2i32,
    ParserSchemeOnly = 524288i32,
    PathIsRooted = 2097152i32,
    SimpleUserSyntax = 131072i32,
    UnEscapeDotsAndSlashes = 33554432i32,
    V1_UnknownUri = 65536i32,
}
#[cfg(feature = "System+UriSyntaxFlags")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::UriSyntaxFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "UriSyntaxFlags";
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
#[cfg(feature = "System+UriSyntaxFlags")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::UriSyntaxFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+UriSyntaxFlags")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::UriSyntaxFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "System+UriSyntaxFlags")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::UriSyntaxFlags {
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
#[cfg(feature = "System+UriSyntaxFlags")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::UriSyntaxFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
