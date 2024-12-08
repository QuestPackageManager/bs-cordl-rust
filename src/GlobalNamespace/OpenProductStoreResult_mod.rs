#[cfg(feature = "OpenProductStoreResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenProductStoreResult {
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "OpenProductStoreResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OpenProductStoreResult => ""
    ."OpenProductStoreResult"
);
