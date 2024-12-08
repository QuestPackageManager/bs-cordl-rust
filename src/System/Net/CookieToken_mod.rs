#[cfg(feature = "System+Net+CookieToken")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookieToken {
    Attribute = 2i32,
    Comment = 7i32,
    CommentUrl = 8i32,
    CookieName = 9i32,
    Discard = 10i32,
    Domain = 11i32,
    End = 5i32,
    EndCookie = 4i32,
    EndToken = 3i32,
    Equals = 6i32,
    Expires = 12i32,
    HttpOnly = 17i32,
    MaxAge = 13i32,
    NameValuePair = 1i32,
    Nothing = 0i32,
    Path = 14i32,
    Port = 15i32,
    Secure = 16i32,
    Unknown = 18i32,
    Version = 19i32,
}
#[cfg(feature = "System+Net+CookieToken")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CookieToken => "System.Net"
    ."CookieToken"
);
