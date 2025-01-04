#[cfg(feature = "System+Net+Security+AuthenticationLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthenticationLevel {
    #[default]
    MutualAuthRequested = 1i32,
    MutualAuthRequired = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Net+Security+AuthenticationLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Security::AuthenticationLevel =>
    "System.Net.Security"."AuthenticationLevel"
);
