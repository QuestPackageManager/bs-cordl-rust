#[cfg(feature = "System+Security+Permissions+SecurityAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityAction {
    _cordl_Assert = 3i32,
    Demand = 2i32,
    Deny = 4i32,
    InheritanceDemand = 7i32,
    LinkDemand = 6i32,
    PermitOnly = 5i32,
    RequestMinimum = 8i32,
    RequestOptional = 9i32,
    RequestRefuse = 10i32,
}
#[cfg(feature = "System+Security+Permissions+SecurityAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Permissions::SecurityAction =>
    "System.Security.Permissions"."SecurityAction"
);
