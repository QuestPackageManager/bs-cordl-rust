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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UriSyntaxFlags => "System"
    ."UriSyntaxFlags"
);
