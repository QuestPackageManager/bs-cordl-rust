#[cfg(feature = "PyramidBloomRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PyramidBloomRendererSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _shader: *mut crate::UnityEngine::Shader,
    pub _material: *mut crate::UnityEngine::Material,
    pub _pyramid: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::PyramidBloomRendererSO_Level,
    >,
    pub kIsScreenspaceEffectKeyword: *mut quest_hook::libil2cpp::Il2CppString,
    pub kLegacyAutoExposureKeyword: *mut quest_hook::libil2cpp::Il2CppString,
    pub _initialized: bool,
}
#[cfg(feature = "PyramidBloomRendererSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PyramidBloomRendererSO => ""
    ."PyramidBloomRendererSO"
);
#[cfg(feature = "PyramidBloomRendererSO")]
impl std::ops::Deref for crate::GlobalNamespace::PyramidBloomRendererSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PyramidBloomRendererSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PyramidBloomRendererSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PyramidBloomRendererSO")]
impl crate::GlobalNamespace::PyramidBloomRendererSO {
    pub const kMaxPyramidSize: i32 = 16i32;
    #[cfg(feature = "PyramidBloomRendererSO+Level")]
    pub type Level = crate::GlobalNamespace::PyramidBloomRendererSO_Level;
    #[cfg(feature = "PyramidBloomRendererSO+Pass")]
    pub type Pass = crate::GlobalNamespace::PyramidBloomRendererSO_Pass;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn RenderBloom__cordl_bool__cordl_bool__cordl_bool0(
        &mut self,
        src: *mut crate::UnityEngine::RenderTexture,
        dest: *mut crate::UnityEngine::RenderTexture,
        radius: f32,
        alphaWeights: bool,
        betterQuality: bool,
        gammaCorrection: bool,
        legacyAutoExposure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RenderBloom",
                (
                    src,
                    dest,
                    radius,
                    alphaWeights,
                    betterQuality,
                    gammaCorrection,
                    legacyAutoExposure,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn RenderBloom_f32_f32_f32__cordl_bool_f32_f32_f32_f32_PyramidBloomRendererSO_Pass_PyramidBloomRendererSO_Pass_PyramidBloomRendererSO_Pass_PyramidBloomRendererSO_Pass__cordl_bool__cordl_bool1(
        &mut self,
        src: *mut crate::UnityEngine::RenderTexture,
        dest: *mut crate::UnityEngine::RenderTexture,
        radius: f32,
        intensity: f32,
        autoExposureLimit: f32,
        downIntensityOffset: f32,
        uniformPyramidWeights: bool,
        downsampleOnFirstPass: bool,
        pyramidWeightsParam: f32,
        alphaWeights: f32,
        firstUpsampleBrightness: f32,
        finalUpsampleBrightness: f32,
        preFilterPass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        downsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        upsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        finalUpsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        legacyAutoExposure: bool,
        isScreenspaceEffect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RenderBloom",
                (
                    src,
                    dest,
                    radius,
                    intensity,
                    autoExposureLimit,
                    downIntensityOffset,
                    uniformPyramidWeights,
                    downsampleOnFirstPass,
                    pyramidWeightsParam,
                    alphaWeights,
                    firstUpsampleBrightness,
                    finalUpsampleBrightness,
                    preFilterPass,
                    downsamplePass,
                    upsamplePass,
                    finalUpsamplePass,
                    legacyAutoExposure,
                    isScreenspaceEffect,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PyramidBloomRendererSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PyramidBloomRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PyramidBloomRendererSO+Level")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PyramidBloomRendererSO_Level {
    pub down: *mut crate::UnityEngine::RenderTexture,
    pub up: *mut crate::UnityEngine::RenderTexture,
}
#[cfg(feature = "PyramidBloomRendererSO+Level")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PyramidBloomRendererSO_Level =>
    ""."PyramidBloomRendererSO/Level"
);
#[cfg(feature = "PyramidBloomRendererSO+Level")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PyramidBloomRendererSO_Level {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PyramidBloomRendererSO+Level")]
impl crate::GlobalNamespace::PyramidBloomRendererSO_Level {}
#[cfg(feature = "PyramidBloomRendererSO+Pass")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyramidBloomRendererSO_Pass {
    Bilinear = 9i32,
    BilinearGamma = 10i32,
    Downsample13 = 2i32,
    Downsample4 = 3i32,
    DownsampleBilinearGamma = 4i32,
    Prefilter13 = 0i32,
    Prefilter4 = 1i32,
    UpsampleBox = 6i32,
    UpsampleBoxGamma = 8i32,
    UpsampleTent = 5i32,
    UpsampleTentAndACESToneMapping = 12i32,
    UpsampleTentAndACESToneMappingGlobalIntensity = 13i32,
    UpsampleTentAndReinhardToneMapping = 11i32,
    UpsampleTentGamma = 7i32,
}
#[cfg(feature = "PyramidBloomRendererSO+Pass")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PyramidBloomRendererSO_Pass =>
    ""."PyramidBloomRendererSO/Pass"
);
