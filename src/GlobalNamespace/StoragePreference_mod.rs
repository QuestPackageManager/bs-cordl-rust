#[cfg(feature = "StoragePreference")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoragePreference {
    Cloud = 0i32,
    Local = 1i32,
}
#[cfg(feature = "StoragePreference")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StoragePreference => ""
    ."StoragePreference"
);
