#[cfg(feature = "System+UriComponents")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UriComponents {
    #[default]
    AbsoluteUri = 127i32,
    Fragment = 64i32,
    Host = 4i32,
    HostAndPort = 132i32,
    HttpRequestUrl = 61i32,
    KeepDelimiter = 1073741824i32,
    NormalizedHost = 256i32,
    Path = 16i32,
    PathAndQuery = 48i32,
    Port = 8i32,
    Query = 32i32,
    Scheme = 1i32,
    SchemeAndServer = 13i32,
    SerializationInfoString = -2147483648i32,
    StrongAuthority = 134i32,
    StrongPort = 128i32,
    UserInfo = 2i32,
}
#[cfg(feature = "System+UriComponents")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UriComponents => "System"
    ."UriComponents"
);
