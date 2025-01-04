#[cfg(feature = "UnityEngine+QueueMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum QueueMode {
    #[default]
    CompleteOthers = 0i32,
    PlayNow = 2i32,
}
#[cfg(feature = "UnityEngine+QueueMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::QueueMode => "UnityEngine"
    ."QueueMode"
);
