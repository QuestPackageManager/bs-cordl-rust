#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsPass")]
#[repr(C)]
#[derive(Debug)]
pub struct DrawObjectsPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
    pub m_FilteringSettings: crate::UnityEngine::Rendering::FilteringSettings,
    pub m_RenderStateBlock: crate::UnityEngine::Rendering::RenderStateBlock,
    pub m_ShaderTagIdList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::Rendering::ShaderTagId,
        >,
    >,
    pub m_IsOpaque: bool,
    pub m_IsActiveTargetBackBuffer: bool,
    pub m_ShouldTransparentsReceiveShadows: bool,
    pub m_PassData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsPass")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "DrawObjectsPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsPass")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass {
    type Target = crate::UnityEngine::Rendering::Universal::ScriptableRenderPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsPass")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsPass")]
impl crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass {
    #[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsPass+PassData")]
    pub type PassData = crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData;
    pub fn Execute(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingData,
                            >,
                        ),
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
            cordl_method_info.invoke_unchecked(self, (context, renderingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecutePass(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
        >,
        rendererList: crate::UnityEngine::Rendering::RendererList,
        objectsWithErrorRendererList: crate::UnityEngine::Rendering::RendererList,
        yFlip: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
                            >,
                            crate::UnityEngine::Rendering::RendererList,
                            crate::UnityEngine::Rendering::RendererList,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ExecutePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecutePass", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (cmd, data, rendererList, objectsWithErrorRendererList, yFlip),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
        shaderTagIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::ShaderTagId,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            bool,
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                            crate::UnityEngine::Rendering::RenderQueueRange,
                            crate::UnityEngine::LayerMask,
                            crate::UnityEngine::Rendering::StencilState,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::ShaderTagId,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Init",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        opaque,
                        evt,
                        renderQueueRange,
                        layerMask,
                        stencilState,
                        stencilReference,
                        shaderTagIds,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitPassData(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
            >,
        >,
        batchLayerMask: u32,
        isActiveTargetBackBuffer: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
                                >,
                            >,
                            u32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("InitPassData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitPassData", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cameraData, passData, batchLayerMask, isActiveTargetBackBuffer),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitRendererLists(
        &mut self,
        renderingData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
            >,
        >,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        useRenderGraph: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalLightData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
                                >,
                            >,
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("InitRendererLists")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitRendererLists", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        renderingData,
                        cameraData,
                        lightData,
                        passData,
                        context,
                        renderGraph,
                        useRenderGraph,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString_Il2CppArray__cordl_bool_RenderPassEvent_RenderQueueRange_LayerMask_StencilState_i32_0(
        profilerTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shaderTagIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::ShaderTagId,
            >,
        >,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    profilerTag,
                    shaderTagIds,
                    opaque,
                    evt,
                    renderQueueRange,
                    layerMask,
                    stencilState,
                    stencilReference,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString__cordl_bool_RenderPassEvent_RenderQueueRange_LayerMask_StencilState_i32_1(
        profilerTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    profilerTag,
                    opaque,
                    evt,
                    renderQueueRange,
                    layerMask,
                    stencilState,
                    stencilReference,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New_URPProfileId__cordl_bool_RenderPassEvent_RenderQueueRange_LayerMask_StencilState_i32_2(
        profileId: crate::UnityEngine::Rendering::Universal::URPProfileId,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    profileId,
                    opaque,
                    evt,
                    renderQueueRange,
                    layerMask,
                    stencilState,
                    stencilReference,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Render(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ContextContainer,
        >,
        colorTarget: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        depthTarget: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        mainShadowsTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        additionalShadowsTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        batchLayerMask: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ContextContainer,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            u32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("Render")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Render",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        renderGraph,
                        frameData,
                        colorTarget,
                        depthTarget,
                        mainShadowsTexture,
                        additionalShadowsTexture,
                        batchLayerMask,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_Il2CppArray__cordl_bool_RenderPassEvent_RenderQueueRange_LayerMask_StencilState_i32_0(
        &mut self,
        profilerTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        shaderTagIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::ShaderTagId,
            >,
        >,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::Rendering::ShaderTagId,
                                >,
                            >,
                            bool,
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                            crate::UnityEngine::Rendering::RenderQueueRange,
                            crate::UnityEngine::LayerMask,
                            crate::UnityEngine::Rendering::StencilState,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        profilerTag,
                        shaderTagIds,
                        opaque,
                        evt,
                        renderQueueRange,
                        layerMask,
                        stencilState,
                        stencilReference,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString__cordl_bool_RenderPassEvent_RenderQueueRange_LayerMask_StencilState_i32_1(
        &mut self,
        profilerTag: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                            crate::UnityEngine::Rendering::RenderQueueRange,
                            crate::UnityEngine::LayerMask,
                            crate::UnityEngine::Rendering::StencilState,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        profilerTag,
                        opaque,
                        evt,
                        renderQueueRange,
                        layerMask,
                        stencilState,
                        stencilReference,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_URPProfileId__cordl_bool_RenderPassEvent_RenderQueueRange_LayerMask_StencilState_i32_2(
        &mut self,
        profileId: crate::UnityEngine::Rendering::Universal::URPProfileId,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::Universal::URPProfileId,
                            bool,
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                            crate::UnityEngine::Rendering::RenderQueueRange,
                            crate::UnityEngine::LayerMask,
                            crate::UnityEngine::Rendering::StencilState,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        profileId,
                        opaque,
                        evt,
                        renderQueueRange,
                        layerMask,
                        stencilState,
                        stencilReference,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsPass")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsPass+PassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DrawObjectsPass_PassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub albedoHdl: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub depthHdl: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub cameraData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::UniversalCameraData,
    >,
    pub isOpaque: bool,
    pub shouldTransparentsReceiveShadows: bool,
    pub batchLayerMask: u32,
    pub isActiveTargetBackBuffer: bool,
    pub rendererListHdl: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    pub objectsWithErrorRendererListHdl: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
    pub debugRendererLists: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::DebugRendererLists,
    >,
    pub rendererList: crate::UnityEngine::Rendering::RendererList,
    pub objectsWithErrorRendererList: crate::UnityEngine::Rendering::RendererList,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsPass+PassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "DrawObjectsPass/PassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsPass+PassData")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsPass+PassData")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsPass+PassData")]
impl crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsPass+PassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
