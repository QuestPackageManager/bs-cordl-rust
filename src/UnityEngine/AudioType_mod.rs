#[cfg(feature = "UnityEngine+AudioType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudioType {
    #[default]
    ACC = 1i32,
    AIFF = 2i32,
    AUDIOQUEUE = 24i32,
    IT = 10i32,
    _cordl_MOD = 12i32,
    MPEG = 13i32,
    OGGVORBIS = 14i32,
    S3M = 17i32,
    UNKNOWN = 0i32,
    VAG = 23i32,
    WAV = 20i32,
    XM = 21i32,
    XMA = 22i32,
}
#[cfg(feature = "UnityEngine+AudioType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioType => "UnityEngine"
    ."AudioType"
);
