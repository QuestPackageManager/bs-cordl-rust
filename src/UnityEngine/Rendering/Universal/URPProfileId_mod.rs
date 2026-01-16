#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+URPProfileId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum URPProfileId {
    #[default]
    AdditionalLightsShadow = 3i32,
    BlitFinalToBackBuffer = 58i32,
    Bloom = 26i32,
    BokehDepthOfField = 21i32,
    ColorGradingLUT = 4i32,
    CopyColor = 5i32,
    CopyDepth = 6i32,
    DepthPrepass = 8i32,
    DrawDepthNormalPrepass = 7i32,
    DrawFullscreen = 31i32,
    DrawMotionVectors = 30i32,
    DrawOpaqueObjects = 10i32,
    DrawScreenSpaceUI = 12i32,
    DrawSkybox = 59i32,
    DrawTransparentObjects = 11i32,
    GaussianDepthOfField = 20i32,
    LensFlareDataDriven = 28i32,
    LensFlareDataDrivenComputeOcclusion = 27i32,
    LensFlareScreenSpace = 29i32,
    LightCookies = 14i32,
    MainLightShadow = 15i32,
    MotionBlur = 23i32,
    PaniniProjection = 24i32,
    RG_BloomDownsample = 51i32,
    RG_BloomPrefilter = 50i32,
    RG_BloomSetup = 49i32,
    RG_BloomUpsample = 52i32,
    RG_DOFBlurBokeh = 43i32,
    RG_DOFBlurH = 41i32,
    RG_DOFBlurV = 42i32,
    RG_DOFComposite = 45i32,
    RG_DOFComputeCOC = 39i32,
    RG_DOFDownscalePrefilter = 40i32,
    RG_DOFPostFilter = 44i32,
    RG_FinalBlit = 57i32,
    RG_FinalFSRScale = 56i32,
    RG_FinalSetup = 55i32,
    RG_MotionBlur = 48i32,
    RG_SMAABlendWeight = 36i32,
    RG_SMAAEdgeDetection = 35i32,
    RG_SMAAMaterialSetup = 34i32,
    RG_SMAANeighborhoodBlend = 37i32,
    RG_SetupDoF = 38i32,
    RG_SetupPostFX = 32i32,
    RG_StopNaNs = 33i32,
    RG_TAA = 46i32,
    RG_TAACopyHistory = 47i32,
    RG_UberPost = 54i32,
    RG_UberPostSetupBloomPass = 53i32,
    RecordRenderGraph = 13i32,
    RenderCameraStack = 2i32,
    ResolveShadows = 16i32,
    SMAA = 19i32,
    SSAO = 17i32,
    StopNaNs = 18i32,
    TemporalAA = 22i32,
    UberPostProcess = 25i32,
    UniversalRenderTotal = 0i32,
    UpdateReflectionProbeAtlas = 9i32,
    UpdateVolumeFramework = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+URPProfileId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::URPProfileId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "URPProfileId";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+URPProfileId")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::URPProfileId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+URPProfileId")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::URPProfileId {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+URPProfileId")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::URPProfileId {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+URPProfileId")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::URPProfileId {
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
