#[cfg(feature = "UnityEngine+ApplicationSandboxType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationSandboxType {
    NotSandboxed = 1i32,
    SandboxBroken = 3i32,
    Sandboxed = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "UnityEngine+ApplicationSandboxType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ApplicationSandboxType =>
    "UnityEngine"."ApplicationSandboxType"
);
