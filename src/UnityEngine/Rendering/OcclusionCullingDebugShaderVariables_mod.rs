#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OcclusionCullingDebugShaderVariables {
    pub _DepthSizeInOccluderPixels: crate::UnityEngine::Vector4,
    pub _OccluderMipBounds: crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer,
    pub _OccluderMipLayoutSizeX: u32,
    pub _OccluderMipLayoutSizeY: u32,
    pub _OcclusionCullingDebugPad0: u32,
    pub _OcclusionCullingDebugPad1: u32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingDebugShaderVariables";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables {
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
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables {
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
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OcclusionCullingDebugShaderVariables")]
impl crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables {
    #[cfg(
        feature = "UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
    )]
    pub type __OccluderMipBounds_e__FixedBuffer = crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer;
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {
    pub FixedElementField: u32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "OcclusionCullingDebugShaderVariables/<_OccluderMipBounds>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {
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
    feature = "cordl_class_UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+OcclusionCullingDebugShaderVariables+__OccluderMipBounds_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::OcclusionCullingDebugShaderVariables___OccluderMipBounds_e__FixedBuffer {}
