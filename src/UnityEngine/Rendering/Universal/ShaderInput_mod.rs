#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderInput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Rendering::Universal::ShaderInput {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ShaderInput";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderInput")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::ShaderInput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderInput")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::ShaderInput {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderInput")]
impl crate::UnityEngine::Rendering::Universal::ShaderInput {
    #[cfg(feature = "UnityEngine+Rendering+Universal+ShaderInput+LightData")]
    pub type LightData = crate::UnityEngine::Rendering::Universal::ShaderInput_LightData;
    #[cfg(feature = "UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
    pub type ShadowData = crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData;
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::Universal::ShaderInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+LightData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ShaderInput_LightData {
    pub position: crate::UnityEngine::Vector4,
    pub color: crate::UnityEngine::Vector4,
    pub attenuation: crate::UnityEngine::Vector4,
    pub spotDirection: crate::UnityEngine::Vector4,
    pub occlusionProbeChannels: crate::UnityEngine::Vector4,
    pub layerMask: u32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+LightData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ShaderInput_LightData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ShaderInput/LightData";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+LightData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::ShaderInput_LightData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+LightData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::ShaderInput_LightData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+LightData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::ShaderInput_LightData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+LightData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::ShaderInput_LightData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+LightData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::ShaderInput_LightData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderInput+LightData")]
impl crate::UnityEngine::Rendering::Universal::ShaderInput_LightData {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ShaderInput_ShadowData {
    pub worldToShadowMatrix: crate::UnityEngine::Matrix4x4,
    pub shadowParams: crate::UnityEngine::Vector4,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "ShaderInput/ShadowData";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+ShaderInput+ShadowData")]
impl crate::UnityEngine::Rendering::Universal::ShaderInput_ShadowData {}
