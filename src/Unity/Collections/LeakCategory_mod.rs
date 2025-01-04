#[cfg(feature = "Unity+Collections+LeakCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeakCategory {
    #[default]
    Invalid = 0i32,
    LightProbesQuery = 4i32,
    Malloc = 1i32,
    MeshDataArray = 6i32,
    NativeTest = 5i32,
    NavMeshQuery = 8i32,
    Persistent = 3i32,
    TempJob = 2i32,
    TransformAccessArray = 7i32,
}
#[cfg(feature = "Unity+Collections+LeakCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::LeakCategory =>
    "Unity.Collections"."LeakCategory"
);
