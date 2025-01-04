#[cfg(feature = "System+Security+Authentication+ExtendedProtection+PolicyEnforcement")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PolicyEnforcement {
    #[default]
    Always = 2i32,
    Never = 0i32,
    WhenSupported = 1i32,
}
#[cfg(feature = "System+Security+Authentication+ExtendedProtection+PolicyEnforcement")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Authentication::ExtendedProtection::PolicyEnforcement =>
    "System.Security.Authentication.ExtendedProtection"."PolicyEnforcement"
);
