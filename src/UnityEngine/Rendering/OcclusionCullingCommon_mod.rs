#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon")]
#[repr(C)]
#[derive(Debug)]
pub struct OcclusionCullingCommon {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_DebugOcclusionTestMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_OccluderDebugViewMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_OcclusionDebugCS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub m_ClearOcclusionDebugKernel: i32,
    pub m_OccluderDepthPyramidCS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub m_OccluderDepthDownscaleKernel: i32,
    pub m_FrameIndex: i32,
    pub m_SilhouettePlaneCache: crate::UnityEngine::Rendering::SilhouettePlaneCache,
    pub m_ViewIDToIndexMap: crate::Unity::Collections::NativeParallelHashMap_2<i32, i32>,
    pub m_OccluderContextData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Rendering::OccluderContext>,
    >,
    pub m_OccluderContextSlots: crate::Unity::Collections::NativeList_1<
        crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot,
    >,
    pub m_FreeOccluderContexts: crate::Unity::Collections::NativeList_1<i32>,
    pub m_CommonShaderVariables: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::OcclusionCullingCommonShaderVariables,
    >,
    pub m_CommonConstantBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_DebugShaderVariables: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables,
    >,
    pub m_DebugConstantBuffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeBuffer>,
    pub m_ProfilingSamplerUpdateOccluders:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    pub m_ProfilingSamplerOcclusionTestOverlay:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    pub m_ProfilingSamplerOccluderOverlay:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::OcclusionCullingCommon {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon";
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
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon")]
impl std::ops::Deref for crate::UnityEngine::Rendering::OcclusionCullingCommon {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::OcclusionCullingCommon {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon {
    #[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
    pub type DebugOccluderViewData =
        crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData;
    #[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
    pub type OccluderContextSlot =
        crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot;
    #[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OccluderOverlayPassData")]
    pub type OccluderOverlayPassData =
        crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderOverlayPassData;
    #[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlayPassData")]
    pub type OcclusionTestOverlayPassData =
        crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData;
    #[cfg(
        feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlaySetupPassData"
    )]
    pub type OcclusionTestOverlaySetupPassData =
        crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlaySetupPassData;
    #[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+ShaderIDs")]
    pub type ShaderIDs = crate::UnityEngine::Rendering::OcclusionCullingCommon_ShaderIDs;
    #[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+UpdateOccludersPassData")]
    pub type UpdateOccludersPassData =
        crate::UnityEngine::Rendering::OcclusionCullingCommon_UpdateOccludersPassData;
    pub fn CreateFarDepthPyramid(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ComputeCommandBuffer>,
        occluderParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderParameters,
        >,
        occluderSubviewUpdates: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
        >,
        occluderHandles: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderHandles,
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
                            crate::UnityEngine::Rendering::OccluderParameters,
                        >,
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OccluderHandles,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "CreateFarDepthPyramid"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateFarDepthPyramid",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cmd, occluderParams, occluderSubviewUpdates, occluderHandles),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DeleteContext(
        &mut self,
        viewInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("DeleteContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DeleteContext",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn DispatchDebugClear(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ComputeCommandBuffer>,
        viewInstanceID: i32,
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
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>("DispatchDebugClear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DispatchDebugClear",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, viewInstanceID))? };
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
    pub fn GetOccluderContext(
        &mut self,
        viewInstanceID: i32,
        occluderContext: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OccluderContext,
                        >,
                    ), bool, 2usize>("GetOccluderContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetOccluderContext",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID, occluderContext))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOcclusionTestDebugOutput(
        &mut self,
        viewInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::OcclusionCullingDebugOutput>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Rendering::OcclusionCullingDebugOutput,
                        1usize,
                    >("GetOcclusionTestDebugOutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOcclusionTestDebugOutput", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::OcclusionCullingDebugOutput =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasOccluderContext(
        &mut self,
        viewInstanceID: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), bool, 1usize>("HasOccluderContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasOccluderContext",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        resources: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::GPUResidentDrawerResources,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::GPUResidentDrawerResources,
                    >), quest_hook::libil2cpp::Void, 1usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Init",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resources))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NewContext(&mut self, viewInstanceID: i32) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("NewContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "NewContext",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID))? };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareCulling(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ComputeCommandBuffer>,
        occluderCtx: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderContext,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OcclusionCullingSettings,
        >,
        subviewSettings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::InstanceOcclusionTestSubviewSettings,
        >,
        shader: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OcclusionTestComputeShader,
        >,
        useOcclusionDebug: bool,
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
                            crate::UnityEngine::Rendering::OccluderContext,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OcclusionCullingSettings,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::InstanceOcclusionTestSubviewSettings,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OcclusionTestComputeShader,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>("PrepareCulling")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareCulling",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    occluderCtx,
                    settings,
                    subviewSettings,
                    shader,
                    useOcclusionDebug,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PrepareOccluders(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        occluderParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::OccluderHandles> {
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
                            crate::UnityEngine::Rendering::OccluderParameters,
                        >,
                    ), crate::UnityEngine::Rendering::OccluderHandles, 2usize>(
                        "PrepareOccluders"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareOccluders",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::OccluderHandles =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, occluderParams))? };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDebugOccluderOverlay(
        &mut self,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugDisplayGPUResidentDrawer,
                        >,
                        crate::UnityEngine::Vector2,
                        f32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "RenderDebugOccluderOverlay"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderDebugOccluderOverlay",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    debugSettings,
                    screenPos,
                    maxHeight,
                    colorBuffer,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDebugOcclusionTestOverlay(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        debugSettings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugDisplayGPUResidentDrawer,
        >,
        viewInstanceID: i32,
        colorBuffer: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
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
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugDisplayGPUResidentDrawer,
                        >,
                        i32,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "RenderDebugOcclusionTestOverlay"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderDebugOcclusionTestOverlay",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (renderGraph, debugSettings, viewInstanceID, colorBuffer),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDebugPyramid(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ComputeCommandBuffer>,
        shader: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OcclusionTestComputeShader,
        >,
        kernel: i32,
        occluderHandles: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderHandles,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ComputeCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OcclusionTestComputeShader,
                        >,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OccluderHandles,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetDebugPyramid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDebugPyramid",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, shader, kernel, occluderHandles))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDepthPyramid(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ComputeCommandBuffer>,
        shader: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OcclusionTestComputeShader,
        >,
        kernel: i32,
        occluderHandles: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderHandles,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ComputeCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OcclusionTestComputeShader,
                        >,
                        i32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::OccluderHandles,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetDepthPyramid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetDepthPyramid",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, shader, kernel, occluderHandles))?
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
    pub fn UpdateInstanceOccluders(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        occluderParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderParameters,
        >,
        occluderSubviewUpdates: crate::System::ReadOnlySpan_1<
            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
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
                            crate::UnityEngine::Rendering::OccluderParameters,
                        >,
                        crate::System::ReadOnlySpan_1<
                            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
                        >,
                    ), bool, 3usize>("UpdateInstanceOccluders")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateInstanceOccluders",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderGraph, occluderParams, occluderSubviewUpdates))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateOccluderStats(
        &mut self,
        debugStats: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugRendererBatcherStats,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::DebugRendererBatcherStats,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "UpdateOccluderStats"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateOccluderStats",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (debugStats))? };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSilhouettePlanes(
        &mut self,
        viewInstanceID: i32,
        planes: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Plane>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UpdateSilhouettePlanes"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateSilhouettePlanes",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (viewInstanceID, planes))? };
        Ok(__cordl_ret.into())
    }
    pub fn UseOcclusionDebug(
        occluderCtx: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::OccluderContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::OccluderContext,
                    >), bool, 1usize>("UseOcclusionDebug")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UseOcclusionDebug",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (occluderCtx))? };
        Ok(__cordl_ret.into())
    }
    pub fn _RenderDebugOcclusionTestOverlay_b__29_1(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData,
        >,
        ctx: crate::UnityEngine::Rendering::RenderGraphModule::RasterGraphContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::RasterGraphContext,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("<RenderDebugOcclusionTestOverlay>b__29_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<RenderDebugOcclusionTestOverlay>b__29_1", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (data, ctx))? };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::OcclusionCullingCommon {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Rendering::OcclusionCullingCommon {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Rendering::OcclusionCullingCommon {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommon_DebugOccluderViewData {
    pub passIndex: i32,
    pub viewport: crate::UnityEngine::Rect,
    pub valid: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon/DebugOccluderViewData";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+DebugOccluderViewData")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon_DebugOccluderViewData {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct OcclusionCullingCommon_OccluderContextSlot {
    pub valid: bool,
    pub lastUsedFrameIndex: i32,
    pub viewInstanceID: i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon/OccluderContextSlot";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OccluderContextSlot")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderContextSlot {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderOverlayPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct OcclusionCullingCommon_OccluderOverlayPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub debugMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub occluderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub viewport: crate::UnityEngine::Rect,
    pub passIndex: i32,
    pub validRange: crate::UnityEngine::Vector2,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderOverlayPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderOverlayPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon/OccluderOverlayPassData";
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
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OccluderOverlayPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderOverlayPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OccluderOverlayPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderOverlayPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OccluderOverlayPassData")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderOverlayPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OccluderOverlayPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OccluderOverlayPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlayPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct OcclusionCullingCommon_OcclusionTestOverlayPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub debugPyramid: crate::UnityEngine::Rendering::RenderGraphModule::BufferHandle,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlayPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon/OcclusionTestOverlayPassData";
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
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlayPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlayPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlayPassData")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlayPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlayPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlaySetupPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct OcclusionCullingCommon_OcclusionTestOverlaySetupPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub cb: crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlaySetupPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlaySetupPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon/OcclusionTestOverlaySetupPassData";
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
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlaySetupPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlaySetupPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlaySetupPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlaySetupPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlaySetupPassData")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlaySetupPassData {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+OcclusionTestOverlaySetupPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_OcclusionTestOverlaySetupPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+ShaderIDs")]
#[repr(C)]
#[derive(Debug)]
pub struct OcclusionCullingCommon_ShaderIDs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+ShaderIDs")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_ShaderIDs
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon/ShaderIDs";
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
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+ShaderIDs")]
impl std::ops::Deref for crate::UnityEngine::Rendering::OcclusionCullingCommon_ShaderIDs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+ShaderIDs")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::OcclusionCullingCommon_ShaderIDs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+ShaderIDs")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon_ShaderIDs {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+ShaderIDs")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_ShaderIDs
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+UpdateOccludersPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct OcclusionCullingCommon_UpdateOccludersPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub occluderParams: crate::UnityEngine::Rendering::OccluderParameters,
    pub occluderSubviewUpdates: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Rendering::OccluderSubviewUpdate,
        >,
    >,
    pub occluderHandles: crate::UnityEngine::Rendering::OccluderHandles,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+UpdateOccludersPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_UpdateOccludersPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingCommon/UpdateOccludersPassData";
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
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+UpdateOccludersPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_UpdateOccludersPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+UpdateOccludersPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_UpdateOccludersPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingCommon+UpdateOccludersPassData")]
impl crate::UnityEngine::Rendering::OcclusionCullingCommon_UpdateOccludersPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingCommon+UpdateOccludersPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::OcclusionCullingCommon_UpdateOccludersPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
