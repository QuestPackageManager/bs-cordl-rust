#[cfg(feature = "EntitlementsStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementsStatus {
    NotDownloaded = 2i32,
    NotOwned = 1i32,
    _cordl_Ok = 3i32,
    Unknown = 0i32,
}
#[cfg(feature = "EntitlementsStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for EntitlementsStatus => ""."EntitlementsStatus"
);
