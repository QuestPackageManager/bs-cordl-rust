#[cfg(feature = "System+Security+AccessControl+PropagationFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropagationFlags {
    InheritOnly = 2i32,
    NoPropagateInherit = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Security+AccessControl+PropagationFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::PropagationFlags =>
    "System.Security.AccessControl"."PropagationFlags"
);
