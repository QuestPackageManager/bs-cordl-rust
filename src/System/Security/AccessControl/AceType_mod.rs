#[cfg(feature = "System+Security+AccessControl+AceType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AceType {
    #[default]
    AccessAllowed = 0u8,
    AccessAllowedCallback = 9u8,
    AccessAllowedCallbackObject = 11u8,
    AccessAllowedCompound = 4u8,
    AccessAllowedObject = 5u8,
    AccessDenied = 1u8,
    AccessDeniedCallback = 10u8,
    AccessDeniedCallbackObject = 12u8,
    AccessDeniedObject = 6u8,
    MaxDefinedAceType = 16u8,
    SystemAlarm = 3u8,
    SystemAlarmCallback = 14u8,
    SystemAlarmObject = 8u8,
    SystemAudit = 2u8,
    SystemAuditCallback = 13u8,
    SystemAuditCallbackObject = 15u8,
    SystemAuditObject = 7u8,
}
#[cfg(feature = "System+Security+AccessControl+AceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::AceType =>
    "System.Security.AccessControl"."AceType"
);
