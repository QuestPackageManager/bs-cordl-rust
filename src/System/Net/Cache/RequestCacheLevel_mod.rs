#[cfg(feature = "System+Net+Cache+RequestCacheLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestCacheLevel {
    BypassCache = 1i32,
    CacheIfAvailable = 3i32,
    CacheOnly = 2i32,
    Default = 0i32,
    NoCacheNoStore = 6i32,
    Reload = 5i32,
    Revalidate = 4i32,
}
#[cfg(feature = "System+Net+Cache+RequestCacheLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Cache::RequestCacheLevel =>
    "System.Net.Cache"."RequestCacheLevel"
);
