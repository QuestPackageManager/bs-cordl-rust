#[cfg(feature = "UnityEngine+LogType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogType {
    _cordl_Assert = 1i32,
    Error = 0i32,
    Exception = 4i32,
    Log = 3i32,
    Warning = 2i32,
}
#[cfg(feature = "UnityEngine+LogType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LogType => "UnityEngine"."LogType"
);
