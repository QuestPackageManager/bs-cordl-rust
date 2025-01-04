#[cfg(feature = "System+Security+AccessControl+AceQualifier")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AceQualifier {
    #[default]
    AccessAllowed = 0i32,
    AccessDenied = 1i32,
    SystemAlarm = 3i32,
    SystemAudit = 2i32,
}
#[cfg(feature = "System+Security+AccessControl+AceQualifier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::AceQualifier =>
    "System.Security.AccessControl"."AceQualifier"
);
