#[cfg(feature = "UnityEngine+Android+AndroidAssetPackStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidAssetPackStatus {
    Canceled = 6i32,
    Completed = 4i32,
    Downloading = 2i32,
    Failed = 5i32,
    NotInstalled = 8i32,
    Pending = 1i32,
    Transferring = 3i32,
    Unknown = 0i32,
    WaitingForWifi = 7i32,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidAssetPackStatus =>
    "UnityEngine.Android"."AndroidAssetPackStatus"
);
