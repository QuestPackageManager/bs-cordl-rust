#[cfg(feature = "System+Net+AuthenticationSchemes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationSchemes {
    Anonymous = 32768i32,
    Basic = 8i32,
    Digest = 1i32,
    IntegratedWindowsAuthentication = 6i32,
    Negotiate = 2i32,
    None = 0i32,
    Ntlm = 4i32,
}
#[cfg(feature = "System+Net+AuthenticationSchemes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::AuthenticationSchemes =>
    "System.Net"."AuthenticationSchemes"
);
