#[cfg(feature = "System+Security+Principal+TokenImpersonationLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TokenImpersonationLevel {
    #[default]
    Anonymous = 1i32,
    Delegation = 4i32,
    Identification = 2i32,
    Impersonation = 3i32,
    None = 0i32,
}
#[cfg(feature = "System+Security+Principal+TokenImpersonationLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Principal::TokenImpersonationLevel => "System.Security.Principal"
    ."TokenImpersonationLevel"
);
