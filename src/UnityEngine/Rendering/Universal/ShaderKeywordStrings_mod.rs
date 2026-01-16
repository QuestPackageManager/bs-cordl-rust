#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderKeywordStrings")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderKeywordStrings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderKeywordStrings")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::ShaderKeywordStrings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ShaderKeywordStrings";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderKeywordStrings")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ShaderKeywordStrings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderKeywordStrings")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::ShaderKeywordStrings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderKeywordStrings")]
impl crate::UnityEngine::Rendering::Universal::ShaderKeywordStrings {
    pub const AdditionalLightShadows: &'static str = "_ADDITIONAL_LIGHT_SHADOWS";
    pub const AdditionalLightsPixel: &'static str = "_ADDITIONAL_LIGHTS";
    pub const AdditionalLightsVertex: &'static str = "_ADDITIONAL_LIGHTS_VERTEX";
    pub const BillboardFaceCameraPos: &'static str = "BILLBOARD_FACE_CAMERA_POS";
    pub const BlitSingleSlice: &'static str = "BLIT_SINGLE_SLICE";
    pub const BloomHQ: &'static str = "_BLOOM_HQ";
    pub const BloomHQDirt: &'static str = "_BLOOM_HQ_DIRT";
    pub const BloomLQ: &'static str = "_BLOOM_LQ";
    pub const BloomLQDirt: &'static str = "_BLOOM_LQ_DIRT";
    pub const CastingPunctualLightShadow: &'static str = "_CASTING_PUNCTUAL_LIGHT_SHADOW";
    pub const ChromaticAberration: &'static str = "_CHROMATIC_ABERRATION";
    pub const DBufferMRT1: &'static str = "_DBUFFER_MRT1";
    pub const DBufferMRT2: &'static str = "_DBUFFER_MRT2";
    pub const DBufferMRT3: &'static str = "_DBUFFER_MRT3";
    pub const DEBUG_DISPLAY: &'static str = "DEBUG_DISPLAY";
    pub const DIRLIGHTMAP_COMBINED: &'static str = "DIRLIGHTMAP_COMBINED";
    pub const DOWNSAMPLING_SIZE_16: &'static str = "DOWNSAMPLING_SIZE_16";
    pub const DOWNSAMPLING_SIZE_2: &'static str = "DOWNSAMPLING_SIZE_2";
    pub const DOWNSAMPLING_SIZE_4: &'static str = "DOWNSAMPLING_SIZE_4";
    pub const DOWNSAMPLING_SIZE_8: &'static str = "DOWNSAMPLING_SIZE_8";
    pub const DYNAMICLIGHTMAP_ON: &'static str = "DYNAMICLIGHTMAP_ON";
    pub const DecalLayers: &'static str = "_DECAL_LAYERS";
    pub const DecalNormalBlendHigh: &'static str = "_DECAL_NORMAL_BLEND_HIGH";
    pub const DecalNormalBlendLow: &'static str = "_DECAL_NORMAL_BLEND_LOW";
    pub const DecalNormalBlendMedium: &'static str = "_DECAL_NORMAL_BLEND_MEDIUM";
    pub const DepthMsaa2: &'static str = "_DEPTH_MSAA_2";
    pub const DepthMsaa4: &'static str = "_DEPTH_MSAA_4";
    pub const DepthMsaa8: &'static str = "_DEPTH_MSAA_8";
    pub const DepthNoMsaa: &'static str = "_DEPTH_NO_MSAA";
    pub const DisableTexture2DXArray: &'static str = "DISABLE_TEXTURE2D_X_ARRAY";
    pub const Distortion: &'static str = "_DISTORTION";
    pub const Dithering: &'static str = "_DITHERING";
    pub const EDITOR_VISUALIZATION: &'static str = "EDITOR_VISUALIZATION";
    pub const EVALUATE_SH_MIXED: &'static str = "EVALUATE_SH_MIXED";
    pub const EVALUATE_SH_VERTEX: &'static str = "EVALUATE_SH_VERTEX";
    pub const EasuRcasAndHDRInput: &'static str = "_EASU_RCAS_AND_HDR_INPUT";
    pub const FilmGrain: &'static str = "_FILM_GRAIN";
    pub const ForwardPlus: &'static str = "_FORWARD_PLUS";
    pub const FoveatedRenderingNonUniformRaster: &'static str = "_FOVEATED_RENDERING_NON_UNIFORM_RASTER";
    pub const Fxaa: &'static str = "_FXAA";
    pub const Gamma20: &'static str = "_GAMMA_20";
    pub const Gamma20AndHDRInput: &'static str = "_GAMMA_20_AND_HDR_INPUT";
    pub const HDRGrading: &'static str = "_HDR_GRADING";
    pub const HDROverlay: &'static str = "_HDR_OVERLAY";
    pub const HighQualitySampling: &'static str = "_HIGH_QUALITY_SAMPLING";
    pub const LIGHTMAP_ON: &'static str = "LIGHTMAP_ON";
    pub const LOD_FADE_CROSSFADE: &'static str = "LOD_FADE_CROSSFADE";
    pub const LightCookies: &'static str = "_LIGHT_COOKIES";
    pub const LightLayers: &'static str = "_LIGHT_LAYERS";
    pub const LightmapShadowMixing: &'static str = "LIGHTMAP_SHADOW_MIXING";
    pub const LinearToSRGBConversion: &'static str = "_LINEAR_TO_SRGB_CONVERSION";
    pub const MainLightShadowCascades: &'static str = "_MAIN_LIGHT_SHADOWS_CASCADE";
    pub const MainLightShadowScreen: &'static str = "_MAIN_LIGHT_SHADOWS_SCREEN";
    pub const MainLightShadows: &'static str = "_MAIN_LIGHT_SHADOWS";
    pub const MixedLightingSubtractive: &'static str = "_MIXED_LIGHTING_SUBTRACTIVE";
    pub const PaniniGeneric: &'static str = "_GENERIC";
    pub const PaniniUnitDistance: &'static str = "_UNIT_DISTANCE";
    pub const PointSampling: &'static str = "_POINT_SAMPLING";
    pub const ProbeVolumeL1: &'static str = "PROBE_VOLUMES_L1";
    pub const ProbeVolumeL2: &'static str = "PROBE_VOLUMES_L2";
    pub const Rcas: &'static str = "_RCAS";
    pub const ReflectionProbeBlending: &'static str = "_REFLECTION_PROBE_BLENDING";
    pub const ReflectionProbeBoxProjection: &'static str = "_REFLECTION_PROBE_BOX_PROJECTION";
    pub const RenderPassEnabled: &'static str = "_RENDER_PASS_ENABLED";
    pub const SCREEN_COORD_OVERRIDE: &'static str = "SCREEN_COORD_OVERRIDE";
    pub const ScreenSpaceOcclusion: &'static str = "_SCREEN_SPACE_OCCLUSION";
    pub const ShadowsShadowMask: &'static str = "SHADOWS_SHADOWMASK";
    pub const SmaaHigh: &'static str = "_SMAA_PRESET_HIGH";
    pub const SmaaLow: &'static str = "_SMAA_PRESET_LOW";
    pub const SmaaMedium: &'static str = "_SMAA_PRESET_MEDIUM";
    pub const SoftShadows: &'static str = "_SHADOWS_SOFT";
    pub const SoftShadowsHigh: &'static str = "_SHADOWS_SOFT_HIGH";
    pub const SoftShadowsLow: &'static str = "_SHADOWS_SOFT_LOW";
    pub const SoftShadowsMedium: &'static str = "_SHADOWS_SOFT_MEDIUM";
    pub const TonemapACES: &'static str = "_TONEMAP_ACES";
    pub const TonemapNeutral: &'static str = "_TONEMAP_NEUTRAL";
    pub const USE_LEGACY_LIGHTMAPS: &'static str = "USE_LEGACY_LIGHTMAPS";
    pub const USE_UNITY_CROSSFADE: &'static str = "USE_UNITY_CROSSFADE";
    pub const UseFastSRGBLinearConversion: &'static str = "_USE_FAST_SRGB_LINEAR_CONVERSION";
    pub const WriteRenderingLayers: &'static str = "_WRITE_RENDERING_LAYERS";
    pub const XROcclusionMeshCombined: &'static str = "XR_OCCLUSION_MESH_COMBINED";
    pub const _ADD_PRECOMPUTED_VELOCITY: &'static str = "_ADD_PRECOMPUTED_VELOCITY";
    pub const _ALPHAMODULATE_ON: &'static str = "_ALPHAMODULATE_ON";
    pub const _ALPHAPREMULTIPLY_ON: &'static str = "_ALPHAPREMULTIPLY_ON";
    pub const _ALPHATEST_ON: &'static str = "_ALPHATEST_ON";
    pub const _CLEARCOAT: &'static str = "_CLEARCOAT";
    pub const _CLEARCOATMAP: &'static str = "_CLEARCOATMAP";
    pub const _DEFERRED_FIRST_LIGHT: &'static str = "_DEFERRED_FIRST_LIGHT";
    pub const _DEFERRED_MAIN_LIGHT: &'static str = "_DEFERRED_MAIN_LIGHT";
    pub const _DEFERRED_MIXED_LIGHTING: &'static str = "_DEFERRED_MIXED_LIGHTING";
    pub const _DEFERRED_STENCIL: &'static str = "_DEFERRED_STENCIL";
    pub const _DETAIL_MULX2: &'static str = "_DETAIL_MULX2";
    pub const _DETAIL_SCALED: &'static str = "_DETAIL_SCALED";
    pub const _DIRECTIONAL: &'static str = "_DIRECTIONAL";
    pub const _EMISSION: &'static str = "_EMISSION";
    pub const _ENABLE_ALPHA_OUTPUT: &'static str = "_ENABLE_ALPHA_OUTPUT";
    pub const _GBUFFER_NORMALS_OCT: &'static str = "_GBUFFER_NORMALS_OCT";
    pub const _NORMALMAP: &'static str = "_NORMALMAP";
    pub const _OUTPUT_DEPTH: &'static str = "_OUTPUT_DEPTH";
    pub const _POINT: &'static str = "_POINT";
    pub const _RECEIVE_SHADOWS_OFF: &'static str = "_RECEIVE_SHADOWS_OFF";
    pub const _SPOT: &'static str = "_SPOT";
    pub const _SURFACE_TYPE_TRANSPARENT: &'static str = "_SURFACE_TYPE_TRANSPARENT";
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderKeywordStrings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::ShaderKeywordStrings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
