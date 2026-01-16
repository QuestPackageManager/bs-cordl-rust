#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InstanceCuller {
    pub m_CompactedVisibilityMasks: crate::UnityEngine::Rendering::ParallelBitArray,
    pub m_CompactedVisibilityMasksJobsHandle: crate::Unity::Jobs::JobHandle,
    pub m_IndirectStorage: crate::UnityEngine::Rendering::IndirectBufferContextStorage,
    pub m_OcclusionTestShader: crate::UnityEngine::Rendering::OcclusionTestComputeShader,
    pub m_ResetDrawArgsKernel: i32,
    pub m_CopyInstancesKernel: i32,
    pub m_CullInstancesKernel: i32,
    pub m_DebugStats:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugRendererBatcherStats>,
    pub m_SplitDebugArray: crate::UnityEngine::Rendering::InstanceCullerSplitDebugArray,
    pub m_OcclusionEventDebugArray: crate::UnityEngine::Rendering::InstanceOcclusionEventDebugArray,
    pub m_ProfilingSampleInstanceOcclusionTest:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    pub m_ShaderVariables: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::InstanceOcclusionCullerShaderVariables,
    >,
    pub m_ConstantBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_CommandBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::InstanceCuller {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceCuller";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::Rendering::InstanceCuller {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::Rendering::InstanceCuller {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::Rendering::InstanceCuller {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::Rendering::InstanceCuller {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Rendering::InstanceCuller {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller")]
impl crate::UnityEngine::Rendering::InstanceCuller {
    #[cfg(feature = "UnityEngine+Rendering+InstanceCuller+InstanceOcclusionTestPassData")]
    pub type InstanceOcclusionTestPassData =
        crate::UnityEngine::Rendering::InstanceCuller_InstanceOcclusionTestPassData;
    #[cfg(feature = "UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
    pub type SetupCullingJobInput =
        crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput;
    #[cfg(feature = "UnityEngine+Rendering+InstanceCuller+ShaderIDs")]
    pub type ShaderIDs = crate::UnityEngine::Rendering::InstanceCuller_ShaderIDs;
    pub fn AddOcclusionCullingDispatch(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ComputeCommandBuffer>,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OcclusionCullingSettings,
        >,
        subviewSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::InstanceOcclusionTestSubviewSettings,
        >,
        bufferHandles: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::IndirectBufferContextHandles,
        >,
        occluderHandles: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderHandles,
        >,
        batchersContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderersBatchersContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ComputeCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OcclusionCullingSettings,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::InstanceOcclusionTestSubviewSettings,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::IndirectBufferContextHandles,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OccluderHandles,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderersBatchersContext,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "AddOcclusionCullingDispatch"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddOcclusionCullingDispatch",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    settings,
                    subviewSettings,
                    bufferHandles,
                    occluderHandles,
                    batchersContext,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeWorstCaseDrawCommandCount(
        &mut self,
        cc: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::BatchCullingContext>,
        binningConfig: crate::UnityEngine::Rendering::BinningConfig,
        drawInstanceData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CPUDrawInstanceData,
        >,
        crossFadedRendererCount: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::BatchCullingContext,
                        >,
                        crate::UnityEngine::Rendering::BinningConfig,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CPUDrawInstanceData,
                        >,
                        i32,
                    ), i32, 4usize>("ComputeWorstCaseDrawCommandCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputeWorstCaseDrawCommandCount",
                            4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cc, binningConfig, drawInstanceData, crossFadedRendererCount),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCompactedVisibilityMaskJob(
        &mut self,
        instanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
        >,
        rendererVisibilityMasks: crate::Unity::Collections::NativeArray_1<u8>,
        cullingJobHandle: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
                        >,
                        crate::Unity::Collections::NativeArray_1<u8>,
                        crate::Unity::Jobs::JobHandle,
                    ), crate::Unity::Jobs::JobHandle, 3usize>(
                        "CreateCompactedVisibilityMaskJob"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateCompactedVisibilityMaskJob",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (instanceData, rendererVisibilityMasks, cullingJobHandle),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateCullJobTree(
        &mut self,
        cc: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::BatchCullingContext>,
        cullingOutput: crate::UnityEngine::Rendering::BatchCullingOutput,
        instanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
        >,
        sharedInstanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
        >,
        instanceDataBuffer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly,
        >,
        lodGroupCullingData: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::LODGroupCullingData,
        >,
        drawInstanceData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CPUDrawInstanceData,
        >,
        batchIDs: crate::Unity::Collections::NativeParallelHashMap_2<
            u32,
            crate::UnityEngine::Rendering::BatchID,
        >,
        crossFadedRendererCount: i32,
        smallMeshScreenPercentage: f32,
        occlusionCullingCommon: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::OcclusionCullingCommon,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::BatchCullingContext,
                        >,
                        crate::UnityEngine::Rendering::BatchCullingOutput,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly,
                        >,
                        crate::Unity::Collections::NativeList_1<
                            crate::UnityEngine::Rendering::LODGroupCullingData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CPUDrawInstanceData,
                        >,
                        crate::Unity::Collections::NativeParallelHashMap_2<
                            u32,
                            crate::UnityEngine::Rendering::BatchID,
                        >,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::OcclusionCullingCommon,
                        >,
                    ), crate::Unity::Jobs::JobHandle, 11usize>(
                        "CreateCullJobTree"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateCullJobTree",
                            11usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cc,
                    cullingOutput,
                    instanceData,
                    sharedInstanceData,
                    instanceDataBuffer,
                    lodGroupCullingData,
                    drawInstanceData,
                    batchIDs,
                    crossFadedRendererCount,
                    smallMeshScreenPercentage,
                    occlusionCullingCommon,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFrustumCullingJob(
        &mut self,
        cc: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::BatchCullingContext>,
        instanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
        >,
        sharedInstanceData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
        >,
        lodGroupCullingData: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::LODGroupCullingData,
        >,
        binningConfig: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::BinningConfig,
        >,
        smallMeshScreenPercentage: f32,
        occlusionCullingCommon: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::OcclusionCullingCommon,
        >,
        rendererVisibilityMasks: crate::Unity::Collections::NativeArray_1<u8>,
        rendererCrossFadeValues: crate::Unity::Collections::NativeArray_1<u8>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::BatchCullingContext,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
                        >,
                        crate::Unity::Collections::NativeList_1<
                            crate::UnityEngine::Rendering::LODGroupCullingData,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::BinningConfig,
                        >,
                        f32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::OcclusionCullingCommon,
                        >,
                        crate::Unity::Collections::NativeArray_1<u8>,
                        crate::Unity::Collections::NativeArray_1<u8>,
                    ), crate::Unity::Jobs::JobHandle, 9usize>(
                        "CreateFrustumCullingJob"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateFrustumCullingJob",
                            9usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cc,
                    instanceData,
                    sharedInstanceData,
                    lodGroupCullingData,
                    binningConfig,
                    smallMeshScreenPercentage,
                    occlusionCullingCommon,
                    rendererVisibilityMasks,
                    rendererCrossFadeValues,
                ),
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
    pub fn DisposeCompactVisibilityMasks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "DisposeCompactVisibilityMasks",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisposeCompactVisibilityMasks",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DisposeSceneViewHiddenBits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "DisposeSceneViewHiddenBits",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DisposeSceneViewHiddenBits",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureValidOcclusionTestResults(
        &mut self,
        viewInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "EnsureValidOcclusionTestResults",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "EnsureValidOcclusionTestResults",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn FlushDebugCounters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("FlushDebugCounters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FlushDebugCounters",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompactedVisibilityMasks(
        &mut self,
        syncCullingJobs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::ParallelBitArray> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), crate::UnityEngine::Rendering::ParallelBitArray, 1usize>(
                        "GetCompactedVisibilityMasks",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCompactedVisibilityMasks",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::ParallelBitArray =
            unsafe { cordl_method_info.invoke_unchecked(self, (syncCullingJobs))? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
        debugStats: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Init",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resources, debugStats))? };
        Ok(__cordl_ret.into())
    }
    pub fn InstanceOccludersUpdated(
        &mut self,
        viewInstanceID: i32,
        subviewMask: i32,
        batchersContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderersBatchersContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderersBatchersContext,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "InstanceOccludersUpdated"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InstanceOccludersUpdated",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (viewInstanceID, subviewMask, batchersContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InstanceOcclusionTest(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OcclusionCullingSettings,
        >,
        subviewOcclusionTests: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::SubviewOcclusionTest,
        >,
        batchersContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderersBatchersContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OcclusionCullingSettings,
                        >,
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::SubviewOcclusionTest,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderersBatchersContext,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "InstanceOcclusionTest"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InstanceOcclusionTest",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    settings,
                    subviewOcclusionTests,
                    batchersContext,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnBeginCameraRendering(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnBeginCameraRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnBeginCameraRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnBeginSceneViewCameraRendering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "OnBeginSceneViewCameraRendering",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnBeginSceneViewCameraRendering",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndCameraRendering(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnEndCameraRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEndCameraRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndSceneViewCameraRendering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "OnEndSceneViewCameraRendering",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEndSceneViewCameraRendering",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFrame(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateFrame",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::InstanceCuller {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::InstanceCuller {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+InstanceOcclusionTestPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct InstanceCuller_InstanceOcclusionTestPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub settings: crate::UnityEngine::Rendering::OcclusionCullingSettings,
    pub subviewSettings: crate::UnityEngine::Rendering::InstanceOcclusionTestSubviewSettings,
    pub occluderHandles: crate::UnityEngine::Rendering::OccluderHandles,
    pub bufferHandles: crate::UnityEngine::Rendering::IndirectBufferContextHandles,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+InstanceOcclusionTestPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceCuller_InstanceOcclusionTestPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceCuller/InstanceOcclusionTestPassData";
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
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+InstanceOcclusionTestPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::InstanceCuller_InstanceOcclusionTestPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+InstanceOcclusionTestPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::InstanceCuller_InstanceOcclusionTestPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+InstanceOcclusionTestPassData")]
impl crate::UnityEngine::Rendering::InstanceCuller_InstanceOcclusionTestPassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+InstanceOcclusionTestPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::InstanceCuller_InstanceOcclusionTestPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct InstanceCuller_SetupCullingJobInput {
    pub lodBias: f32,
    pub context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub receiverPlanes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub receiverSphereCuller: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub frustumPlaneCuller: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub screenRelativeMetric: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceCuller/SetupCullingJobInput";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
impl crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput {
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
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
impl AsRef<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+SetupCullingJobInput")]
impl AsMut<crate::Unity::Jobs::IJob>
    for crate::UnityEngine::Rendering::InstanceCuller_SetupCullingJobInput
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+ShaderIDs")]
#[repr(C)]
#[derive(Debug)]
pub struct InstanceCuller_ShaderIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+ShaderIDs")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::InstanceCuller_ShaderIDs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceCuller/ShaderIDs";
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
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+ShaderIDs")]
impl std::ops::Deref for crate::UnityEngine::Rendering::InstanceCuller_ShaderIDs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+ShaderIDs")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::InstanceCuller_ShaderIDs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCuller+ShaderIDs")]
impl crate::UnityEngine::Rendering::InstanceCuller_ShaderIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCuller+ShaderIDs")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::InstanceCuller_ShaderIDs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
