#[cfg(feature = "EntitlementStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EntitlementStatus {
    #[default]
    Failed = 0i32,
    NotOwned = 2i32,
    Owned = 1i32,
}
#[cfg(feature = "EntitlementStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EntitlementStatus => ""
    ."EntitlementStatus"
);
