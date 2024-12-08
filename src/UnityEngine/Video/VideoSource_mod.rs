#[cfg(feature = "UnityEngine+Video+VideoSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoSource {
    Url = 1i32,
    VideoClip = 0i32,
}
#[cfg(feature = "UnityEngine+Video+VideoSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::VideoSource =>
    "UnityEngine.Video"."VideoSource"
);
