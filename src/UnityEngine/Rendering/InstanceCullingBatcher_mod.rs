#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCullingBatcher")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct InstanceCullingBatcher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_BatchersContext:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RenderersBatchersContext>,
    pub m_DrawInstanceData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CPUDrawInstanceData>,
    pub m_BRG: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::BatchRendererGroup>,
    pub m_GlobalBatchIDs: crate::Unity::Collections::NativeParallelHashMap_2<
        u32,
        crate::UnityEngine::Rendering::BatchID,
    >,
    pub m_Culler: crate::UnityEngine::Rendering::InstanceCuller,
    pub m_BatchMaterialHash: crate::Unity::Collections::NativeParallelHashMap_2<
        i32,
        crate::UnityEngine::Rendering::BatchMaterialID,
    >,
    pub m_PackedMaterialHash: crate::Unity::Collections::NativeParallelHashMap_2<
        i32,
        crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
    >,
    pub m_BatchMeshHash: crate::Unity::Collections::NativeParallelHashMap_2<
        i32,
        crate::UnityEngine::Rendering::BatchMeshID,
    >,
    pub m_CachedInstanceDataBufferLayoutVersion: i32,
    pub m_OnCompleteCallback:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::OnCullingCompleteCallback>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCullingBatcher")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::InstanceCullingBatcher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "InstanceCullingBatcher";
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
#[cfg(feature = "UnityEngine+Rendering+InstanceCullingBatcher")]
impl std::ops::Deref for crate::UnityEngine::Rendering::InstanceCullingBatcher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCullingBatcher")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::InstanceCullingBatcher {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCullingBatcher")]
impl crate::UnityEngine::Rendering::InstanceCullingBatcher {
    pub fn BuildBatch(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        usedMaterialIDs: crate::Unity::Collections::NativeArray_1<i32>,
        usedMeshIDs: crate::Unity::Collections::NativeArray_1<i32>,
        rendererData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
        >,
        registerMaterialsAndMeshes: bool,
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
                        crate::Unity::Collections::NativeArray_1<i32>,
                        crate::Unity::Collections::NativeArray_1<i32>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("BuildBatch")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildBatch",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    instances,
                    usedMaterialIDs,
                    usedMeshIDs,
                    rendererData,
                    registerMaterialsAndMeshes,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyDrawInstances(
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
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "DestroyDrawInstances"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DestroyDrawInstances",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (instances))? };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyMaterials(
        &mut self,
        destroyedMaterials: crate::Unity::Collections::NativeArray_1<i32>,
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
                    >("DestroyMaterials")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyMaterials", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (destroyedMaterials))? };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyMeshes(
        &mut self,
        destroyedMeshes: crate::Unity::Collections::NativeArray_1<i32>,
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
                    >("DestroyMeshes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyMeshes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (destroyedMeshes))? };
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
    pub fn GetBatchID(
        &mut self,
        componentsOverriden: crate::UnityEngine::Rendering::InstanceComponentGroup,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::BatchID> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::InstanceComponentGroup),
                        crate::UnityEngine::Rendering::BatchID,
                        1usize,
                    >("GetBatchID")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBatchID", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::BatchID =
            unsafe { cordl_method_info.invoke_unchecked(self, (componentsOverriden))? };
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
    pub fn GetDrawInstanceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CPUDrawInstanceData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CPUDrawInstanceData,
                        >,
                        0usize,
                    >("GetDrawInstanceData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDrawInstanceData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::CPUDrawInstanceData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn InstanceOccludersUpdated(
        &mut self,
        viewInstanceID: i32,
        subviewMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32, i32), quest_hook::libil2cpp::Void, 2usize>(
                        "InstanceOccludersUpdated",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InstanceOccludersUpdated",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID, subviewMask))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        batcherContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderersBatchersContext,
        >,
        desc: crate::UnityEngine::Rendering::InstanceCullingBatcherDesc,
        onFinishedCulling: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::BatchRendererGroup_OnFinishedCulling,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (batcherContext, desc, onFinishedCulling))?;
        Ok(__cordl_object.into())
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
    pub fn OnEndContextRendering(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEndContextRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEndContextRendering",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnFinishedCulling(
        &mut self,
        customCullingResult: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        "OnFinishedCulling",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnFinishedCulling",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (customCullingResult))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnPerformCulling(
        &mut self,
        rendererGroup: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::BatchRendererGroup>,
        cc: crate::UnityEngine::Rendering::BatchCullingContext,
        cullingOutput: crate::UnityEngine::Rendering::BatchCullingOutput,
        userContext: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::BatchRendererGroup,
                        >,
                        crate::UnityEngine::Rendering::BatchCullingContext,
                        crate::UnityEngine::Rendering::BatchCullingOutput,
                        crate::System::IntPtr,
                    ), crate::Unity::Jobs::JobHandle, 4usize>("OnPerformCulling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnPerformCulling",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rendererGroup, cc, cullingOutput, userContext))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PostCullBeginCameraRendering(
        &mut self,
        context: crate::UnityEngine::Rendering::RenderRequestBatcherContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::RenderRequestBatcherContext),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("PostCullBeginCameraRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PostCullBeginCameraRendering", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterBatchMaterials(
        &mut self,
        usedMaterialIDs: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeArray_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::Unity::Collections::NativeArray_1<i32>,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("RegisterBatchMaterials")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterBatchMaterials", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (usedMaterialIDs))? };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterBatchMeshes(
        &mut self,
        meshIDs: crate::Unity::Collections::NativeArray_1<i32>,
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
                    >("RegisterBatchMeshes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterBatchMeshes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (meshIDs))? };
        Ok(__cordl_ret.into())
    }
    pub fn SchedulePackedMaterialCacheUpdate(
        &mut self,
        materialIDs: crate::Unity::Collections::NativeArray_1<i32>,
        packedMaterialDatas: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
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
                            crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
                        >,
                    ), crate::Unity::Jobs::JobHandle, 2usize>(
                        "SchedulePackedMaterialCacheUpdate"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SchedulePackedMaterialCacheUpdate",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (materialIDs, packedMaterialDatas))?
        };
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
    pub fn UpdateInstanceDataBufferLayoutVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UpdateInstanceDataBufferLayoutVersion",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceDataBufferLayoutVersion",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        batcherContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderersBatchersContext,
        >,
        desc: crate::UnityEngine::Rendering::InstanceCullingBatcherDesc,
        onFinishedCulling: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::BatchRendererGroup_OnFinishedCulling,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderersBatchersContext,
                        >,
                        crate::UnityEngine::Rendering::InstanceCullingBatcherDesc,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::BatchRendererGroup_OnFinishedCulling,
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
            cordl_method_info.invoke_unchecked(self, (batcherContext, desc, onFinishedCulling))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_batchMaterialHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeParallelHashMap_2<
            i32,
            crate::UnityEngine::Rendering::BatchMaterialID,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeParallelHashMap_2<
                        i32,
                        crate::UnityEngine::Rendering::BatchMaterialID,
                    >, 0usize>("get_batchMaterialHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_batchMaterialHash",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeParallelHashMap_2<
            i32,
            crate::UnityEngine::Rendering::BatchMaterialID,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_culler(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::InstanceCuller>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::InstanceCuller,
                        >,
                        0usize,
                    >("get_culler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_culler", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::InstanceCuller,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_packedMaterialHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeParallelHashMap_2<
            i32,
            crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Collections::NativeParallelHashMap_2<
                        i32,
                        crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
                    >, 0usize>("get_packedMaterialHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_packedMaterialHash",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeParallelHashMap_2<
            i32,
            crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+InstanceCullingBatcher")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::InstanceCullingBatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCullingBatcher")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::InstanceCullingBatcher {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+InstanceCullingBatcher")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::InstanceCullingBatcher {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
