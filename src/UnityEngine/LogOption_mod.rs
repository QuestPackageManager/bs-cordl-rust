#[cfg(feature = "UnityEngine+LogOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogOption {
    NoStacktrace = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+LogOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LogOption => "UnityEngine"
    ."LogOption"
);
