#[cfg(feature = "UnityEngine+Video+VideoAudioOutputMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VideoAudioOutputMode {
    #[default]
    APIOnly = 3i32,
    AudioSource = 1i32,
    Direct = 2i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Video+VideoAudioOutputMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::VideoAudioOutputMode =>
    "UnityEngine.Video"."VideoAudioOutputMode"
);
