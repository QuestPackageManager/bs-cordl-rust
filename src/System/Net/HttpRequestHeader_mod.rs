#[cfg(feature = "System+Net+HttpRequestHeader")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpRequestHeader {
    #[default]
    Accept = 20i32,
    AcceptCharset = 21i32,
    AcceptEncoding = 22i32,
    AcceptLanguage = 23i32,
    Allow = 10i32,
    Authorization = 24i32,
    CacheControl = 0i32,
    Connection = 1i32,
    ContentEncoding = 13i32,
    ContentLanguage = 14i32,
    ContentLength = 11i32,
    ContentLocation = 15i32,
    ContentMd5 = 16i32,
    ContentRange = 17i32,
    ContentType = 12i32,
    Cookie = 25i32,
    Date = 2i32,
    Expect = 26i32,
    Expires = 18i32,
    From = 27i32,
    Host = 28i32,
    IfMatch = 29i32,
    IfModifiedSince = 30i32,
    IfNoneMatch = 31i32,
    IfRange = 32i32,
    IfUnmodifiedSince = 33i32,
    KeepAlive = 3i32,
    LastModified = 19i32,
    MaxForwards = 34i32,
    Pragma = 4i32,
    ProxyAuthorization = 35i32,
    Range = 37i32,
    Referer = 36i32,
    Te = 38i32,
    Trailer = 5i32,
    TransferEncoding = 6i32,
    Translate = 39i32,
    Upgrade = 7i32,
    UserAgent = 40i32,
    Via = 8i32,
    Warning = 9i32,
}
#[cfg(feature = "System+Net+HttpRequestHeader")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpRequestHeader => "System.Net"
    ."HttpRequestHeader"
);
