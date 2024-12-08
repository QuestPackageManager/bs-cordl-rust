#[cfg(feature = "System+Security+AccessControl+AceFlags")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AceFlags {
    AuditFlags = 192u8,
    ContainerInherit = 2u8,
    FailedAccess = 128u8,
    InheritOnly = 8u8,
    InheritanceFlags = 15u8,
    Inherited = 16u8,
    NoPropagateInherit = 4u8,
    None = 0u8,
    ObjectInherit = 1u8,
    SuccessfulAccess = 64u8,
}
#[cfg(feature = "System+Security+AccessControl+AceFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::AceFlags =>
    "System.Security.AccessControl"."AceFlags"
);
