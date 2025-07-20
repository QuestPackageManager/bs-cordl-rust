#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpriteIntermediateRendererInfo {
    pub SpriteID: i32,
    pub TextureID: i32,
    pub MaterialID: i32,
    pub Color: crate::UnityEngine::Color,
    pub Transform: crate::UnityEngine::Matrix4x4,
    pub Bounds: crate::UnityEngine::Bounds,
    pub Layer: i32,
    pub SortingLayer: i32,
    pub SortingOrder: i32,
    pub SceneCullingMask: u64,
    pub IndexData: crate::System::IntPtr,
    pub VertexData: crate::System::IntPtr,
    pub IndexCount: i32,
    pub VertexCount: i32,
    pub ShaderChannelMask: i32,
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.U2D";
    const CLASS_NAME: &'static str = "SpriteIntermediateRendererInfo";
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
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {
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
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {
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
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {
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
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Experimental+U2D+SpriteIntermediateRendererInfo")]
impl crate::UnityEngine::Experimental::U2D::SpriteIntermediateRendererInfo {}
