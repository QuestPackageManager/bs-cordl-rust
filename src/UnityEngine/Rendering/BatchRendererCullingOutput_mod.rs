#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BatchRendererCullingOutput {
    pub cullingJobsFence: crate::Unity::Jobs::JobHandle,
    pub localToWorldMatrix: crate::UnityEngine::Matrix4x4,
    pub cullingPlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cullingPlaneCount: i32,
    pub receiverPlaneOffset: i32,
    pub receiverPlaneCount: i32,
    pub cullingSplits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cullingSplitCount: i32,
    pub viewType: crate::UnityEngine::Rendering::BatchCullingViewType,
    pub projectionType: crate::UnityEngine::Rendering::BatchCullingProjectionType,
    pub cullingFlags: crate::UnityEngine::Rendering::BatchCullingFlags,
    pub viewID: u64,
    pub cullingLayerMask: u32,
    pub sceneCullingMask: u64,
    pub drawCommands: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::BatchRendererCullingOutput => "UnityEngine.Rendering"
    ."BatchRendererCullingOutput"
);
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchRendererCullingOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchRendererCullingOutput")]
impl crate::UnityEngine::Rendering::BatchRendererCullingOutput {}
