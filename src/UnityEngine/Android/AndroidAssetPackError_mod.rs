#[cfg(feature = "UnityEngine+Android+AndroidAssetPackError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidAssetPackError {
    AccessDenied = -7i32,
    ApiNotAvailable = -5i32,
    AppNotOwned = -13i32,
    AppUnavailable = -1i32,
    DownloadNotFound = -4i32,
    InsufficientStorage = -10i32,
    InternalError = -100i32,
    InvalidRequest = -3i32,
    NetworkError = -6i32,
    NetworkUnrestricted = -12i32,
    NoError = 0i32,
    PackUnavailable = -2i32,
    PlayStoreNotFound = -11i32,
}
#[cfg(feature = "UnityEngine+Android+AndroidAssetPackError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidAssetPackError =>
    "UnityEngine.Android"."AndroidAssetPackError"
);
