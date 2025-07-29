#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinRenderTextureType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuiltinRenderTextureType {
    #[default]
    BindableTexture = -1i32,
    BufferPtr = -3i32,
    CameraTarget = 2i32,
    CurrentActive = 1i32,
    Depth = 3i32,
    DepthNormals = 4i32,
    GBuffer0 = 10i32,
    GBuffer1 = 11i32,
    GBuffer2 = 12i32,
    GBuffer3 = 13i32,
    GBuffer4 = 16i32,
    GBuffer5 = 17i32,
    GBuffer6 = 18i32,
    GBuffer7 = 19i32,
    MotionVectors = 15i32,
    None = 0i32,
    PrepassLight = 8i32,
    PrepassLightSpec = 9i32,
    PrepassNormalsSpec = 7i32,
    PropertyName = -4i32,
    Reflections = 14i32,
    RenderTexture = -2i32,
    ResolvedDepth = 5i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinRenderTextureType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::BuiltinRenderTextureType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "BuiltinRenderTextureType";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinRenderTextureType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::BuiltinRenderTextureType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinRenderTextureType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::BuiltinRenderTextureType {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinRenderTextureType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::BuiltinRenderTextureType {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinRenderTextureType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::BuiltinRenderTextureType {
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
