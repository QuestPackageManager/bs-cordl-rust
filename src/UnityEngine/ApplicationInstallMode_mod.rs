#[cfg(feature = "UnityEngine+ApplicationInstallMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationInstallMode {
    Adhoc = 3i32,
    DeveloperBuild = 2i32,
    Editor = 5i32,
    Enterprise = 4i32,
    Store = 1i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+ApplicationInstallMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ApplicationInstallMode =>
    "UnityEngine"."ApplicationInstallMode"
);
