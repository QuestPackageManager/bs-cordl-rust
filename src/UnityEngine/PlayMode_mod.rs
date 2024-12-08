#[cfg(feature = "UnityEngine+PlayMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    StopAll = 4i32,
    StopSameLayer = 0i32,
}
#[cfg(feature = "UnityEngine+PlayMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PlayMode => "UnityEngine"
    ."PlayMode"
);
