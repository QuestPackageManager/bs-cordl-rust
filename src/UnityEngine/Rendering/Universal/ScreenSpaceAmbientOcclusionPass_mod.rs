#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenSpaceAmbientOcclusionPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
    pub m_SupportsR8RenderTextureFormat: bool,
    pub m_BlueNoiseTextureIndex: i32,
    pub m_Material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_PassData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData,
    >,
    pub m_BlueNoiseTextures: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    >,
    pub m_CameraTopLeftCorner: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub m_CameraXExtent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub m_CameraYExtent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub m_CameraZExtent: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub m_SSAOTextures: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub m_BlurType: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes,
    pub m_CameraViewProjections: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    >,
    pub m_ProfilingSampler: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ProfilingSampler,
    >,
    pub m_Renderer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::ScriptableRenderer,
    >,
    pub m_AOPassDescriptor: crate::UnityEngine::RenderTextureDescriptor,
    pub m_CurrentSettings: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings,
    >,
    pub m_SSAOParamsPrev: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass {
    type Target = crate::UnityEngine::Rendering::Universal::ScriptableRenderPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass")]
impl crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass {
    pub const k_AmbientOcclusionParamName: &'static str = "_AmbientOcclusionParam";
    pub const k_SSAOTextureName: &'static str = "_ScreenSpaceOcclusionTexture";
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+BlurTypes")]
    pub type BlurTypes =
        crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
    )]
    pub type SSAOMaterialParams = crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOPassData")]
    pub type SSAOPassData =
        crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+ShaderPasses")]
    pub type ShaderPasses =
        crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses;
    pub fn CreateRenderTextureHandles(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        resourceData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalResourceData,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        aoTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        blurTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        finalTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
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
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalResourceData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "CreateRenderTextureHandles"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRenderTextureHandles",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    resourceData,
                    cameraData,
                    aoTexture,
                    blurTexture,
                    finalTexture,
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
    pub fn GetPassOrder(
        blurType: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes,
        isAfterOpaque: bool,
        textureIndices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        shaderPasses: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<i32>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("GetPassOrder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPassOrder", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (blurType, isAfterOpaque, textureIndices, shaderPasses))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitSSAOPassData(
        &mut self,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InitSSAOPassData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitSSAOPassData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (data))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsAfterOpaquePass(
        pass: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses,
                        >),
                        bool,
                        1usize,
                    >("IsAfterOpaquePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsAfterOpaquePass", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (pass))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnCameraCleanup(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnCameraCleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnCameraCleanup", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnCameraSetup(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
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
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("OnCameraSetup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnCameraSetup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordRenderGraph(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
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
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
                    ), quest_hook::libil2cpp::Void, 2usize>("RecordRenderGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RecordRenderGraph",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, frameData))? };
        Ok(__cordl_ret.into())
    }
    pub fn RenderAndSetBaseMap(
        cmd: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        >,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
        renderer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ScriptableRenderer>,
        >,
        mat: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
        baseMap: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        target: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        pass: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::CommandBuffer,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::ScriptableRenderer,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::RTHandle,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::RTHandle,
                                >,
                            >,
                            crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("RenderAndSetBaseMap")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderAndSetBaseMap", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (cmd, renderingData, renderer, mat, baseMap, target, pass),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        featureSettings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings,
            >,
        >,
        renderer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ScriptableRenderer>,
        >,
        material: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
        blueNoiseTextures: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::ScriptableRenderer,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                                    >,
                                >,
                            >,
                        ),
                        bool,
                        4usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Setup",
                            4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (featureSettings, renderer, material, blueNoiseTextures),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupKeywordsAndParameters(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings,
            >,
        >,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
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
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetupKeywordsAndParameters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupKeywordsAndParameters", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (settings, cameraData))? };
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
    pub fn get_isRendererDeferred(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isRendererDeferred")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_isRendererDeferred",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+BlurTypes"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ScreenSpaceAmbientOcclusionPass_BlurTypes {
    #[default]
    Bilateral = 0i32,
    Gaussian = 1i32,
    Kawase = 2i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+BlurTypes"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionPass/BlurTypes";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+BlurTypes"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+BlurTypes"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+BlurTypes"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+BlurTypes"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_BlurTypes
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams {
    pub orthographicCamera: bool,
    pub aoBlueNoise: bool,
    pub aoInterleavedGradient: bool,
    pub sampleCountHigh: bool,
    pub sampleCountMedium: bool,
    pub sampleCountLow: bool,
    pub sourceDepthNormals: bool,
    pub sourceDepthHigh: bool,
    pub sourceDepthMedium: bool,
    pub sourceDepthLow: bool,
    pub ssaoParams: crate::UnityEngine::Vector4,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionPass/SSAOMaterialParams";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOMaterialParams"
)]
impl crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams {
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOMaterialParams,
                        >),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (other))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        settings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings,
            >,
        >,
        isOrthographic: bool,
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
                                    crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (settings, isOrthographic))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenSpaceAmbientOcclusionPass_SSAOPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub afterOpaque: bool,
    pub BlurQuality: crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionSettings_BlurQualityOptions,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub directLightingStrength: f32,
    pub cameraColor: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub AOTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub finalTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub blurTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub cameraNormalsTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionPass/SSAOPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOPassData")]
impl crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+SSAOPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_SSAOPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+ShaderPasses"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ScreenSpaceAmbientOcclusionPass_ShaderPasses {
    #[default]
    AmbientOcclusion = 0i32,
    BilateralAfterOpaque = 4i32,
    BilateralBlurFinal = 3i32,
    BilateralBlurHorizontal = 1i32,
    BilateralBlurVertical = 2i32,
    GaussianAfterOpaque = 7i32,
    GaussianBlurHorizontal = 5i32,
    GaussianBlurVertical = 6i32,
    KawaseAfterOpaque = 9i32,
    KawaseBlur = 8i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+ShaderPasses"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ScreenSpaceAmbientOcclusionPass/ShaderPasses";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+ShaderPasses"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+ShaderPasses"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+ShaderPasses"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+ScreenSpaceAmbientOcclusionPass+ShaderPasses"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::ScreenSpaceAmbientOcclusionPass_ShaderPasses
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
