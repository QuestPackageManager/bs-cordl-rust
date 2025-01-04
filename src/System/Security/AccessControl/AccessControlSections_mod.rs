#[cfg(feature = "System+Security+AccessControl+AccessControlSections")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccessControlSections {
    #[default]
    Access = 2i32,
    All = 15i32,
    Audit = 1i32,
    Group = 8i32,
    None = 0i32,
    Owner = 4i32,
}
#[cfg(feature = "System+Security+AccessControl+AccessControlSections")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::AccessControlSections =>
    "System.Security.AccessControl"."AccessControlSections"
);
