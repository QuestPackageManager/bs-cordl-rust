#[cfg(feature = "UnityEngine+ThreadPriority")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadPriority {
    BelowNormal = 1i32,
    High = 4i32,
    Low = 0i32,
    Normal = 2i32,
}
#[cfg(feature = "UnityEngine+ThreadPriority")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ThreadPriority => "UnityEngine"
    ."ThreadPriority"
);
