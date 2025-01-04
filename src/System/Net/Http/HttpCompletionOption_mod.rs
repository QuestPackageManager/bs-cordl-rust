#[cfg(feature = "System+Net+Http+HttpCompletionOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpCompletionOption {
    #[default]
    ResponseContentRead = 0i32,
    ResponseHeadersRead = 1i32,
}
#[cfg(feature = "System+Net+Http+HttpCompletionOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::HttpCompletionOption =>
    "System.Net.Http"."HttpCompletionOption"
);
