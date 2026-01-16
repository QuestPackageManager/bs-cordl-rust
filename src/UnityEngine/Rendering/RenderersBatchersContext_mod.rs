#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderersBatchersContext")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderersBatchersContext {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_InstanceDataSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::InstanceDataSystem,
    >,
    pub m_Resources: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::GPUResidentDrawerResources,
    >,
    pub m_GPUDrivenProcessor: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::GPUDrivenProcessor,
    >,
    pub m_LODGroupDataPool: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::LODGroupDataPool,
    >,
    pub m_InstanceDataBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
    >,
    pub m_RenderersParameters: crate::UnityEngine::Rendering::RenderersParameters,
    pub m_UploadResources: crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader_GPUResources,
    pub m_GrowerResources: crate::UnityEngine::Rendering::GPUInstanceDataBufferGrower_GPUResources,
    pub m_CmdBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub m_CachedAmbientProbe: crate::UnityEngine::Rendering::SphericalHarmonicsL2,
    pub m_SmallMeshScreenPercentage: f32,
    pub m_UpdateLODGroupCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::GPUDrivenLODGroupDataCallback,
    >,
    pub m_TransformLODGroupCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::GPUDrivenLODGroupDataCallback,
    >,
    pub m_OcclusionCullingCommon: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::OcclusionCullingCommon,
    >,
    pub m_DebugStats: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::DebugRendererBatcherStats,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderersBatchersContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderersBatchersContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "RenderersBatchersContext";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+RenderersBatchersContext")]
impl std::ops::Deref for crate::UnityEngine::Rendering::RenderersBatchersContext {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderersBatchersContext")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::RenderersBatchersContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderersBatchersContext")]
impl crate::UnityEngine::Rendering::RenderersBatchersContext {
    pub fn ChangeInstanceBufferVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ChangeInstanceBufferVersion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ChangeInstanceBufferVersion", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDataBufferUploader(
        &mut self,
        capacity: i32,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, crate::UnityEngine::Rendering::InstanceType),
                        crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader,
                        2usize,
                    >("CreateDataBufferUploader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDataBufferUploader", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader = unsafe {
            cordl_method_info.invoke_unchecked(self, (capacity, instanceType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyLODGroups(
        &mut self,
        destroyed: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DestroyLODGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DestroyLODGroups", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (destroyed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInstanceBufferCapacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("EnsureInstanceBufferCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EnsureInstanceBufferCapacity", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeInstances(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FreeInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeInstances", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FreeRendererGroupInstances(
        &mut self,
        rendererGroupsID: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererGroupsID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAliveInstancesOfType(
        &mut self,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::InstanceType),
                        i32,
                        1usize,
                    >("GetAliveInstancesOfType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAliveInstancesOfType", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (instanceType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceDataBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::GPUInstanceDataBuffer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
                        >,
                        0usize,
                    >("GetInstanceDataBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetInstanceDataBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUInstanceDataBuffer,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxInstancesOfType(
        &mut self,
        instanceType: crate::UnityEngine::Rendering::InstanceType,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::InstanceType),
                        i32,
                        1usize,
                    >("GetMaxInstancesOfType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMaxInstancesOfType", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (instanceType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetRendererInstanceHandle(
        &mut self,
        rendererID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::InstanceHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::InstanceHandle,
                        1usize,
                    >("GetRendererInstanceHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetRendererInstanceHandle", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::InstanceHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererID))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
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
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("GetVisibleTreeInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetVisibleTreeInstances", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
    pub fn GrowInstanceBuffer(
        &mut self,
        instanceNumInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::InstanceNumInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::InstanceNumInfo,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("GrowInstanceBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GrowInstanceBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instanceNumInfo))?
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Matrix4x4,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InitializeInstanceTransforms")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeInstanceTransforms", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (instances, localToWorldMatrices, prevLocalToWorldMatrices),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersBatchersContextDesc,
        >,
        gpuDrivenProcessor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUDrivenProcessor,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (desc, gpuDrivenProcessor, resources))?;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ReallocateAndGetInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReallocateAndGetInstances", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererData, instances))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleCollectInstancesLODGroupAndMasksJob(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        lodGroupAndMasks: crate::Unity::Collections::NativeArray_1<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            crate::Unity::Collections::NativeArray_1<u32>,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        2usize,
                    >("ScheduleCollectInstancesLODGroupAndMasksJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScheduleCollectInstancesLODGroupAndMasksJob", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances, lodGroupAndMasks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleQueryMeshInstancesJob(
        &mut self,
        sortedMeshIDs: crate::Unity::Collections::NativeArray_1<i32>,
        instances: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeList_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        2usize,
                    >("ScheduleQueryMeshInstancesJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScheduleQueryMeshInstancesJob", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (sortedMeshIDs, instances))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        2usize,
                    >("ScheduleQueryRendererGroupInstancesJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScheduleQueryRendererGroupInstancesJob", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererGroupIDs, instances))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeList_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        4usize,
                    >("ScheduleQueryRendererGroupInstancesJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScheduleQueryRendererGroupInstancesJob", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeList_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        2usize,
                    >("ScheduleQueryRendererGroupInstancesJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScheduleQueryRendererGroupInstancesJob", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendererGroupIDs, instances))?
        };
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
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::GPUDrivenRendererGroupData,
                            >,
                        ),
                        crate::Unity::Jobs::JobHandle,
                        2usize,
                    >("ScheduleUpdateInstanceDataJob")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ScheduleUpdateInstanceDataJob", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances, rendererData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitToGpu_NativeArray_1_ByRefMut__cordl_bool0(
        &mut self,
        instances: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
        uploader: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader,
        >,
        submitOnlyWrittenParams: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SubmitToGpu")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SubmitToGpu", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (instances, uploader, submitOnlyWrittenParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitToGpu_NativeArray_1_ByRefMut__cordl_bool1(
        &mut self,
        gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
        uploader: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader,
        >,
        submitOnlyWrittenParams: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::GPUInstanceIndex,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::GPUInstanceDataBufferUploader,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("SubmitToGpu")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SubmitToGpu", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (gpuInstanceIndices, uploader, submitOnlyWrittenParams),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransformLODGroupData(
        &mut self,
        lodGroupData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("TransformLODGroupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TransformLODGroupData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lodGroupData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TransformLODGroups(
        &mut self,
        lodGroupsID: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("TransformLODGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TransformLODGroups", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lodGroupsID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAmbientProbeAndGpuBuffer(
        &mut self,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateAmbientProbeAndGpuBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateAmbientProbeAndGpuBuffer", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (forceUpdate))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("UpdateFrame")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateFrame", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceMotions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("UpdateInstanceMotions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateInstanceMotions", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::InstanceHandle,
                            >,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UpdateInstanceTransforms")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateInstanceTransforms", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances, localToWorldMatrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceWindDataHistory(
        &mut self,
        gpuInstanceIndices: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUInstanceIndex,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::GPUInstanceIndex,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateInstanceWindDataHistory")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateInstanceWindDataHistory", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (gpuInstanceIndices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLODGroupData(
        &mut self,
        lodGroupData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::GPUDrivenLODGroupData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateLODGroupData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateLODGroupData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (lodGroupData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateLODGroups(
        &mut self,
        changedID: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateLODGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateLODGroups", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (changedID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdatePerFrameInstanceVisibility(
        &mut self,
        compactedVisibilityMasks: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::ParallelBitArray,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::ParallelBitArray,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdatePerFrameInstanceVisibility")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdatePerFrameInstanceVisibility", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (compactedVisibilityMasks))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderersBatchersContextDesc,
        >,
        gpuDrivenProcessor: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUDrivenProcessor,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderersBatchersContextDesc,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::GPUDrivenProcessor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (desc, gpuDrivenProcessor, resources))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_activeLodGroupCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_activeLodGroupCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_activeLodGroupCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_aliveInstances(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::InstanceHandle,
                        >,
                        0usize,
                    >("get_aliveInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_aliveInstances", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::InstanceHandle,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_cachedAmbientProbe(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SphericalHarmonicsL2,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::SphericalHarmonicsL2,
                        0usize,
                    >("get_cachedAmbientProbe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_cachedAmbientProbe", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::SphericalHarmonicsL2 = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_crossfadedRendererCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_crossfadedRendererCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_crossfadedRendererCount", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_debugStats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
                        >,
                        0usize,
                    >("get_debugStats")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_debugStats", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultDescriptions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::Rendering::GPUInstanceComponentDesc,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Collections::NativeArray_1_ReadOnly<
                            crate::UnityEngine::Rendering::GPUInstanceComponentDesc,
                        >,
                        0usize,
                    >("get_defaultDescriptions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_defaultDescriptions", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1_ReadOnly<
            crate::UnityEngine::Rendering::GPUInstanceComponentDesc,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMetadata(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::MetadataValue,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Collections::NativeArray_1<
                            crate::UnityEngine::Rendering::MetadataValue,
                        >,
                        0usize,
                    >("get_defaultMetadata")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_defaultMetadata", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::MetadataValue,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_gpuInstanceDataBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
                        0usize,
                    >("get_gpuInstanceDataBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_gpuInstanceDataBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasBoundingSpheres(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasBoundingSpheres")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_hasBoundingSpheres", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instanceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: crate::UnityEngine::Rendering::CPUInstanceData_ReadOnly = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_instanceDataBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly,
                        0usize,
                    >("get_instanceDataBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_instanceDataBuffer", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUInstanceDataBuffer_ReadOnly = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_instanceDataBufferLayoutVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        i32,
                        0usize,
                    >("get_instanceDataBufferLayoutVersion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_instanceDataBufferLayoutVersion", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instanceDataBufferVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_instanceDataBufferVersion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_instanceDataBufferVersion", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_lodGroupCullingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::LODGroupCullingData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Collections::NativeList_1<
                            crate::UnityEngine::Rendering::LODGroupCullingData,
                        >,
                        0usize,
                    >("get_lodGroupCullingData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_lodGroupCullingData", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeList_1<
            crate::UnityEngine::Rendering::LODGroupCullingData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_occlusionCullingCommon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::OcclusionCullingCommon>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::OcclusionCullingCommon,
                        >,
                        0usize,
                    >("get_occlusionCullingCommon")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_occlusionCullingCommon", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::OcclusionCullingCommon,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_renderersParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderersParameters,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::RenderersParameters,
                        0usize,
                    >("get_renderersParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_renderersParameters", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderersParameters = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_resources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                        >,
                        0usize,
                    >("get_resources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_resources", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sharedInstanceData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
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
        let __cordl_ret: crate::UnityEngine::Rendering::CPUSharedInstanceData_ReadOnly = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_smallMeshScreenPercentage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_smallMeshScreenPercentage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_smallMeshScreenPercentage", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+RenderersBatchersContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderersBatchersContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderersBatchersContext")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::Rendering::RenderersBatchersContext {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+RenderersBatchersContext")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::Rendering::RenderersBatchersContext {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
