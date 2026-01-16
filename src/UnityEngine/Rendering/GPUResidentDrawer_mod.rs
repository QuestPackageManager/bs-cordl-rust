#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer")]
#[repr(C)]
#[derive(Debug)]
pub struct GPUResidentDrawer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ContextIntPtr: crate::System::IntPtr,
    pub m_Settings: crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
    pub m_GPUDrivenProcessor: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::GPUDrivenProcessor,
    >,
    pub m_BatchersContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderersBatchersContext,
    >,
    pub m_Batcher: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::GPUResidentBatcher,
    >,
    pub m_Dispatcher: quest_hook::libil2cpp::Gc<crate::UnityEngine::ObjectDispatcher>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUResidentDrawer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUResidentDrawer";
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
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::GPUResidentDrawer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::GPUResidentDrawer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer")]
impl crate::UnityEngine::Rendering::GPUResidentDrawer {
    #[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob")]
    pub type ClassifyMaterialsJob = crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob;
    #[cfg(
        feature = "UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
    )]
    pub type FindRenderersFromMaterialJob = crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob;
    #[cfg(
        feature = "UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
    )]
    pub type FindUnsupportedRenderersJob = crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob;
    #[cfg(
        feature = "UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
    )]
    pub type GetMaterialsWithChangedPackedMaterialJob = crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob;
    #[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+Strings")]
    pub type Strings = crate::UnityEngine::Rendering::GPUResidentDrawer_Strings;
    pub fn AppendNewInstance(
        &mut self,
        rendererGroupID: i32,
        instanceTransform: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::InstanceHandle> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Matrix4x4,
                            >,
                        ),
                        crate::UnityEngine::Rendering::InstanceHandle,
                        2usize,
                    >("AppendNewInstance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AppendNewInstance", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::InstanceHandle = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rendererGroupID, instanceTransform))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClassifyMaterials(
        &mut self,
        materials: crate::Unity::Collections::NativeArray_1<i32>,
        unsupportedMaterials: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeList_1<i32>,
        >,
        supportedMaterials: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeList_1<i32>,
        >,
        supportedPackedMaterialDatas: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Collections::NativeList_1<
                crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
            >,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeList_1<i32>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeList_1<i32>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Unity::Collections::NativeList_1<
                                    crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
                                >,
                            >,
                            crate::Unity::Collections::Allocator,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ClassifyMaterials")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClassifyMaterials", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        materials,
                        unsupportedMaterials,
                        supportedMaterials,
                        supportedPackedMaterialDatas,
                        allocator,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CleanUp() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CleanUp")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "CleanUp",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
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
    pub fn FindRenderersFromMaterials(
        &mut self,
        sortedExcludeRenderers: crate::Unity::Collections::NativeArray_1<i32>,
        materials: crate::Unity::Collections::NativeHashSet_1<i32>,
        rendererListAllocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeList_1<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeHashSet_1<i32>,
                            crate::Unity::Collections::Allocator,
                        ),
                        crate::Unity::Collections::NativeList_1<i32>,
                        3usize,
                    >("FindRenderersFromMaterials")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindRenderersFromMaterials", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeList_1<i32> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (sortedExcludeRenderers, materials, rendererListAllocator),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FindUnsupportedRenderers(
        &mut self,
        unsupportedMaterials: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeList_1<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        crate::Unity::Collections::NativeList_1<i32>,
                        1usize,
                    >("FindUnsupportedRenderers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FindUnsupportedRenderers", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeList_1<i32> = unsafe {
            cordl_method_info.invoke_unchecked(self, (unsupportedMaterials))?
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
        rendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
        unsupportedRendererGroupIDs: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("FreeRendererGroupInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FreeRendererGroupInstances", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rendererGroupIDs, unsupportedRendererGroupIDs))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetDebugStats() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
                        >,
                        0usize,
                    >("GetDebugStats")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDebugStats", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlobalSettingsFromRPAsset() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
                        0usize,
                    >("GetGlobalSettingsFromRPAsset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGlobalSettingsFromRPAsset", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUResidentDrawerSettings = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaterialsWithChangedPackedMaterial(
        &mut self,
        materials: crate::Unity::Collections::NativeArray_1<i32>,
        packedMaterialDatas: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
        >,
        allocator: crate::Unity::Collections::Allocator,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeHashSet_1<i32>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
                            >,
                            crate::Unity::Collections::Allocator,
                        ),
                        crate::Unity::Collections::NativeHashSet_1<i32>,
                        3usize,
                    >("GetMaterialsWithChangedPackedMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMaterialsWithChangedPackedMaterial", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Collections::NativeHashSet_1<i32> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (materials, packedMaterialDatas, allocator))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertIntoPlayerLoop(
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
                    >("InsertIntoPlayerLoop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertIntoPlayerLoop", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InstanceOcclusionTest(
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OcclusionCullingSettings,
        >,
        subviewOcclusionTests: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::SubviewOcclusionTest,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::OcclusionCullingSettings,
                            >,
                            crate::System::ReadOnlySpan_1<
                                crate::UnityEngine::Rendering::SubviewOcclusionTest,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InstanceOcclusionTest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstanceOcclusionTest", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (renderGraph, settings, subviewOcclusionTests))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsEnabled() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsEnabled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsForcedOnViaCommandLine() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsForcedOnViaCommandLine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsForcedOnViaCommandLine", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsGPUResidentDrawerSupportedBySRP(
        settings: crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
        message: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        severity: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::LogType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::LogType>,
                        ),
                        bool,
                        3usize,
                    >("IsGPUResidentDrawerSupportedBySRP")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsGPUResidentDrawerSupportedBySRP", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (settings, message, severity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsInstanceOcclusionCullingEnabled() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        bool,
                        0usize,
                    >("IsInstanceOcclusionCullingEnabled")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsInstanceOcclusionCullingEnabled", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsOcclusionForcedOnViaCommandLine() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        bool,
                        0usize,
                    >("IsOcclusionForcedOnViaCommandLine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsOcclusionForcedOnViaCommandLine", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsProjectSupported_0() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("IsProjectSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsProjectSupported", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn IsProjectSupported_ByRefMut_ByRefMut1(
        message: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        severity: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::LogType>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppString,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::LogType>,
                        ),
                        bool,
                        2usize,
                    >("IsProjectSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsProjectSupported", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (message, severity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LogMessage(
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        severity: crate::UnityEngine::LogType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::UnityEngine::LogType,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("LogMessage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LogMessage", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (message, severity))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        settings: crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
        maxInstanceCount: i32,
        maxTreeInstanceCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (settings, maxInstanceCount, maxTreeInstanceCount))?;
        Ok(__cordl_object.into())
    }
    pub fn OnBeginCameraRendering(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnBeginCameraRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnBeginCameraRendering", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnBeginContextRendering(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameras: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnBeginContextRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnBeginContextRendering", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, cameras))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndCameraRendering(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnEndCameraRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEndCameraRendering", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, camera))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEndContextRendering(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cameras: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnEndContextRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEndContextRendering", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, cameras))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnSceneLoaded(
        &mut self,
        scene: crate::UnityEngine::SceneManagement::Scene,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::SceneManagement::Scene,
                            crate::UnityEngine::SceneManagement::LoadSceneMode,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnSceneLoaded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSceneLoaded", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (scene, mode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnSetupAmbientProbe() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("OnSetupAmbientProbe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnSetupAmbientProbe", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PostCullBeginCameraRendering(
        context: crate::UnityEngine::Rendering::RenderRequestBatcherContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
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
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PostPostLateUpdate(
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
                    >("PostPostLateUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PostPostLateUpdate", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PostPostLateUpdateStatic() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("PostPostLateUpdateStatic")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PostPostLateUpdateStatic", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessLODGroups(
        &mut self,
        changedID: crate::Unity::Collections::NativeArray_1<i32>,
        destroyed: crate::Unity::Collections::NativeArray_1<i32>,
        transformedID: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessLODGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessLODGroups", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (changedID, destroyed, transformedID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMaterials(
        &mut self,
        destroyedID: crate::Unity::Collections::NativeArray_1<i32>,
        unsupportedMaterials: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessMaterials")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessMaterials", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (destroyedID, unsupportedMaterials))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMeshes(
        &mut self,
        destroyedID: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Collections::NativeArray_1<i32>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ProcessMeshes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessMeshes", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (destroyedID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessRendererMaterialChanges(
        &mut self,
        excludedRenderers: crate::Unity::Collections::NativeArray_1<i32>,
        changedMaterials: crate::Unity::Collections::NativeArray_1<i32>,
        changedPackedMaterialDatas: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<i32>,
                            crate::Unity::Collections::NativeArray_1<
                                crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ProcessRendererMaterialChanges")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessRendererMaterialChanges", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (excludedRenderers, changedMaterials, changedPackedMaterialDatas),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessRenderers(
        &mut self,
        rendererChanges: crate::UnityEngine::TypeDispatchData,
        unsupportedRenderers: crate::Unity::Collections::NativeArray_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::TypeDispatchData,
                            crate::Unity::Collections::NativeArray_1<i32>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ProcessRenderers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ProcessRenderers", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (rendererChanges, unsupportedRenderers))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Recreate(
        settings: crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::GPUResidentDrawerSettings),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Recreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Recreate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (settings))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Reinitialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("Reinitialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Reinitialize", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReinitializeIfNeeded() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ReinitializeIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReinitializeIfNeeded", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromPlayerLoop(
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
                    >("RemoveFromPlayerLoop")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RemoveFromPlayerLoop", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDebugOccluderOverlay(
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        debugSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugDisplayGPUResidentDrawer,
        >,
        screenPos: crate::UnityEngine::Vector2,
        maxHeight: f32,
        colorBuffer: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugDisplayGPUResidentDrawer,
                            >,
                            crate::UnityEngine::Vector2,
                            f32,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("RenderDebugOccluderOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderDebugOccluderOverlay", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (renderGraph, debugSettings, screenPos, maxHeight, colorBuffer),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDebugOcclusionTestOverlay(
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        debugSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugDisplayGPUResidentDrawer,
        >,
        viewInstanceID: i32,
        colorBuffer: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::DebugDisplayGPUResidentDrawer,
                            >,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("RenderDebugOcclusionTestOverlay")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderDebugOcclusionTestOverlay", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (renderGraph, debugSettings, viewInstanceID, colorBuffer),
                )?
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
    pub fn TransformInstances(
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
                    >("TransformInstances")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TransformInstances", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (instances, localToWorldMatrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInstanceOccluders(
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        occluderParameters: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderParameters,
        >,
        occluderSubviewUpdates: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::OccluderParameters,
                            >,
                            crate::System::ReadOnlySpan_1<
                                crate::UnityEngine::Rendering::OccluderSubviewUpdate,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("UpdateInstanceOccluders")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateInstanceOccluders", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (renderGraph, occluderParameters, occluderSubviewUpdates),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        settings: crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
        maxInstanceCount: i32,
        maxTreeInstanceCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
                            i32,
                            i32,
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
                .invoke_unchecked(
                    self,
                    (settings, maxInstanceCount, maxTreeInstanceCount),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_batcher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::GPUResidentBatcher>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUResidentBatcher,
                        >,
                        0usize,
                    >("get_batcher")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_batcher", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentBatcher,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::GPUResidentDrawer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::GPUResidentDrawer,
                        >,
                        0usize,
                    >("get_instance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_instance", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawer,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::GPUResidentDrawerSettings,
                        0usize,
                    >("get_settings")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_settings", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::GPUResidentDrawerSettings = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::GPUResidentDrawer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUResidentDrawer_ClassifyMaterialsJob {
    pub batchMaterialHash: crate::Unity::Collections::NativeParallelHashMap_2_ReadOnly<
        i32,
        crate::UnityEngine::Rendering::BatchMaterialID,
    >,
    pub materialIDs: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub supportedMaterialIDs: crate::Unity::Collections::NativeList_1<i32>,
    pub unsupportedMaterialIDs: crate::Unity::Collections::NativeList_1<i32>,
    pub supportedPackedMaterialDatas: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUResidentDrawer/ClassifyMaterialsJob";
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob")]
impl crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
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
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+ClassifyMaterialsJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::GPUResidentDrawer_ClassifyMaterialsJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUResidentDrawer_FindRenderersFromMaterialJob {
    pub materialIDs: crate::Unity::Collections::NativeHashSet_1_ReadOnly<i32>,
    pub materialIDArrays: crate::Unity::Collections::NativeArray_1_ReadOnly<
        crate::UnityEngine::Rendering::SmallIntegerArray,
    >,
    pub rendererGroupIDs: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub sortedExcludeRendererIDs: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub selectedRenderGroups: crate::Unity::Collections::NativeList_1_ParallelWriter<
        i32,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUResidentDrawer/FindRenderersFromMaterialJob";
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob")]
impl crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
    pub const k_BatchSize: i32 = 128i32;
    pub fn Execute(
        &mut self,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (startIndex, count))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelForBatch>
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+FindRenderersFromMaterialJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelForBatch>
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindRenderersFromMaterialJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelForBatch {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUResidentDrawer_FindUnsupportedRenderersJob {
    pub unsupportedMaterials: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub materialIDArrays: crate::Unity::Collections::NativeArray_1_ReadOnly<
        crate::UnityEngine::Rendering::SmallIntegerArray,
    >,
    pub rendererGroups: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub unsupportedRenderers: crate::Unity::Collections::NativeList_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUResidentDrawer/FindUnsupportedRenderersJob";
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob")]
impl crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
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
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+FindUnsupportedRenderersJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::GPUResidentDrawer_FindUnsupportedRenderersJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
    pub materialIDs: crate::Unity::Collections::NativeArray_1_ReadOnly<i32>,
    pub packedMaterialDatas: crate::Unity::Collections::NativeArray_1_ReadOnly<
        crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
    >,
    pub packedMaterialHash: crate::Unity::Collections::NativeParallelHashMap_2_ReadOnly<
        i32,
        crate::UnityEngine::Rendering::GPUDrivenPackedMaterialData,
    >,
    pub filteredMaterials: crate::Unity::Collections::NativeHashSet_1<i32>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUResidentDrawer/GetMaterialsWithChangedPackedMaterialJob";
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
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
    feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
impl crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
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
    feature = "UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+GPUResidentDrawer+GetMaterialsWithChangedPackedMaterialJob"
)]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::UnityEngine::Rendering::GPUResidentDrawer_GetMaterialsWithChangedPackedMaterialJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+Strings")]
#[repr(C)]
#[derive(Debug)]
pub struct GPUResidentDrawer_Strings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+Strings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::GPUResidentDrawer_Strings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "GPUResidentDrawer/Strings";
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
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+Strings")]
impl std::ops::Deref for crate::UnityEngine::Rendering::GPUResidentDrawer_Strings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+Strings")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::GPUResidentDrawer_Strings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+GPUResidentDrawer+Strings")]
impl crate::UnityEngine::Rendering::GPUResidentDrawer_Strings {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+GPUResidentDrawer+Strings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::GPUResidentDrawer_Strings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
