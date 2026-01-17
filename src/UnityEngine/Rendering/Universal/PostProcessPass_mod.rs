#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
    pub m_Descriptor: crate::UnityEngine::RenderTextureDescriptor,
    pub m_Source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_Destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_Depth: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_InternalLut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_MotionVectors: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_FullCoCTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_HalfCoCTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_PingTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_PongTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_BloomMipDown: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub m_BloomMipUp: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub _BloomMipUp: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    >,
    pub _BloomMipDown: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    >,
    pub m_BlendTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_EdgeColorTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_EdgeStencilTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_TempTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_TempTarget2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_StreakTmpTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_StreakTmpTexture2: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_ScreenSpaceLensFlareResult:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_UserLut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_Materials: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::PostProcessPass_MaterialLibrary,
    >,
    pub m_Data:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::PostProcessData>,
    pub m_DepthOfField:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DepthOfField>,
    pub m_MotionBlur:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::MotionBlur>,
    pub m_LensFlareScreenSpace:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare>,
    pub m_PaniniProjection:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::PaniniProjection>,
    pub m_Bloom: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::Bloom>,
    pub m_LensDistortion:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::LensDistortion>,
    pub m_ChromaticAberration:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ChromaticAberration>,
    pub m_Vignette: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::Vignette>,
    pub m_ColorLookup:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ColorLookup>,
    pub m_ColorAdjustments:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ColorAdjustments>,
    pub m_Tonemapping:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::Tonemapping>,
    pub m_FilmGrain: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::FilmGrain>,
    pub m_DefaultColorFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub m_DefaultColorFormatIsAlpha: bool,
    pub m_SMAAEdgeFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub m_GaussianCoCFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    pub m_DitheringTextureIndex: i32,
    pub m_MRT2: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
    >,
    pub m_BokehKernel:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>>,
    pub m_BokehHash: i32,
    pub m_BokehMaxRadius: f32,
    pub m_BokehRCPAspect: f32,
    pub m_IsFinalPass: bool,
    pub m_HasFinalPass: bool,
    pub m_EnableColorEncodingIfNeeded: bool,
    pub m_UseFastSRGBLinearConversion: bool,
    pub m_SupportScreenSpaceLensFlare: bool,
    pub m_SupportDataDrivenLensFlare: bool,
    pub m_ResolveToScreen: bool,
    pub m_UseSwapBuffer: bool,
    pub m_ScalingSetupTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_UpscaledTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_BlitMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub m_BloomParamsPrev:
        crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::PostProcessPass {
    type Target = crate::UnityEngine::Rendering::Universal::ScriptableRenderPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::PostProcessPass {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass {
    pub const _TemporalAATargetName: &'static str = "_TemporalAATarget";
    pub const _UpscaledColorTargetName: &'static str = "_UpscaledColorTarget";
    pub const k_BokehDoFPassBlur: i32 = 2i32;
    pub const k_BokehDoFPassComposite: i32 = 4i32;
    pub const k_BokehDoFPassComputeCoc: i32 = 0i32;
    pub const k_BokehDoFPassDownscalePrefilter: i32 = 1i32;
    pub const k_BokehDoFPassPostFilter: i32 = 3i32;
    pub const k_GaussianDoFPassBlurH: i32 = 2i32;
    pub const k_GaussianDoFPassBlurV: i32 = 3i32;
    pub const k_GaussianDoFPassComposite: i32 = 4i32;
    pub const k_GaussianDoFPassComputeCoc: i32 = 0i32;
    pub const k_GaussianDoFPassDownscalePrefilter: i32 = 1i32;
    pub const k_MaxPyramidSize: i32 = 16i32;
    pub const k_RenderFinalPostProcessingTag: &'static str = "Blit Final PostProcessing";
    pub const k_RenderPostProcessingTag: &'static str = "Blit PostProcessing Effects";
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
    pub type BloomMaterialParams =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+BloomPassData")]
    pub type BloomPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFBokehPassData")]
    pub type DoFBokehPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFBokehPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFGaussianPassData")]
    pub type DoFGaussianPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFGaussianPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
    pub type FinalBlitSettings =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlarePassData")]
    pub type LensFlarePassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlarePassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlareScreenSpacePassData")]
    pub type LensFlareScreenSpacePassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlareScreenSpacePassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MaterialLibrary")]
    pub type MaterialLibrary =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_MaterialLibrary;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MotionBlurPassData")]
    pub type MotionBlurPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_MotionBlurPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PaniniProjectionPassData")]
    pub type PaniniProjectionPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_PaniniProjectionPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostFXSetupPassData")]
    pub type PostFXSetupPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_PostFXSetupPassData;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalBlitPassData"
    )]
    pub type PostProcessingFinalBlitPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalBlitPassData;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalFSRScalePassData"
    )]
    pub type PostProcessingFinalFSRScalePassData = crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalFSRScalePassData;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalSetupPassData"
    )]
    pub type PostProcessingFinalSetupPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalSetupPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAAPassData")]
    pub type SMAAPassData = crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAAPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAASetupPassData")]
    pub type SMAASetupPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAASetupPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+ShaderConstants")]
    pub type ShaderConstants =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_ShaderConstants;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+StopNaNsPassData")]
    pub type StopNaNsPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_StopNaNsPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberPostPassData")]
    pub type UberPostPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_UberPostPassData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberSetupBloomPassData")]
    pub type UberSetupBloomPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_UberSetupBloomPassData;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+PostProcessPass+UpdateCameraResolutionPassData"
    )]
    pub type UpdateCameraResolutionPassData =
        crate::UnityEngine::Rendering::Universal::PostProcessPass_UpdateCameraResolutionPassData;
    pub fn CalcCropExtents(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        d: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>, f32),
                        crate::UnityEngine::Vector2,
                        2usize,
                    >("CalcCropExtents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalcCropExtents", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera, d))? };
        Ok(__cordl_ret.into())
    }
    pub fn CalcViewExtents(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>),
                        crate::UnityEngine::Vector2,
                        1usize,
                    >("CalcViewExtents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CalcViewExtents", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector2 =
            unsafe { cordl_method_info.invoke_unchecked(self, (camera))? };
        Ok(__cordl_ret.into())
    }
    pub fn CanRunOnTile(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("CanRunOnTile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CanRunOnTile",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn DoBokehDepthOfField(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        pixelRect: crate::UnityEngine::Rect,
        enableAlphaOutput: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rect,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DoBokehDepthOfField"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoBokehDepthOfField",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cmd, source, destination, pixelRect, enableAlphaOutput),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoDepthOfField(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        pixelRect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rect,
                    ), quest_hook::libil2cpp::Void, 5usize>("DoDepthOfField")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoDepthOfField",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (cameraData, cmd, source, destination, pixelRect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoGaussianDepthOfField(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        pixelRect: crate::UnityEngine::Rect,
        enableAlphaOutput: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rect,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DoGaussianDepthOfField"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoGaussianDepthOfField",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cmd, source, destination, pixelRect, enableAlphaOutput),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoLensFlareScreenSpace(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        originalBloomTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        screenSpaceLensFlareBloomMipTexture: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RTHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "DoLensFlareScreenSpace"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoLensFlareScreenSpace",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    camera,
                    cmd,
                    source,
                    originalBloomTexture,
                    screenSpaceLensFlareBloomMipTexture,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoMotionBlur(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        motionVectors: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>("DoMotionBlur")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoMotionBlur",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (cmd, source, destination, motionVectors, cameraData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoPaniniProjection(
        &mut self,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 4usize>("DoPaniniProjection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoPaniniProjection",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (camera, cmd, source, destination))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoSubpixelMorphologicalAntialiasing(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::CameraData,
        >,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::CameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "DoSubpixelMorphologicalAntialiasing"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DoSubpixelMorphologicalAntialiasing",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (cameraData, cmd, source, destination))?
        };
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
    pub fn GetCompatibleDescriptor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureDescriptor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::UnityEngine::RenderTextureDescriptor, 0usize>(
                        "GetCompatibleDescriptor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCompatibleDescriptor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureDescriptor =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompatibleDescriptor_RenderTextureDescriptor_i32_i32_GraphicsFormat_GraphicsFormat2(
        desc: crate::UnityEngine::RenderTextureDescriptor,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureDescriptor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::RenderTextureDescriptor,
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    ), crate::UnityEngine::RenderTextureDescriptor, 5usize>(
                        "GetCompatibleDescriptor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCompatibleDescriptor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureDescriptor = unsafe {
            cordl_method_info
                .invoke_unchecked((), (desc, width, height, format, depthStencilFormat))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCompatibleDescriptor_i32_i32_GraphicsFormat_GraphicsFormat1(
        &mut self,
        width: i32,
        height: i32,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        depthStencilFormat: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureDescriptor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        i32,
                        i32,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                    ), crate::UnityEngine::RenderTextureDescriptor, 4usize>(
                        "GetCompatibleDescriptor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCompatibleDescriptor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::RenderTextureDescriptor = unsafe {
            cordl_method_info.invoke_unchecked(self, (width, height, format, depthStencilFormat))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLensFlareLightAttenuation(
        light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        wo: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Vector3,
                    ), f32, 3usize>("GetLensFlareLightAttenuation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetLensFlareLightAttenuation",
                            3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (light, cam, wo))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxBokehRadiusInPixels(viewportHeight: f32) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32), f32, 1usize>("GetMaxBokehRadiusInPixels")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMaxBokehRadiusInPixels",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked((), (viewportHeight))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsAlphaFormat(
        &mut self,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsAlphaFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsAlphaFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsHDRFormat(
        &mut self,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Experimental::Rendering::GraphicsFormat),
                        bool,
                        1usize,
                    >("IsHDRFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsHDRFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn LensFlareDataDriven(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
            >,
        >,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        bool,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "LensFlareDataDriven"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LensFlareDataDriven",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cameraData,
                    cmd,
                    source,
                    usePanini,
                    paniniDistance,
                    paniniCropToFit,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LensFlareDataDrivenComputeOcclusion_ByRefMut_CommandBuffer_RenderTargetIdentifier__cordl_bool_f32_f32_0(
        &mut self,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
            >,
        >,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        usePanini: bool,
        paniniDistance: f32,
        paniniCropToFit: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        bool,
                        f32,
                        f32,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "LensFlareDataDrivenComputeOcclusion"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LensFlareDataDrivenComputeOcclusion",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cameraData,
                    cmd,
                    source,
                    usePanini,
                    paniniDistance,
                    paniniCropToFit,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LensFlareDataDrivenComputeOcclusion_RenderGraph_UniversalResourceData_UniversalCameraData1(
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
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "LensFlareDataDrivenComputeOcclusion"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LensFlareDataDrivenComputeOcclusion",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderGraph, resourceData, cameraData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::PostProcessData>,
        postProcessParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::PostProcessParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (evt, data, postProcessParams))?;
        Ok(__cordl_object.into())
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
    pub fn PrepareBokehKernel(
        &mut self,
        maxRadius: f32,
        rcpAspect: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32, f32), quest_hook::libil2cpp::Void, 2usize>(
                        "PrepareBokehKernel",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PrepareBokehKernel",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (maxRadius, rcpAspect))? };
        Ok(__cordl_ret.into())
    }
    pub fn Render(
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
                    ), quest_hook::libil2cpp::Void, 2usize>("Render")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Render",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn RenderBloomTexture(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        enableAlphaOutput: bool,
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
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>("RenderBloomTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderBloomTexture",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderGraph, source, destination, enableAlphaOutput))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDoF(
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
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
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
                    ), quest_hook::libil2cpp::Void, 5usize>("RenderDoF")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderDoF",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (renderGraph, resourceData, cameraData, source, destination),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDoFBokeh(
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
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        dofMaterial: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>("RenderDoFBokeh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderDoFBokeh",
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
                    source,
                    destination,
                    dofMaterial,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderDoFGaussian(
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
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        dofMaterial: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>("RenderDoFGaussian")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderDoFGaussian",
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
                    source,
                    destination,
                    dofMaterial,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderFinalBlit(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        overlayUITexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        postProcessingTarget: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings,
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
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
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
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("RenderFinalBlit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderFinalBlit", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    cameraData,
                    source,
                    overlayUITexture,
                    postProcessingTarget,
                    settings,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderFinalFSRScale(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        enableAlphaOutput: bool,
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
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "RenderFinalFSRScale"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderFinalFSRScale",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderGraph, source, destination, enableAlphaOutput))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderFinalPass(
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
                    ), quest_hook::libil2cpp::Void, 2usize>("RenderFinalPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderFinalPass",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn RenderFinalPassRenderGraph(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        overlayUITexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        postProcessingTarget: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        enableColorEncodingIfNeeded: bool,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "RenderFinalPassRenderGraph"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderFinalPassRenderGraph",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    frameData,
                    source,
                    overlayUITexture,
                    postProcessingTarget,
                    enableColorEncodingIfNeeded,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderFinalSetup(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        settings: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings,
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
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
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
                                crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("RenderFinalSetup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderFinalSetup", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (renderGraph, cameraData, source, destination, settings),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderLensFlareDataDriven(
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
        destination: quest_hook::libil2cpp::ByRefMut<
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
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "RenderLensFlareDataDriven"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderLensFlareDataDriven",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderGraph, resourceData, cameraData, destination))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderLensFlareScreenSpace(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        destination: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        originalBloomTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        screenSpaceLensFlareBloomMipTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        enableXR: bool,
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
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        bool,
                    ), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 6usize>(
                        "RenderLensFlareScreenSpace",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderLensFlareScreenSpace",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    camera,
                    destination,
                    originalBloomTexture,
                    screenSpaceLensFlareBloomMipTexture,
                    enableXR,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderMotionBlur(
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
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
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
                    ), quest_hook::libil2cpp::Void, 5usize>("RenderMotionBlur")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderMotionBlur",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (renderGraph, resourceData, cameraData, source, destination),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderPaniniProjection(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
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
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "RenderPaniniProjection"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderPaniniProjection",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (renderGraph, camera, source, destination))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderPostProcessingRenderGraph(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
        activeCameraColorTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        lutTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        overlayUITexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        postProcessingTarget: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        hasFinalPass: bool,
        resolveToDebugScreen: bool,
        enableColorEndingIfNeeded: bool,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
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
                        bool,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 9usize>(
                        "RenderPostProcessingRenderGraph"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderPostProcessingRenderGraph",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    frameData,
                    activeCameraColorTexture,
                    lutTexture,
                    overlayUITexture,
                    postProcessingTarget,
                    hasFinalPass,
                    resolveToDebugScreen,
                    enableColorEndingIfNeeded,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderSMAA(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        resourceData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalResourceData,
        >,
        antialiasingQuality: crate::UnityEngine::Rendering::Universal::AntialiasingQuality,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        SMAATarget: quest_hook::libil2cpp::ByRefMut<
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
                        crate::UnityEngine::Rendering::Universal::AntialiasingQuality,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>("RenderSMAA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderSMAA",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    resourceData,
                    antialiasingQuality,
                    source,
                    SMAATarget,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderSTP(
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
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
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
                    ), quest_hook::libil2cpp::Void, 5usize>("RenderSTP")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderSTP",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (renderGraph, resourceData, cameraData, source, destination),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderStopNaN(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        cameraTargetDescriptor: crate::UnityEngine::RenderTextureDescriptor,
        activeCameraColor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        stopNaNTarget: quest_hook::libil2cpp::ByRefMut<
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
                        crate::UnityEngine::RenderTextureDescriptor,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>("RenderStopNaN")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderStopNaN",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    cameraTargetDescriptor,
                    activeCameraColor,
                    stopNaNTarget,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderTemporalAA(
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
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destination: quest_hook::libil2cpp::ByRefMut<
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
                    ), quest_hook::libil2cpp::Void, 5usize>("RenderTemporalAA")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderTemporalAA",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (renderGraph, resourceData, cameraData, source, destination),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderUberPost(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        postProcessingData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalPostProcessingData,
        >,
        sourceTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        destTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        lutTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        overlayUITexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        requireHDROutput: bool,
        enableAlphaOutput: bool,
        resolveToDebugScreen: bool,
        hasFinalPass: bool,
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
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalPostProcessingData,
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
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        bool,
                        bool,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 12usize>("RenderUberPost")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RenderUberPost",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    frameData,
                    cameraData,
                    postProcessingData,
                    sourceTexture,
                    destTexture,
                    lutTexture,
                    overlayUITexture,
                    requireHDROutput,
                    enableAlphaOutput,
                    resolveToDebugScreen,
                    hasFinalPass,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequireHDROutput(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                    >), bool, 1usize>("RequireHDROutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequireHDROutput",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn RequireSRGBConversionBlitToBackBuffer(
        &mut self,
        requireSrgbConversion: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), bool, 1usize>("RequireSRGBConversionBlitToBackBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RequireSRGBConversionBlitToBackBuffer",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (requireSrgbConversion))? };
        Ok(__cordl_ret.into())
    }
    pub fn ScaleViewportAndBlit(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        sourceTextureHdl: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        hasFinalPass: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "ScaleViewportAndBlit"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ScaleViewportAndBlit",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    sourceTextureHdl,
                    dest,
                    cameraData,
                    material,
                    hasFinalPass,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        baseDescriptor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::RenderTextureDescriptor,
        >,
        source: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        resolveToScreen: bool,
        depth: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        internalLut: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        motionVectors: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        hasFinalPass: bool,
        enableColorEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 8usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    baseDescriptor,
                    source,
                    resolveToScreen,
                    depth,
                    internalLut,
                    motionVectors,
                    hasFinalPass,
                    enableColorEncoding,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupBloom(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        uberMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        enableAlphaOutput: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>("SetupBloom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupBloom",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (cmd, source, uberMaterial, enableAlphaOutput))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupChromaticAberration(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetupChromaticAberration")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupChromaticAberration", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupColorGrading(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetupColorGrading")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupColorGrading",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, renderingData, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupDithering(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupDithering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupDithering",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraData, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupFinalPass(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        useSwapBuffer: bool,
        enableColorEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetupFinalPass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupFinalPass",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (source, useSwapBuffer, enableColorEncoding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupGrain(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupGrain")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupGrain",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraData, material))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupHDROutput(
        &mut self,
        hdrDisplayInformation: crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation,
        hdrDisplayColorGamut: crate::UnityEngine::ColorGamut,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        hdrOperations: crate::UnityEngine::Rendering::HDROutputUtils_Operation,
        rendersOverlayUI: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation,
                        crate::UnityEngine::ColorGamut,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        crate::UnityEngine::Rendering::HDROutputUtils_Operation,
                        bool,
                    ), quest_hook::libil2cpp::Void, 5usize>("SetupHDROutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupHDROutput",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    hdrDisplayInformation,
                    hdrDisplayColorGamut,
                    material,
                    hdrOperations,
                    rendersOverlayUI,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupLensDistortion(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        isSceneView: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetupLensDistortion"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupLensDistortion",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, isSceneView))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetupVignette(
        &mut self,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        xrPass: quest_hook::libil2cpp::Gc<crate::UnityEngine::Experimental::Rendering::XRPass>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Experimental::Rendering::XRPass,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetupVignette")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupVignette",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (material, xrPass))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCachedUserLutTextureHandle(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                    >), crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle, 1usize>(
                        "TryGetCachedUserLutTextureHandle",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetCachedUserLutTextureHandle",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn UberPostSetupBloomPass(
        &mut self,
        rendergraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        bloomTexture: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        uberMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UberPostSetupBloomPass"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UberPostSetupBloomPass",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (rendergraph, bloomTexture, uberMaterial))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCameraResolution(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        newCameraTargetSize: crate::UnityEngine::Vector2Int,
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
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        crate::UnityEngine::Vector2Int,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UpdateCameraResolution"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateCameraResolution",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (renderGraph, cameraData, newCameraTargetSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMotionBlurMatrices(
        material: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        xr: quest_hook::libil2cpp::Gc<crate::UnityEngine::Experimental::Rendering::XRPass>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Experimental::Rendering::XRPass,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(
                        "UpdateMotionBlurMatrices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UpdateMotionBlurMatrices",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (material, camera, xr))? };
        Ok(__cordl_ret.into())
    }
    pub fn _Render_g__GetDestination_89_1(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::ValueTypePadding<40>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::ValueTypePadding<40>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        1usize,
                    >("<Render>g__GetDestination|89_1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<Render>g__GetDestination|89_1", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _Render_g__GetSource_89_0(
        &mut self,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::ValueTypePadding<40>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::ValueTypePadding<40>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RTHandle,
                        >,
                        1usize,
                    >("<Render>g__GetSource|89_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<Render>g__GetSource|89_0", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle> = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _Render_g__Swap_89_2(
        &mut self,
        r: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ScriptableRenderer>,
        >,
        _cordl_fixed_empty_name_whitespace: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::ValueTypePadding<40>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::ScriptableRenderer,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::ValueTypePadding<40>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "<Render>g__Swap|89_2"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "<Render>g__Swap|89_2",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (r, _cordl_fixed_empty_name_whitespace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::PostProcessData>,
        postProcessParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::PostProcessParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::PostProcessData,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::PostProcessParams,
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (evt, data, postProcessParams))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostProcessPass_BloomMaterialParams {
    pub parameters: crate::UnityEngine::Vector4,
    pub highQualityFiltering: bool,
    pub enableAlphaOutput: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/BloomMaterialParams";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+BloomMaterialParams")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams {
    pub fn Equals(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomMaterialParams,
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
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_BloomPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mipCount: i32,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub upsampleMaterials: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>,
    >,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub bloomMipUp: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    >,
    pub bloomMipDown: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/BloomPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+BloomPassData")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomPassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+BloomPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+BloomPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+BloomPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_BloomPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+DoFBokehPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_DoFBokehPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub bokehKernel:
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>>,
    pub downSample: i32,
    pub uvMargin: f32,
    pub cocParams: crate::UnityEngine::Vector4,
    pub useFastSRGBLinearConversion: bool,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub depthTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub materialCoC: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub halfCoCTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub fullCoCTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub pingTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub pongTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+DoFBokehPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFBokehPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/DoFBokehPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFBokehPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFBokehPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFBokehPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFBokehPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFBokehPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFBokehPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+DoFBokehPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFBokehPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+DoFGaussianPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_DoFGaussianPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub downsample: i32,
    pub renderingData: crate::UnityEngine::Rendering::Universal::RenderingData,
    pub cocParams: crate::UnityEngine::Vector3,
    pub highQualitySamplingValue: bool,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub depthTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub materialCoC: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub halfCoCTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub fullCoCTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub pingTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub pongTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub multipleRenderTargets: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::RenderTargetIdentifier>,
    >,
    pub destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+DoFGaussianPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFGaussianPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/DoFGaussianPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFGaussianPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFGaussianPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFGaussianPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFGaussianPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+DoFGaussianPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFGaussianPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+DoFGaussianPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_DoFGaussianPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct PostProcessPass_FinalBlitSettings {
    pub isFxaaEnabled: bool,
    pub isFsrEnabled: bool,
    pub isTaaSharpeningEnabled: bool,
    pub requireHDROutput: bool,
    pub resolveToDebugScreen: bool,
    pub isAlphaOutputEnabled: bool,
    pub hdrOperations: crate::UnityEngine::Rendering::HDROutputUtils_Operation,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/FinalBlitSettings";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+FinalBlitSettings")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings,
                        0usize,
                    >("Create")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Create",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+LensFlarePassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_LensFlarePassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceDescriptor: crate::UnityEngine::RenderTextureDescriptor,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub viewport: crate::UnityEngine::Rect,
    pub paniniDistance: f32,
    pub paniniCropToFit: f32,
    pub width: f32,
    pub height: f32,
    pub usePanini: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+LensFlarePassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlarePassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/LensFlarePassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlarePassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlarePassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlarePassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlarePassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlarePassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlarePassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+LensFlarePassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlarePassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+LensFlareScreenSpacePassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_LensFlareScreenSpacePassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub streakTmpTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub streakTmpTexture2: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub originalBloomTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub screenSpaceLensFlareBloomMipTexture:
        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub result: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceDescriptor: crate::UnityEngine::RenderTextureDescriptor,
    pub camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub lensFlareScreenSpace:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::ScreenSpaceLensFlare>,
    pub downsample: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+LensFlareScreenSpacePassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlareScreenSpacePassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/LensFlareScreenSpacePassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlareScreenSpacePassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlareScreenSpacePassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlareScreenSpacePassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlareScreenSpacePassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+LensFlareScreenSpacePassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlareScreenSpacePassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+LensFlareScreenSpacePassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_LensFlareScreenSpacePassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+MaterialLibrary")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_MaterialLibrary {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub stopNaN: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub subpixelMorphologicalAntialiasing: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub gaussianDepthOfField: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub gaussianDepthOfFieldCoC: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub bokehDepthOfField: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub bokehDepthOfFieldCoC: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub cameraMotionBlur: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub paniniProjection: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub bloom: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub bloomUpsample: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>,
    >,
    pub temporalAntialiasing: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub scalingSetup: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub easu: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub uber: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub finalPass: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub lensFlareDataDriven: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub lensFlareScreenSpace: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+MaterialLibrary")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_MaterialLibrary
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/MaterialLibrary";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MaterialLibrary")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::PostProcessPass_MaterialLibrary {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MaterialLibrary")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_MaterialLibrary
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MaterialLibrary")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_MaterialLibrary {
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
    pub fn Load(
        &mut self,
        shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        1usize,
                    >("Load")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Load",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked(self, (shader))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::PostProcessData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::PostProcessData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::PostProcessData,
                    >), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (data))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+MaterialLibrary")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_MaterialLibrary
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+MotionBlurPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_MotionBlurPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub motionVectors: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub passIndex: i32,
    pub camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub xr: quest_hook::libil2cpp::Gc<crate::UnityEngine::Experimental::Rendering::XRPass>,
    pub intensity: f32,
    pub clamp: f32,
    pub enableAlphaOutput: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+MotionBlurPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_MotionBlurPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/MotionBlurPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MotionBlurPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_MotionBlurPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MotionBlurPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_MotionBlurPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+MotionBlurPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_MotionBlurPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+MotionBlurPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_MotionBlurPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PaniniProjectionPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_PaniniProjectionPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTextureDesc: crate::UnityEngine::RenderTextureDescriptor,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub paniniParams: crate::UnityEngine::Vector4,
    pub isPaniniGeneric: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PaniniProjectionPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PaniniProjectionPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/PaniniProjectionPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PaniniProjectionPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PaniniProjectionPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PaniniProjectionPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PaniniProjectionPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PaniniProjectionPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_PaniniProjectionPassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PaniniProjectionPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PaniniProjectionPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostFXSetupPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_PostFXSetupPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostFXSetupPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostFXSetupPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/PostFXSetupPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostFXSetupPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostFXSetupPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostFXSetupPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostFXSetupPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostFXSetupPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_PostFXSetupPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostFXSetupPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostFXSetupPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalBlitPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_PostProcessingFinalBlitPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
    pub settings: crate::UnityEngine::Rendering::Universal::PostProcessPass_FinalBlitSettings,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalBlitPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalBlitPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/PostProcessingFinalBlitPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalBlitPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalBlitPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalBlitPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalBlitPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalBlitPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalBlitPassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalBlitPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalBlitPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalFSRScalePassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_PostProcessingFinalFSRScalePassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub enableAlphaOutput: bool,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalFSRScalePassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalFSRScalePassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/PostProcessingFinalFSRScalePassData";
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
    feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalFSRScalePassData"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalFSRScalePassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalFSRScalePassData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalFSRScalePassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalFSRScalePassData"
)]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalFSRScalePassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalFSRScalePassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalFSRScalePassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalSetupPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_PostProcessingFinalSetupPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalSetupPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalSetupPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/PostProcessingFinalSetupPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalSetupPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalSetupPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalSetupPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalSetupPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalSetupPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalSetupPassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+PostProcessingFinalSetupPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_PostProcessingFinalSetupPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+SMAAPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_SMAAPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub depthStencilTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub blendTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+SMAAPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAAPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/SMAAPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAAPassData")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAAPassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAAPassData")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAAPassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAAPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAAPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+SMAAPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAAPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+SMAASetupPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_SMAASetupPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub metrics: crate::UnityEngine::Vector4,
    pub areaTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub searchTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub stencilRef: f32,
    pub stencilMask: f32,
    pub antialiasingQuality: crate::UnityEngine::Rendering::Universal::AntialiasingQuality,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+SMAASetupPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAASetupPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/SMAASetupPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAASetupPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAASetupPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAASetupPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAASetupPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+SMAASetupPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAASetupPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+SMAASetupPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_SMAASetupPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+ShaderConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_ShaderConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+ShaderConstants")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_ShaderConstants
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/ShaderConstants";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+ShaderConstants")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::PostProcessPass_ShaderConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+ShaderConstants")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_ShaderConstants
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+ShaderConstants")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_ShaderConstants {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+ShaderConstants")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_ShaderConstants
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+StopNaNsPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_StopNaNsPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub stopNaNTarget: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub stopNaN: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+StopNaNsPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_StopNaNsPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/StopNaNsPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+StopNaNsPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_StopNaNsPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+StopNaNsPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_StopNaNsPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+StopNaNsPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_StopNaNsPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+StopNaNsPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_StopNaNsPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UberPostPassData")]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_UberPostPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub destinationTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub lutTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub lutParams: crate::UnityEngine::Vector4,
    pub userLutTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub userLutParams: crate::UnityEngine::Vector4,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
    pub toneMappingMode: crate::UnityEngine::Rendering::Universal::TonemappingMode,
    pub isHdrGrading: bool,
    pub isBackbuffer: bool,
    pub enableAlphaOutput: bool,
    pub hasFinalPass: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UberPostPassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberPostPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/UberPostPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberPostPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberPostPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberPostPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberPostPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberPostPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_UberPostPassData {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UberPostPassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberPostPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UberSetupBloomPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_UberSetupBloomPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub bloomParams: crate::UnityEngine::Vector4,
    pub dirtScaleOffset: crate::UnityEngine::Vector4,
    pub dirtIntensity: f32,
    pub dirtTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub highQualityFilteringValue: bool,
    pub bloomTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub uberMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UberSetupBloomPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberSetupBloomPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/UberSetupBloomPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberSetupBloomPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberSetupBloomPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberSetupBloomPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberSetupBloomPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UberSetupBloomPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_UberSetupBloomPassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UberSetupBloomPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UberSetupBloomPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UpdateCameraResolutionPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PostProcessPass_UpdateCameraResolutionPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub newCameraTargetSize: crate::UnityEngine::Vector2Int,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UpdateCameraResolutionPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UpdateCameraResolutionPassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "PostProcessPass/UpdateCameraResolutionPassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UpdateCameraResolutionPassData")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UpdateCameraResolutionPassData
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UpdateCameraResolutionPassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UpdateCameraResolutionPassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+PostProcessPass+UpdateCameraResolutionPassData")]
impl crate::UnityEngine::Rendering::Universal::PostProcessPass_UpdateCameraResolutionPassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+PostProcessPass+UpdateCameraResolutionPassData"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::PostProcessPass_UpdateCameraResolutionPassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
