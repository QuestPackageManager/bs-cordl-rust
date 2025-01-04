#[cfg(feature = "System+Security+AccessControl+AccessControlType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccessControlType {
    #[default]
    Allow = 0i32,
    Deny = 1i32,
}
#[cfg(feature = "System+Security+AccessControl+AccessControlType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::AccessControlType =>
    "System.Security.AccessControl"."AccessControlType"
);
