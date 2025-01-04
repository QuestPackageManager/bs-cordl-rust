#[cfg(feature = "UnityEngine+OperatingSystemFamily")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperatingSystemFamily {
    #[default]
    Linux = 3i32,
    MacOSX = 1i32,
    Other = 0i32,
    Windows = 2i32,
}
#[cfg(feature = "UnityEngine+OperatingSystemFamily")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::OperatingSystemFamily =>
    "UnityEngine"."OperatingSystemFamily"
);
