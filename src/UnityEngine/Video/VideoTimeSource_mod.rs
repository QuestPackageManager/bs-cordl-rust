#[cfg(feature = "UnityEngine+Video+VideoTimeSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoTimeSource {
    AudioDSPTimeSource = 0i32,
    GameTimeSource = 1i32,
}
#[cfg(feature = "UnityEngine+Video+VideoTimeSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::VideoTimeSource =>
    "UnityEngine.Video"."VideoTimeSource"
);
