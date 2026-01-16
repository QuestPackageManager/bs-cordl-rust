#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRRenderTextureFormat")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum UnityXRRenderTextureFormat {
    #[default]
    kUnityXRRenderTextureFormatBGRA1010102 = 5i32,
    kUnityXRRenderTextureFormatBGRA32 = 1i32,
    kUnityXRRenderTextureFormatNone = 66i32,
    kUnityXRRenderTextureFormatR11G11B10_UFloat = 6i32,
    kUnityXRRenderTextureFormatR16G16B16A16_SFloat = 3i32,
    kUnityXRRenderTextureFormatRGB565 = 2i32,
    kUnityXRRenderTextureFormatRGBA1010102 = 4i32,
    kUnityXRRenderTextureFormatRGBA32 = 0i32,
    kUnityXRRenderTextureFormatReference = 64i32,
    kUnityXRRenderTextureFormatSoftReferenceMSAA = 65i32,
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRRenderTextureFormat")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::XR::OpenXR::API::UnityXRRenderTextureFormat
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR.OpenXR.API";
    const CLASS_NAME: &'static str = "UnityXRRenderTextureFormat";
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRRenderTextureFormat")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::XR::OpenXR::API::UnityXRRenderTextureFormat
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRRenderTextureFormat")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::XR::OpenXR::API::UnityXRRenderTextureFormat
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRRenderTextureFormat")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::XR::OpenXR::API::UnityXRRenderTextureFormat
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
#[cfg(feature = "cordl_class_UnityEngine+XR+OpenXR+API+UnityXRRenderTextureFormat")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::XR::OpenXR::API::UnityXRRenderTextureFormat
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
