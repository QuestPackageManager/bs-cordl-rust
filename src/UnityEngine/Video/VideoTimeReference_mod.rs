#[cfg(feature = "UnityEngine+Video+VideoTimeReference")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VideoTimeReference {
    #[default]
    ExternalTime = 2i32,
    Freerun = 0i32,
    InternalTime = 1i32,
}
#[cfg(feature = "UnityEngine+Video+VideoTimeReference")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Video::VideoTimeReference =>
    "UnityEngine.Video"."VideoTimeReference"
);
