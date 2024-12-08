#[cfg(feature = "System+Security+AccessControl+ObjectAceFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectAceFlags {
    InheritedObjectAceTypePresent = 2i32,
    None = 0i32,
    ObjectAceTypePresent = 1i32,
}
#[cfg(feature = "System+Security+AccessControl+ObjectAceFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::AccessControl::ObjectAceFlags
    => "System.Security.AccessControl"."ObjectAceFlags"
);
