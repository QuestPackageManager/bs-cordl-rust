#[cfg(feature = "UnityEngine+AudioClipLoadType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioClipLoadType {
    CompressedInMemory = 1i32,
    DecompressOnLoad = 0i32,
    Streaming = 2i32,
}
#[cfg(feature = "UnityEngine+AudioClipLoadType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioClipLoadType => "UnityEngine"
    ."AudioClipLoadType"
);
