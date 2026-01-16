#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeShadingParameters")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ProbeVolumeShadingParameters {
    pub normalBias: f32,
    pub viewBias: f32,
    pub scaleBiasByMinDistanceBetweenProbes: bool,
    pub samplingNoise: f32,
    pub weight: f32,
    pub leakReductionMode: crate::UnityEngine::Rendering::APVLeakReductionMode,
    pub frameIndexForNoise: i32,
    pub reflNormalizationLowerClamp: f32,
    pub reflNormalizationUpperClamp: f32,
    pub skyOcclusionIntensity: f32,
    pub skyOcclusionShadingDirection: bool,
    pub regionCount: i32,
    pub regionLayerMasks: crate::Unity::Mathematics::uint4,
    pub worldOffset: crate::UnityEngine::Vector3,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeShadingParameters")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::ProbeVolumeShadingParameters
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "ProbeVolumeShadingParameters";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeShadingParameters")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::ProbeVolumeShadingParameters
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeShadingParameters")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::ProbeVolumeShadingParameters
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeShadingParameters")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::ProbeVolumeShadingParameters
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeShadingParameters")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::ProbeVolumeShadingParameters
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+ProbeVolumeShadingParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::ProbeVolumeShadingParameters
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ProbeVolumeShadingParameters")]
impl crate::UnityEngine::Rendering::ProbeVolumeShadingParameters {}
