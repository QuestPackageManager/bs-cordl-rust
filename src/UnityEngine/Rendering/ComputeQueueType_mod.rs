#[cfg(feature = "UnityEngine+Rendering+ComputeQueueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeQueueType {
    Background = 1i32,
    Default = 0i32,
    Urgent = 2i32,
}
#[cfg(feature = "UnityEngine+Rendering+ComputeQueueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::ComputeQueueType =>
    "UnityEngine.Rendering"."ComputeQueueType"
);
