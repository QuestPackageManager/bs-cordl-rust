#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct InstanceDataSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_InstanceAllocators: crate::UnityEngine::Rendering::InstanceAllocators,
    pub m_SharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub m_InstanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub m_RendererGroupInstanceMultiHash: crate::Unity::Collections::NativeParallelMultiHashMap_2<
        i32,
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
    pub m_TransformUpdateCS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub m_WindDataUpdateCS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub m_TransformInitKernel: i32,
    pub m_TransformUpdateKernel: i32,
    pub m_MotionUpdateKernel: i32,
    pub m_ProbeUpdateKernel: i32,
    pub m_LODUpdateKernel: i32,
    pub m_WindDataCopyHistoryKernel: i32,
    pub m_UpdateIndexQueueBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_ProbeUpdateDataQueueBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_ProbeOcclusionUpdateDataQueueBuffer:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_TransformUpdateDataQueueBuffer:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_BoundingSpheresUpdateDataQueueBuffer:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_EnableBoundingSpheres: bool,
    pub m_ScratchWindParamAddressArray:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::InstanceDataSystem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem")]
impl std::ops::Deref for crate::UnityEngine::Rendering::InstanceDataSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::InstanceDataSystem {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem")]
impl crate::UnityEngine::Rendering::InstanceDataSystem {
    #[cfg(
        feature = "UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
    )]
    pub type CalculateInterpolatedLightAndOcclusionProbesBatchJob = crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob;
    #[cfg(
        feature = "UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
    )]
    pub type CollectInstancesLODGroupsAndMasksJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob;
    #[cfg(
        feature = "UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
    )]
    pub type ComputeInstancesOffsetAndResizeInstancesArrayJob = crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
    pub type FreeInstancesJob = crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob")]
    pub type FreeRendererGroupInstancesJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob;
    #[cfg(
        feature = "UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
    )]
    pub type GetVisibleNonProcessedTreeInstancesJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceTransformUpdateIDs")]
    pub type InstanceTransformUpdateIDs =
        crate::UnityEngine::Rendering::InstanceDataSystem_InstanceTransformUpdateIDs;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceWindDataUpdateIDs")]
    pub type InstanceWindDataUpdateIDs =
        crate::UnityEngine::Rendering::InstanceDataSystem_InstanceWindDataUpdateIDs;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
    pub type MotionUpdateJob = crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
    pub type ProbesUpdateJob = crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob")]
    pub type QueryRendererGroupInstancesCountJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob")]
    pub type QueryRendererGroupInstancesJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob")]
    pub type QueryRendererGroupInstancesMultiJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
    pub type QuerySortedMeshInstancesJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
    pub type ReallocateInstancesJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob")]
    pub type ScatterTetrahedronCacheIndicesJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
    pub type TransformUpdateJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob;
    #[cfg(
        feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
    )]
    pub type UpdateCompactedInstanceVisibilityJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob;
    #[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
    pub type UpdateRendererInstancesJob =
        crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob;
    pub fn AtomicAddLengthNoResize<T>(
        list: quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeList_1<T>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<crate::Unity::Collections::NativeList_1<T>>,
                        i32,
                    ), i32, 2usize>("AtomicAddLengthNoResize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AtomicAddLengthNoResize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (list, count))? };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchMotionUpdateCommand(
        &mut self,
        motionQueueCount: i32,
        transformInstanceQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        renderersParameters: crate::UnityEngine::Rendering::RenderersParameters,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::UnityEngine::Rendering::RenderersParameters,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "DispatchMotionUpdateCommand"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchMotionUpdateCommand",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    motionQueueCount,
                    transformInstanceQueue,
                    renderersParameters,
                    outputBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchProbeUpdateCommand(
        &mut self,
        queueCount: i32,
        probeInstanceQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        probeUpdateDataQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        >,
        probeOcclusionUpdateDataQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Vector4,
        >,
        renderersParameters: crate::UnityEngine::Rendering::RenderersParameters,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector4>,
                        crate::UnityEngine::Rendering::RenderersParameters,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "DispatchProbeUpdateCommand"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchProbeUpdateCommand",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    queueCount,
                    probeInstanceQueue,
                    probeUpdateDataQueue,
                    probeOcclusionUpdateDataQueue,
                    renderersParameters,
                    outputBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchTransformUpdateCommand(
        &mut self,
        initialize: bool,
        transformQueueCount: i32,
        transformInstanceQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        updateDataQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::TransformUpdatePacket,
        >,
        boundingSphereUpdateDataQueue: crate::Unity::Collections::NativeArray_1<
            crate::Unity::Mathematics::float4,
        >,
        renderersParameters: crate::UnityEngine::Rendering::RenderersParameters,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        bool,
                        i32,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::TransformUpdatePacket,
                        >,
                        crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4>,
                        crate::UnityEngine::Rendering::RenderersParameters,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "DispatchTransformUpdateCommand"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchTransformUpdateCommand",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    initialize,
                    transformQueueCount,
                    transformInstanceQueue,
                    updateDataQueue,
                    boundingSphereUpdateDataQueue,
                    renderersParameters,
                    outputBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchWindDataCopyHistoryCommand(
        &mut self,
        gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
        renderersParameters: crate::UnityEngine::Rendering::RenderersParameters,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::GPUInstanceIndex,
                        >,
                        crate::UnityEngine::Rendering::RenderersParameters,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "DispatchWindDataCopyHistoryCommand"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchWindDataCopyHistoryCommand",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (gpuInstanceIndices, renderersParameters, outputBuffer),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureIndexQueueBufferCapacity(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "EnsureIndexQueueBufferCapacity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnsureIndexQueueBufferCapacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (capacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureProbeBuffersCapacity(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "EnsureProbeBuffersCapacity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnsureProbeBuffersCapacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (capacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureTransformBuffersCapacity(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "EnsureTransformBuffersCapacity",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnsureTransformBuffersCapacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (capacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn FreeInstances(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::Rendering::InstanceHandle,
                    >), quest_hook::libil2cpp::Void, 1usize>("FreeInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FreeInstances",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instances))? };
        Ok(__cordl_ret.into())
    }
    pub fn FreeRendererGroupInstances(
        &mut self,
        rendererGroupsID: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeRendererGroupInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeRendererGroupInstances", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererGroupsID))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAliveInstancesOfType(
        &mut self,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Rendering::InstanceType), i32, 1usize>(
                        "GetAliveInstancesOfType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAliveInstancesOfType",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instanceType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxInstancesOfType(
        &mut self,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::Rendering::InstanceType), i32, 1usize>(
                        "GetMaxInstancesOfType",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaxInstancesOfType",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, (instanceType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetVisibleTreeInstances(
        &mut self,
        compactedVisibilityMasks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ParallelBitArray,
        >,
        processedBits: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ParallelBitArray,
        >,
        visibeTreeRendererIDs: crate::Unity::Collections::NativeList_1<i32>,
        visibeTreeInstances: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        becomeVisibleOnly: bool,
        becomeVisibeTreeInstancesCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ParallelBitArray,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ParallelBitArray,
                        >,
                        crate::Unity::Collections::NativeList_1<i32>,
                        crate::Unity::Collections::NativeList_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "GetVisibleTreeInstances"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetVisibleTreeInstances",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    compactedVisibilityMasks,
                    processedBits,
                    visibeTreeRendererIDs,
                    visibeTreeInstances,
                    becomeVisibleOnly,
                    becomeVisibeTreeInstancesCount,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeInstanceTransforms(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        localToWorldMatrices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Matrix4x4,
        >,
        prevLocalToWorldMatrices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Matrix4x4,
        >,
        renderersParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersParameters,
        >,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderersParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "InitializeInstanceTransforms"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitializeInstanceTransforms",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    instances,
                    localToWorldMatrices,
                    prevLocalToWorldMatrices,
                    renderersParameters,
                    outputBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InternalSanityCheckStates(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("InternalSanityCheckStates")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InternalSanityCheckStates",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        maxInstances: i32,
        enableBoundingSpheres: bool,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxInstances, enableBoundingSpheres, resources))?;
        Ok(__cordl_object.into())
    }
    pub fn ReallocateAndGetInstances(
        &mut self,
        rendererData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
        >,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
                        >,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "ReallocateAndGetInstances"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReallocateAndGetInstances",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererData, instances))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleCollectInstancesLODGroupAndMasksJob(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        lodGroupAndMasks: crate::Unity::Collections::NativeArray_1<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::Unity::Collections::NativeArray_1<u32>,
                    ), crate::Unity::Jobs::JobHandle, 2usize>(
                        "ScheduleCollectInstancesLODGroupAndMasksJob",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleCollectInstancesLODGroupAndMasksJob",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (instances, lodGroupAndMasks))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleInterpolateProbesAndUpdateTetrahedronCache(
        &mut self,
        queueCount: i32,
        probeUpdateInstanceQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        compactTetrahedronCache: crate::Unity::Collections::NativeArray_1<i32>,
        probeQueryPosition: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
        probeUpdateDataQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
        >,
        probeOcclusionUpdateDataQueue: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Vector4,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        >,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector4>,
                    ), crate::Unity::Jobs::JobHandle, 6usize>(
                        "ScheduleInterpolateProbesAndUpdateTetrahedronCache",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleInterpolateProbesAndUpdateTetrahedronCache",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    queueCount,
                    probeUpdateInstanceQueue,
                    compactTetrahedronCache,
                    probeQueryPosition,
                    probeUpdateDataQueue,
                    probeOcclusionUpdateDataQueue,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleQueryRendererGroupInstancesJob_NativeArray_1_0(
        &mut self,
        rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                    ), crate::Unity::Jobs::JobHandle, 2usize>(
                        "ScheduleQueryRendererGroupInstancesJob",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleQueryRendererGroupInstancesJob",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererGroupIDs, instances))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleQueryRendererGroupInstancesJob_NativeArray_1_NativeArray_1_NativeList_1_2(
        &mut self,
        rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
        instancesOffset: crate::Unity::Collections::NativeArray_1<i32>,
        instancesCount: crate::Unity::Collections::NativeArray_1<i32>,
        instances: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeList_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                    ), crate::Unity::Jobs::JobHandle, 4usize>(
                        "ScheduleQueryRendererGroupInstancesJob",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleQueryRendererGroupInstancesJob",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (rendererGroupIDs, instancesOffset, instancesCount, instances),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleQueryRendererGroupInstancesJob_NativeList_1_1(
        &mut self,
        rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
        instances: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeList_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                    ), crate::Unity::Jobs::JobHandle, 2usize>(
                        "ScheduleQueryRendererGroupInstancesJob",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleQueryRendererGroupInstancesJob",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (rendererGroupIDs, instances))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleQuerySortedMeshInstancesJob(
        &mut self,
        sortedMeshIDs: crate::Unity::Collections::NativeArray_1<i32>,
        instances: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeList_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                    ), crate::Unity::Jobs::JobHandle, 2usize>(
                        "ScheduleQuerySortedMeshInstancesJob"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleQuerySortedMeshInstancesJob",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (sortedMeshIDs, instances))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleUpdateInstanceDataJob(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        rendererData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
        >,
        lodGroupDataMap: crate::Unity::Collections::NativeParallelHashMap_2<
            i32,
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
                        >,
                        crate::Unity::Collections::NativeParallelHashMap_2<
                            i32,
                            crate::UnityEngine::Rendering::GPUInstanceIndex,
                        >,
                    ), crate::Unity::Jobs::JobHandle, 3usize>(
                        "ScheduleUpdateInstanceDataJob"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScheduleUpdateInstanceDataJob",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances, rendererData, lodGroupDataMap))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAllInstanceProbes(
        &mut self,
        renderersParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersParameters,
        >,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderersParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UpdateAllInstanceProbes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateAllInstanceProbes",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderersParameters, outputBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceMotions(
        &mut self,
        renderersParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersParameters,
        >,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderersParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UpdateInstanceMotions"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceMotions",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderersParameters, outputBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceMotionsData(
        &mut self,
        renderersParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersParameters,
        >,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderersParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UpdateInstanceMotionsData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceMotionsData",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderersParameters, outputBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceProbesData(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        renderersParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersParameters,
        >,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderersParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UpdateInstanceProbesData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceProbesData",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instances, renderersParameters, outputBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceTransforms(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        localToWorldMatrices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Matrix4x4,
        >,
        renderersParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersParameters,
        >,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderersParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "UpdateInstanceTransforms"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceTransforms",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    instances,
                    localToWorldMatrices,
                    renderersParameters,
                    outputBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceTransformsData(
        &mut self,
        initialize: bool,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        localToWorldMatrices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Matrix4x4,
        >,
        prevLocalToWorldMatrices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Matrix4x4,
        >,
        renderersParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersParameters,
        >,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        bool,
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderersParameters,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "UpdateInstanceTransformsData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceTransformsData",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    initialize,
                    instances,
                    localToWorldMatrices,
                    prevLocalToWorldMatrices,
                    renderersParameters,
                    outputBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceWindDataHistory(
        &mut self,
        gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
        renderersParameters: crate::UnityEngine::Rendering::RenderersParameters,
        outputBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::GPUInstanceIndex,
                        >,
                        crate::UnityEngine::Rendering::RenderersParameters,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UpdateInstanceWindDataHistory"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceWindDataHistory",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (gpuInstanceIndices, renderersParameters, outputBuffer),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePerFrameInstanceVisibility(
        &mut self,
        compactedVisibilityMasks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ParallelBitArray,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::ParallelBitArray,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UpdatePerFrameInstanceVisibility"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdatePerFrameInstanceVisibility",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (compactedVisibilityMasks))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        maxInstances: i32,
        enableBoundingSpheres: bool,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (maxInstances, enableBoundingSpheres, resources))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_aliveInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::Rendering::InstanceHandle,
                    >, 0usize>("get_aliveInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_aliveInstances",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasBoundingSpheres(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasBoundingSpheres")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_hasBoundingSpheres",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instanceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
                        0usize,
                    >("get_instanceData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_instanceData", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sharedInstanceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
                        0usize,
                    >("get_sharedInstanceData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_sharedInstanceData", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::InstanceDataSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::InstanceDataSystem {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::InstanceDataSystem {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    pub probesCount: i32,
    pub lightProbesQuery: crate::UnityEngine::LightProbesQuery,
    pub queryPostitions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
    pub compactTetrahedronCache: crate::Unity::Collections::NativeArray_1<i32>,
    pub probesSphericalHarmonics: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::SphericalHarmonicsL2,
    >,
    pub probesOcclusion: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector4>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/CalculateInterpolatedLightAndOcclusionProbesBatchJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
impl crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    pub const k_BatchSize: i32 = 1i32;
    pub const k_CalculatedProbesPerBatch: i32 = 8i32;
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+InstanceDataSystem+CalculateInterpolatedLightAndOcclusionProbesBatchJob"
)]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
for crate::UnityEngine::Rendering::InstanceDataSystem_CalculateInterpolatedLightAndOcclusionProbesBatchJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob {
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
    pub lodGroupAndMasks: crate::Unity::Collections::NativeArray_1<u32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/CollectInstancesLODGroupsAndMasksJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+CollectInstancesLODGroupsAndMasksJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_CollectInstancesLODGroupsAndMasksJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    pub instancesCount: crate::Unity::Collections::NativeArray_1<i32>,
    pub instancesOffset: crate::Unity::Collections::NativeArray_1<i32>,
    pub instances:
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::InstanceHandle>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/ComputeInstancesOffsetAndResizeInstancesArrayJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
impl crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+InstanceDataSystem+ComputeInstancesOffsetAndResizeInstancesArrayJob"
)]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::InstanceDataSystem_ComputeInstancesOffsetAndResizeInstancesArrayJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_FreeInstancesJob {
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub instanceAllocators: crate::UnityEngine::Rendering::InstanceAllocators,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub rendererGroupInstanceMultiHash: crate::Unity::Collections::NativeParallelMultiHashMap_2<
        i32,
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/FreeInstancesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob {
    pub fn Execute(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeInstancesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_FreeRendererGroupInstancesJob {
    pub rendererGroupsID: crate::Unity::Collections::NativeArray_1<i32>,
    pub instanceAllocators: crate::UnityEngine::Rendering::InstanceAllocators,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub rendererGroupInstanceMultiHash: crate::Unity::Collections::NativeParallelMultiHashMap_2<
        i32,
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/FreeRendererGroupInstancesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob {
    pub fn Execute(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+FreeRendererGroupInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceDataSystem_FreeRendererGroupInstancesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob {
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub compactedVisibilityMasks: crate::UnityEngine::Rendering::ParallelBitArray,
    pub becomeVisible: bool,
    pub processedBits: crate::UnityEngine::Rendering::ParallelBitArray,
    pub rendererIDs: crate::Unity::Collections::NativeArray_1<i32>,
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub atomicTreeInstancesCount:
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeAtomicCounter32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/GetVisibleNonProcessedTreeInstancesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob {
    pub const k_BatchSize: i32 = 64i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+GetVisibleNonProcessedTreeInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_GetVisibleNonProcessedTreeInstancesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+InstanceTransformUpdateIDs")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct InstanceDataSystem_InstanceTransformUpdateIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+InstanceTransformUpdateIDs")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceTransformUpdateIDs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/InstanceTransformUpdateIDs";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceTransformUpdateIDs")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceTransformUpdateIDs
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceTransformUpdateIDs")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceTransformUpdateIDs
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceTransformUpdateIDs")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_InstanceTransformUpdateIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+InstanceTransformUpdateIDs")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceTransformUpdateIDs
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+InstanceWindDataUpdateIDs")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct InstanceDataSystem_InstanceWindDataUpdateIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+InstanceWindDataUpdateIDs")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceWindDataUpdateIDs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/InstanceWindDataUpdateIDs";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceWindDataUpdateIDs")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceWindDataUpdateIDs
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceWindDataUpdateIDs")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceWindDataUpdateIDs
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+InstanceWindDataUpdateIDs")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_InstanceWindDataUpdateIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+InstanceWindDataUpdateIDs")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::InstanceDataSystem_InstanceWindDataUpdateIDs
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_MotionUpdateJob {
    pub queueWriteBase: i32,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub atomicUpdateQueueCount: crate::Unity::Collections::LowLevel::Unsafe::UnsafeAtomicCounter32,
    pub transformUpdateInstanceQueue:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/MotionUpdateJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob {
    pub const k_BatchSize: i32 = 16i32;
    pub fn Execute(
        &mut self,
        chunk_index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (chunk_index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+MotionUpdateJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_MotionUpdateJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_ProbesUpdateJob {
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub atomicProbesQueueCount: crate::Unity::Collections::LowLevel::Unsafe::UnsafeAtomicCounter32,
    pub probeInstanceQueue:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub compactTetrahedronCache: crate::Unity::Collections::NativeArray_1<i32>,
    pub probeQueryPosition: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/ProbesUpdateJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob {
    pub const k_BatchSize: i32 = 64i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ProbesUpdateJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_ProbesUpdateJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_QueryRendererGroupInstancesCountJob {
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub rendererGroupInstanceMultiHash: crate::Unity::Collections::NativeParallelMultiHashMap_2<
        i32,
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
    pub rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
    pub instancesCount: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/QueryRendererGroupInstancesCountJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesCountJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesCountJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_QueryRendererGroupInstancesJob {
    pub rendererGroupInstanceMultiHash: crate::Unity::Collections::NativeParallelMultiHashMap_2<
        i32,
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
    pub rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub atomicNonFoundInstancesCount:
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeAtomicCounter32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/QueryRendererGroupInstancesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_QueryRendererGroupInstancesMultiJob {
    pub rendererGroupInstanceMultiHash: crate::Unity::Collections::NativeParallelMultiHashMap_2<
        i32,
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
    pub rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
    pub instancesOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub instancesCounts: crate::Unity::Collections::NativeArray_1<i32>,
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub atomicNonFoundSharedInstancesCount:
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeAtomicCounter32,
    pub atomicNonFoundInstancesCount:
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeAtomicCounter32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/QueryRendererGroupInstancesMultiJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QueryRendererGroupInstancesMultiJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QueryRendererGroupInstancesMultiJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_QuerySortedMeshInstancesJob {
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub sortedMeshID: crate::Unity::Collections::NativeArray_1<i32>,
    pub instances:
        crate::Unity::Collections::NativeList_1<crate::UnityEngine::Rendering::InstanceHandle>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/QuerySortedMeshInstancesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob {
    pub const k_BatchSize: i32 = 64i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+QuerySortedMeshInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_QuerySortedMeshInstancesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_ReallocateInstancesJob {
    pub implicitInstanceIndices: bool,
    pub rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
    pub packedRendererData: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::GPUDrivenPackedRendererData,
    >,
    pub instanceOffsets: crate::Unity::Collections::NativeArray_1<i32>,
    pub instanceCounts: crate::Unity::Collections::NativeArray_1<i32>,
    pub instanceAllocators: crate::UnityEngine::Rendering::InstanceAllocators,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub rendererGroupInstanceMultiHash: crate::Unity::Collections::NativeParallelMultiHashMap_2<
        i32,
        crate::UnityEngine::Rendering::InstanceHandle,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/ReallocateInstancesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob {
    pub fn Execute(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ReallocateInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceDataSystem_ReallocateInstancesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_ScatterTetrahedronCacheIndicesJob {
    pub probeInstances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub compactTetrahedronCache: crate::Unity::Collections::NativeArray_1<i32>,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/ScatterTetrahedronCacheIndicesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+ScatterTetrahedronCacheIndicesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_ScatterTetrahedronCacheIndicesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_TransformUpdateJob {
    pub initialize: bool,
    pub enableBoundingSpheres: bool,
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub localToWorldMatrices:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
    pub prevLocalToWorldMatrices:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
    pub atomicTransformQueueCount:
        crate::Unity::Collections::LowLevel::Unsafe::UnsafeAtomicCounter32,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub transformUpdateInstanceQueue:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub transformUpdateDataQueue: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::TransformUpdatePacket,
    >,
    pub boundingSpheresDataQueue:
        crate::Unity::Collections::NativeArray_1<crate::Unity::Mathematics::float4>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/TransformUpdateJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob {
    pub const k_BatchSize: i32 = 64i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+TransformUpdateJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_TransformUpdateJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_UpdateCompactedInstanceVisibilityJob {
    pub compactedVisibilityMasks: crate::UnityEngine::Rendering::ParallelBitArray,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/UpdateCompactedInstanceVisibilityJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob {
    pub const k_BatchSize: i32 = 64i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (startIndex, count))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateCompactedInstanceVisibilityJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateCompactedInstanceVisibilityJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct InstanceDataSystem_UpdateRendererInstancesJob {
    pub implicitInstanceIndices: bool,
    pub rendererData: crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
    pub instances:
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Rendering::InstanceHandle>,
    pub lodGroupDataMap: crate::Unity::Collections::NativeParallelHashMap_2<
        i32,
        crate::UnityEngine::Rendering::GPUInstanceIndex,
    >,
    pub instanceData: crate::UnityEngine::Rendering::CPUInstanceData,
    pub sharedInstanceData: crate::UnityEngine::Rendering::CPUSharedInstanceData,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceDataSystem/UpdateRendererInstancesJob";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
impl crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceDataSystem+UpdateRendererInstancesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::Rendering::InstanceDataSystem_UpdateRendererInstancesJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
