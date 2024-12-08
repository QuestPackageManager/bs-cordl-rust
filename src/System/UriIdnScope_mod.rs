#[cfg(feature = "System+UriIdnScope")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UriIdnScope {
    All = 2i32,
    AllExceptIntranet = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+UriIdnScope")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::UriIdnScope => "System"."UriIdnScope"
);
