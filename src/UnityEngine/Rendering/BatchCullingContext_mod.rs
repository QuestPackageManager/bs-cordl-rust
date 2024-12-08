#[cfg(feature = "UnityEngine+Rendering+BatchCullingContext")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BatchCullingContext {
    pub cullingPlanes: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Plane,
    >,
    pub cullingSplits: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::CullingSplit,
    >,
    pub lodParameters: crate::UnityEngine::Rendering::LODParameters,
    pub localToWorldMatrix: crate::UnityEngine::Matrix4x4,
    pub viewType: crate::UnityEngine::Rendering::BatchCullingViewType,
    pub projectionType: crate::UnityEngine::Rendering::BatchCullingProjectionType,
    pub cullingFlags: crate::UnityEngine::Rendering::BatchCullingFlags,
    pub viewID: crate::UnityEngine::Rendering::BatchPackedCullingViewID,
    pub cullingLayerMask: u32,
    pub sceneCullingMask: u64,
    pub isOrthographic: u8,
    pub receiverPlaneOffset: i32,
    pub receiverPlaneCount: i32,
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingContext")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::BatchCullingContext =>
    "UnityEngine.Rendering"."BatchCullingContext"
);
#[cfg(feature = "UnityEngine+Rendering+BatchCullingContext")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::BatchCullingContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+BatchCullingContext")]
impl crate::UnityEngine::Rendering::BatchCullingContext {
    pub fn _ctor(
        &mut self,
        inCullingPlanes: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Plane,
        >,
        inCullingSplits: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::CullingSplit,
        >,
        inLodParameters: crate::UnityEngine::Rendering::LODParameters,
        inLocalToWorldMatrix: crate::UnityEngine::Matrix4x4,
        inViewType: crate::UnityEngine::Rendering::BatchCullingViewType,
        inProjectionType: crate::UnityEngine::Rendering::BatchCullingProjectionType,
        inBatchCullingFlags: crate::UnityEngine::Rendering::BatchCullingFlags,
        inViewID: u64,
        inCullingLayerMask: u32,
        inSceneCullingMask: u64,
        inReceiverPlaneOffset: i32,
        inReceiverPlaneCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                inCullingPlanes,
                inCullingSplits,
                inLodParameters,
                inLocalToWorldMatrix,
                inViewType,
                inProjectionType,
                inBatchCullingFlags,
                inViewID,
                inCullingLayerMask,
                inSceneCullingMask,
                inReceiverPlaneOffset,
                inReceiverPlaneCount,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
