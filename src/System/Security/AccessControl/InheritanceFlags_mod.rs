#[cfg(feature = "System+Security+AccessControl+InheritanceFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InheritanceFlags {
    #[default]
    ContainerInherit = 1i32,
    None = 0i32,
    ObjectInherit = 2i32,
}
#[cfg(feature = "System+Security+AccessControl+InheritanceFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::InheritanceFlags =>
    "System.Security.AccessControl"."InheritanceFlags"
);
