#[cfg(feature = "UnityEngine+ApplicationMemoryUsage")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApplicationMemoryUsage {
    #[default]
    Critical = 4i32,
    High = 3i32,
    Low = 1i32,
    Medium = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+ApplicationMemoryUsage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ApplicationMemoryUsage =>
    "UnityEngine"."ApplicationMemoryUsage"
);
