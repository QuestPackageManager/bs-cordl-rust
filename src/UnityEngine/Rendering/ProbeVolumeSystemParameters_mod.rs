#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeSystemParameters")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct ProbeVolumeSystemParameters {
    pub memoryBudget: crate::UnityEngine::Rendering::ProbeVolumeTextureMemoryBudget,
    pub blendingMemoryBudget: crate::UnityEngine::Rendering::ProbeVolumeBlendingTextureMemoryBudget,
    pub shBands: crate::UnityEngine::Rendering::ProbeVolumeSHBands,
    pub supportScenarios: bool,
    pub supportScenarioBlending: bool,
    pub supportGPUStreaming: bool,
    pub supportDiskStreaming: bool,
    pub probeDebugShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub probeSamplingDebugShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub probeSamplingDebugTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub probeSamplingDebugMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub offsetDebugShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub fragmentationDebugShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub scenarioBlendingShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub streamingUploadShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
    pub sceneData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProbeVolumeSceneData>,
    pub supportsRuntimeDebug: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeSystemParameters")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeVolumeSystemParameters
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeVolumeSystemParameters";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeSystemParameters")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ProbeVolumeSystemParameters
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeSystemParameters")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ProbeVolumeSystemParameters
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeSystemParameters")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ProbeVolumeSystemParameters
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeSystemParameters")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ProbeVolumeSystemParameters
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeSystemParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::ProbeVolumeSystemParameters
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeSystemParameters")]
impl crate::UnityEngine::Rendering::ProbeVolumeSystemParameters {}
