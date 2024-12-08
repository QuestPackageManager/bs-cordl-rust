#[cfg(feature = "System+Net+Http+Headers+HttpHeaderKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpHeaderKind {
    Content = 4i32,
    None = 0i32,
    Request = 1i32,
    Response = 2i32,
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaderKind")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::HttpHeaderKind =>
    "System.Net.Http.Headers"."HttpHeaderKind"
);
