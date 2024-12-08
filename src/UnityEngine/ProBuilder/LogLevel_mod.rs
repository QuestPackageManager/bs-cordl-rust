#[cfg(feature = "UnityEngine+ProBuilder+LogLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    All = 255i32,
    Default = 3i32,
    Error = 1i32,
    Info = 4i32,
    None = 0i32,
    Warning = 2i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+LogLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::LogLevel =>
    "UnityEngine.ProBuilder"."LogLevel"
);
