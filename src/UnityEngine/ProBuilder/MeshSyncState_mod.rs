#[cfg(feature = "UnityEngine+ProBuilder+MeshSyncState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshSyncState {
    InSync = 3i32,
    InstanceIDMismatch = 1i32,
    Lightmap = 2i32,
    NeedsRebuild = 4i32,
    Null = 0i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshSyncState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshSyncState =>
    "UnityEngine.ProBuilder"."MeshSyncState"
);
