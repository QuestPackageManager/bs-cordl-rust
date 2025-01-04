#[cfg(feature = "System+Security+AccessControl+AuditFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuditFlags {
    #[default]
    Failure = 2i32,
    None = 0i32,
    Success = 1i32,
}
#[cfg(feature = "System+Security+AccessControl+AuditFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::AuditFlags =>
    "System.Security.AccessControl"."AuditFlags"
);
