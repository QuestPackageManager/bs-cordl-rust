#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncOperationStatus {
    Failed = 2i32,
    None = 0i32,
    Succeeded = 1i32,
}
#[cfg(feature = "UnityEngine+ResourceManagement+AsyncOperations+AsyncOperationStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::AsyncOperations::AsyncOperationStatus =>
    "UnityEngine.ResourceManagement.AsyncOperations"."AsyncOperationStatus"
);
