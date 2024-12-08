#[cfg(feature = "UnityEngine+Yoga+YogaLogLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YogaLogLevel {
    Debug = 3i32,
    Error = 0i32,
    Fatal = 5i32,
    Info = 2i32,
    Verbose = 4i32,
    Warn = 1i32,
}
#[cfg(feature = "UnityEngine+Yoga+YogaLogLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::YogaLogLevel =>
    "UnityEngine.Yoga"."YogaLogLevel"
);
