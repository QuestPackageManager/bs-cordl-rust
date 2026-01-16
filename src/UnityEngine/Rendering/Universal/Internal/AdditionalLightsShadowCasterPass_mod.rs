#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AdditionalLightsShadowCasterPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
    pub m_AdditionalLightsShadowmapHandle: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RTHandle,
    >,
    pub renderTargetWidth: i32,
    pub renderTargetHeight: i32,
    pub m_CreateEmptyShadowmap: bool,
    pub m_EmptyShadowmapNeedsClear: bool,
    pub m_IssuedMessageAboutShadowSlicesTooMany: bool,
    pub m_IssuedMessageAboutShadowMapsRescale: bool,
    pub m_IssuedMessageAboutShadowMapsTooBig: bool,
    pub m_IssuedMessageAboutRemovedShadowSlices: bool,
    pub m_UseStructuredBuffer: bool,
    pub m_MaxShadowDistanceSq: f32,
    pub m_CascadeBorder: f32,
    pub m_PassData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData,
    >,
    pub m_EmptyAdditionalLightShadowmapTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RTHandle,
    >,
    pub m_VisibleLightIndexToIsCastingShadows: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<bool>,
    >,
    pub m_VisibleLightIndexToAdditionalLightIndex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i16>,
    >,
    pub m_AdditionalLightIndexToVisibleLightIndex: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i16>,
    >,
    pub m_AdditionalLightIndexToShadowParams: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub m_AdditionalLightShadowSliceIndexTo_WorldShadowMatrix: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
    >,
    pub m_AdditionalLightsShadowSlices: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::Universal::ShadowSliceData,
        >,
    >,
    pub m_GlobalShadowSliceIndexToPerLightShadowSliceIndex: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<u8>,
    >,
    pub m_ShadowSliceToAdditionalLightIndex: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<i16>,
    >,
    pub m_ShadowRequestsHashes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<i32, u64>,
    >,
    pub m_ProfilingSetupSampler: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::ProfilingSampler,
    >,
    pub m_AdditionalLightShadowDescriptor: crate::UnityEngine::RenderTextureDescriptor,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "AdditionalLightsShadowCasterPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass
{
    type Target = crate::UnityEngine::Rendering::Universal::ScriptableRenderPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass")]
impl crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass {
    pub const k_AdditionalLightShadowMapTextureName: &'static str =
        "_AdditionalLightsShadowmapTexture";
    pub const k_EmptyAdditionalLightShadowMapTextureName: &'static str =
        "_EmptyAdditionalLightShadowmapTexture";
    pub const k_EmptyShadowMapDimensions: i32 = 1i32;
    pub const k_LightTypeIdentifierInShadowParams_Point: f32 = 1f32;
    pub const k_LightTypeIdentifierInShadowParams_Spot: f32 = 0f32;
    pub const k_ShadowmapBufferBits: i32 = 16i32;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+AdditionalShadowsConstantBuffer"
    )]
    pub type AdditionalShadowsConstantBuffer = crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+PassData"
    )]
    pub type PassData = crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData;
    pub fn CalcGuardAngle(
        frustumAngleInDegrees: f32,
        guardBandSizeInTexels: f32,
        sliceResolutionInTexels: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(f32, f32, f32), f32, 3usize>("CalcGuardAngle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CalcGuardAngle",
                            3usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    frustumAngleInDegrees,
                    guardBandSizeInTexels,
                    sliceResolutionInTexels,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
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
    pub fn ComputeShadowRequestHash(
        &mut self,
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        shadowData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
        >,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalLightData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                        >,
                    ), u64, 2usize>("ComputeShadowRequestHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ComputeShadowRequestHash",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u64 =
            unsafe { cordl_method_info.invoke_unchecked(self, (lightData, shadowData))? };
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
    pub fn GetLightTypeIdentifierForShadowParams(
        &mut self,
        lightType: crate::UnityEngine::LightType,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::UnityEngine::LightType), f32, 1usize>(
                        "GetLightTypeIdentifierForShadowParams",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetLightTypeIdentifierForShadowParams",
                            1usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, (lightType))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPointLightShadowFrustumFovBiasInDegrees(
        shadowSliceResolution: i32,
        shadowFiltering: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32, bool), f32, 2usize>(
                        "GetPointLightShadowFrustumFovBiasInDegrees",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPointLightShadowFrustumFovBiasInDegrees",
                            2usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe {
            cordl_method_info.invoke_unchecked((), (shadowSliceResolution, shadowFiltering))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowLightIndexFromLightIndex(
        &mut self,
        visibleLightIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), i32, 1usize>("GetShadowLightIndexFromLightIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetShadowLightIndexFromLightIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked(self, (visibleLightIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitPassData(
        &mut self,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData,
            >,
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
                                    crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData,
                                >,
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
                .invoke_unchecked(self, (passData, cameraData, lightData, shadowData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitRendererLists(
        &mut self,
        cullResults: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::CullingResults>,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData,
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
                                crate::UnityEngine::Rendering::CullingResults,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData,
                                >,
                            >,
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("InitRendererLists")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitRendererLists", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (cullResults, passData, context, renderGraph, useRenderGraph),
            )?
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
    pub fn RenderAdditionalShadowmapAtlas(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData,
            >,
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
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("RenderAdditionalShadowmapAtlas")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderAdditionalShadowmapAtlas", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, data, useRenderGraph))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResolutionLog2ForHash(&mut self, resolution: i32) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), u64, 1usize>("ResolutionLog2ForHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResolutionLog2ForHash",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe { cordl_method_info.invoke_unchecked(self, (resolution))? };
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
    pub fn SetupAdditionalLightsShadowReceiverConstants(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        allocatedShadowAtlasSize: crate::UnityEngine::Vector2Int,
        useStructuredBuffer: bool,
        softShadows: bool,
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
                        crate::UnityEngine::Vector2Int,
                        bool,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetupAdditionalLightsShadowReceiverConstants",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupAdditionalLightsShadowReceiverConstants",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    cmd,
                    allocatedShadowAtlasSize,
                    useStructuredBuffer,
                    softShadows,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupForEmptyRendering(
        &mut self,
        stripShadowsOffVariants: bool,
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
                        bool,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalLightData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalShadowData,
                        >,
                    ), bool, 3usize>("SetupForEmptyRendering")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetupForEmptyRendering",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (stripShadowsOffVariants, lightData, shadowData))?
        };
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
    pub fn UsesBakedShadows(
        &mut self,
        light: quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Light>),
                        bool,
                        1usize,
                    >("UsesBakedShadows")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UsesBakedShadows", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (light))? };
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+AdditionalShadowsConstantBuffer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+AdditionalShadowsConstantBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "AdditionalLightsShadowCasterPass/AdditionalShadowsConstantBuffer";
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
    feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+AdditionalShadowsConstantBuffer"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+AdditionalShadowsConstantBuffer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+AdditionalShadowsConstantBuffer"
)]
impl crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+AdditionalShadowsConstantBuffer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_AdditionalShadowsConstantBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+PassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AdditionalLightsShadowCasterPass_PassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub shadowmapID: i32,
    pub emptyShadowmap: bool,
    pub useStructuredBuffer: bool,
    pub stripShadowsOffVariants: bool,
    pub viewMatrix: crate::UnityEngine::Matrix4x4,
    pub allocatedShadowAtlasSize: crate::UnityEngine::Vector2Int,
    pub shadowmapTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub lightData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalLightData>,
    pub shadowData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalShadowData>,
    pub pass: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass,
    >,
    pub shadowRendererLists: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Rendering::RendererList>,
    >,
    pub shadowRendererListsHdl: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+PassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "AdditionalLightsShadowCasterPass/PassData";
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
    feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+PassData"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+PassData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+PassData"
)]
impl crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+AdditionalLightsShadowCasterPass+PassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::AdditionalLightsShadowCasterPass_PassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
