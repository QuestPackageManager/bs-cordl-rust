#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NativePassCompiler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub graph: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo,
    pub contextData: Blacklisted,
    pub defaultContextData: Blacklisted,
    pub previousCommandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub toVisitPassIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Stack_1<i32>,
    >,
    pub m_CompilationCache: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RenderGraphCompilationCache,
    >,
    pub m_BeginRenderPassAttachments: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::AttachmentDescriptor,
    >,
    pub m_Disposed: bool,
    pub graphPassNamesForDebug: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::DynamicArray_1<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::Name,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "NativePassCompiler";
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
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
impl
    crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler
{
    pub const ArbitraryMaxNbMergedPasses: i32 = 16i32;
    pub const k_EstimatedPassCount: i32 = 100i32;
    pub const k_MaxSubpass: i32 = 8i32;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+NativeCompilerProfileId"
    )]
    pub type NativeCompilerProfileId = crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_NativeCompilerProfileId;
    #[cfg(
        feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
    )]
    pub type RenderGraphInputInfo = crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo;
    pub fn BuildGraph(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("BuildGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "BuildGraph",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Cleanup(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Cleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Cleanup",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
        clearContextData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (clearContextData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Compile(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Compile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Compile",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resources))? };
        Ok(__cordl_ret.into())
    }
    pub fn CullUnusedRenderPasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "CullUnusedRenderPasses",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CullUnusedRenderPasses",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DetectMemoryLessResources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "DetectMemoryLessResources",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DetectMemoryLessResources",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn DetermineLoadStoreActions(
        &mut self,
        nativePass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("DetermineLoadStoreActions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DetermineLoadStoreActions", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nativePass))? };
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
    pub fn ExecuteBeginRenderPass(
        &mut self,
        rgContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        nativePass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExecuteBeginRenderPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteBeginRenderPass", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rgContext, resources, nativePass))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCreateRessource(
        &mut self,
        rgContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        pass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExecuteCreateRessource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteCreateRessource", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rgContext, resources, pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteDestroyResource(
        &mut self,
        rgContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        pass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExecuteDestroyResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteDestroyResource", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rgContext, resources, pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteGraph(
        &mut self,
        rgContext: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        passes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<
                                        quest_hook::libil2cpp::Gc<
                                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExecuteGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteGraph", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rgContext, resources, passes))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteGraphNode(
        &mut self,
        rgContext: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
            >,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::RenderGraphModule::InternalRenderGraphContext,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExecuteGraphNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteGraphNode", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (rgContext, resources, pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Finalize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Finalize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FindResourceUsageRanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "FindResourceUsageRanges",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FindResourceUsageRanges",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateNativeCompilerDebugData(
        &mut self,
        debugData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph_DebugData,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "GenerateNativeCompilerDebugData"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenerateNativeCompilerDebugData",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (debugData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        renderPasses: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                >,
            >,
        >,
        disableCulling: bool,
        debugName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        useCompilationCaching: bool,
        graphHash: i32,
        frameIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                                    >,
                                >,
                            >,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            i32,
                            i32,
                        ),
                        bool,
                        7usize,
                    >("Initialize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Initialize", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    resources,
                    renderPasses,
                    disableCulling,
                    debugName,
                    useCompilationCaching,
                    graphHash,
                    frameIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InjectSpaces(
        camelCaseString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("InjectSpaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InjectSpaces", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (camelCaseString))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsGlobalTextureInPass(
        pass: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
        >,
        handle: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
                        >,
                        crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                    ), bool, 2usize>("IsGlobalTextureInPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsGlobalTextureInPass",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (pass, handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSameNativeSubPass(
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SubPassDescriptor>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::SubPassDescriptor>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SubPassDescriptor,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::SubPassDescriptor,
                        >,
                    ), bool, 2usize>("IsSameNativeSubPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsSameNativeSubPass",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeAttachmentInfo(
        ctx: Blacklisted,
        nativePass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        >,
        attachmentIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo,
        >,
    >{
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            Blacklisted,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo,
                        >,
                        3usize,
                    >("MakeAttachmentInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MakeAttachmentInfo", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassInfo_NRPInfo_PassData_DebugData_RenderGraph_AttachmentInfo,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (ctx, nativePass, attachmentIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakePassBreakInfoMessage(
        ctx: Blacklisted,
        nativePass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            Blacklisted,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        2usize,
                    >("MakePassBreakInfoMessage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MakePassBreakInfoMessage", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (ctx, nativePass))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakePassMergeMessage(
        ctx: Blacklisted,
        pass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        >,
        prevPass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
        >,
        mergeResult: crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            Blacklisted,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassData,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::PassBreakAudit,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        4usize,
                    >("MakePassMergeMessage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MakePassMergeMessage", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (ctx, pass, prevPass, mergeResult))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        cache: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::RenderGraphCompilationCache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cache))?;
        Ok(__cordl_object.into())
    }
    pub fn PrepareNativeRenderPasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "PrepareNativeRenderPasses",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareNativeRenderPasses",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SetPassStatesForNativePass(
        &mut self,
        nativePassId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(
                        "SetPassStatesForNativePass",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetPassStatesForNativePass",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (nativePassId))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetRandomWriteTarget(
        &mut self,
        cmd: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        index: i32,
        resource: crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
        preserveCounterValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::CommandBuffer,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            i32,
                            crate::UnityEngine::Rendering::RenderGraphModule::ResourceHandle,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetRandomWriteTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetRandomWriteTarget", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cmd, resources, index, resource, preserveCounterValue),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupContextData(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetupContextData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupContextData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resources))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryMergeNativePasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("TryMergeNativePasses")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryMergeNativePasses",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateAttachmentRenderTarget(
        &mut self,
        attRenderTargetInfo: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
        >,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
        >,
        nativePassWidth: i32,
        nativePassHeight: i32,
        nativePassMSAASamples: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderTargetInfo,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
                            >,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ValidateAttachmentRenderTarget")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateAttachmentRenderTarget", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    attRenderTargetInfo,
                    resources,
                    nativePassWidth,
                    nativePassHeight,
                    nativePassMSAASamples,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateNativePass(
        &mut self,
        nativePass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
        >,
        width: i32,
        height: i32,
        depth: i32,
        samples: i32,
        attachmentCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassData,
                            >,
                            i32,
                            i32,
                            i32,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("ValidateNativePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ValidateNativePass", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (nativePass, width, height, depth, samples, attachmentCount),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        cache: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::RenderGraphCompilationCache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::RenderGraphCompilationCache,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cache))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+NativeCompilerProfileId"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum NativePassCompiler_NativeCompilerProfileId {
    #[default]
    NRPRGComp_BuildGraph = 2i32,
    NRPRGComp_CullNodes = 3i32,
    NRPRGComp_DetectMemorylessResources = 6i32,
    NRPRGComp_ExecuteBeginRenderpassCommand = 8i32,
    NRPRGComp_ExecuteCreateResources = 7i32,
    NRPRGComp_ExecuteDestroyResources = 9i32,
    NRPRGComp_FindResourceUsageRanges = 5i32,
    NRPRGComp_PrepareNativePass = 0i32,
    NRPRGComp_SetupContextData = 1i32,
    NRPRGComp_TryMergeNativePasses = 4i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+NativeCompilerProfileId"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_NativeCompilerProfileId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "NativePassCompiler/NativeCompilerProfileId";
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+NativeCompilerProfileId"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_NativeCompilerProfileId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+NativeCompilerProfileId"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_NativeCompilerProfileId {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+NativeCompilerProfileId"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_NativeCompilerProfileId {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+NativeCompilerProfileId"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_NativeCompilerProfileId {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct NativePassCompiler_RenderGraphInputInfo {
    pub m_ResourcesForDebugOnly: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphResourceRegistry,
    >,
    pub m_RenderPasses: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraphPass,
            >,
        >,
    >,
    pub debugName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub disableCulling: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.RenderGraphModule.NativeRenderPassCompiler";
    const CLASS_NAME: &'static str = "NativePassCompiler/RenderGraphInputInfo";
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo {
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
    feature = "cordl_class_UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+RenderGraphModule+NativeRenderPassCompiler+NativePassCompiler+RenderGraphInputInfo"
)]
impl crate::UnityEngine::Rendering::RenderGraphModule::NativeRenderPassCompiler::NativePassCompiler_RenderGraphInputInfo {}
