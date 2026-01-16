#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass")]
#[repr(C)]
#[derive(Debug)]
pub struct MainLightShadowCasterPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
    pub m_MainLightShadowmapTexture:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub renderTargetWidth: i32,
    pub renderTargetHeight: i32,
    pub m_ShadowCasterCascadesCount: i32,
    pub m_CreateEmptyShadowmap: bool,
    pub m_EmptyShadowmapNeedsClear: bool,
    pub m_CascadeBorder: f32,
    pub m_MaxShadowDistanceSq: f32,
    pub m_PassData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData,
    >,
    pub m_EmptyMainLightShadowmapTexture:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_MainLightShadowDescriptor: crate::UnityEngine::RenderTextureDescriptor,
    pub m_CascadeSplitDistances:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>>,
    pub m_MainLightShadowMatrices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    >,
    pub m_ProfilingSetupSampler:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProfilingSampler>,
    pub m_CascadeSlices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "MainLightShadowCasterPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass
{
    type Target = crate::UnityEngine::Rendering::Universal::ScriptableRenderPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass")]
impl crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass {
    pub const k_EmptyMainLightShadowMapTextureName: &'static str =
        "_EmptyMainLightShadowmapTexture";
    pub const k_EmptyShadowMapDimensions: i32 = 1i32;
    pub const k_MainLightShadowMapTextureName: &'static str = "_MainLightShadowmapTexture";
    pub const k_MaxCascades: i32 = 4i32;
    pub const k_ShadowmapBufferBits: i32 = 16i32;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+MainLightShadowConstantBuffer"
    )]
    pub type MainLightShadowConstantBuffer = crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_MainLightShadowConstantBuffer;
    #[cfg(feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+PassData")]
    pub type PassData =
        crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData;
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Clear")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Clear",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Configure(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cameraTextureDescriptor: crate::UnityEngine::RenderTextureDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::RenderTextureDescriptor,
                    ), quest_hook::libil2cpp::Void, 2usize>("Configure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Configure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, cameraTextureDescriptor))? };
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
    pub fn Execute(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
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
            unsafe { cordl_method_info.invoke_unchecked(self, (context, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitPassData(
        &mut self,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData,
            >,
        >,
        renderingData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
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
                                    crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalLightData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InitPassData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitPassData", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (passData, renderingData, cameraData, lightData, shadowData),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitRendererLists(
        &mut self,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData,
            >,
        >,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        useRenderGraph: bool,
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
                                    crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData,
                                >,
                            >,
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("InitRendererLists")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitRendererLists", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (passData, context, renderGraph, useRenderGraph))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (evt))?;
        Ok(__cordl_object.into())
    }
    pub fn Render(
        &mut self,
        graph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 2usize>(
                        "Render",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Render",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (graph, frameData))? };
        Ok(__cordl_ret.into())
    }
    pub fn RenderMainLightCascadeShadowmap(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData,
            >,
        >,
        isRenderGraph: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("RenderMainLightCascadeShadowmap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderMainLightCascadeShadowmap", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, data, isRenderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetShadowParamsForEmptyShadowmap(
        rasterCommandBuffer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetShadowParamsForEmptyShadowmap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetShadowParamsForEmptyShadowmap", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (rasterCommandBuffer))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupForEmptyRendering(
        &mut self,
        stripShadowsOffVariants: bool,
        light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        bool,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                        >,
                    ), bool, 4usize>("SetupForEmptyRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupForEmptyRendering",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (stripShadowsOffVariants, light, cameraData, shadowData),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupMainLightShadowReceiverConstants(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        shadowLight: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::VisibleLight>,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::VisibleLight,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "SetupMainLightShadowReceiverConstants"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupMainLightShadowReceiverConstants",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, shadowLight, shadowData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Setup_ByRefMut0(
        &mut self,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::Universal::RenderingData,
                    >), bool, 1usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Setup_UniversalRenderingData_UniversalCameraData_UniversalLightData_UniversalShadowData1(
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
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalLightData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                        >,
                    ), bool, 4usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderingData, cameraData, lightData, shadowData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTextureDescriptorIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "UpdateTextureDescriptorIfNeeded",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateTextureDescriptorIfNeeded",
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
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::Universal::RenderPassEvent),
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
            unsafe { cordl_method_info.invoke_unchecked(self, (evt))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+MainLightShadowConstantBuffer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct MainLightShadowCasterPass_MainLightShadowConstantBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+MainLightShadowConstantBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_MainLightShadowConstantBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "MainLightShadowCasterPass/MainLightShadowConstantBuffer";
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
    feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+MainLightShadowConstantBuffer"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_MainLightShadowConstantBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+MainLightShadowConstantBuffer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_MainLightShadowConstantBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+MainLightShadowConstantBuffer"
)]
impl crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_MainLightShadowConstantBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+MainLightShadowConstantBuffer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_MainLightShadowConstantBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+PassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct MainLightShadowCasterPass_PassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub emptyShadowmap: bool,
    pub renderingData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalRenderingData>,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
    pub lightData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalLightData>,
    pub shadowData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalShadowData>,
    pub pass: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass,
    >,
    pub shadowmapTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub shadowRendererLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::RendererList>,
    >,
    pub shadowRendererListsHandle: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+PassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "MainLightShadowCasterPass/PassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+PassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+PassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+PassData")]
impl crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+MainLightShadowCasterPass+PassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::Internal::MainLightShadowCasterPass_PassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
