#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshBuilderNative {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MeshBuilderNative =>
    "UnityEngine.UIElements"."MeshBuilderNative"
);
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MeshBuilderNative {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MeshBuilderNative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl crate::UnityEngine::UIElements::MeshBuilderNative {
    #[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
    pub type NativeColorPage = crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage;
    #[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
    pub type NativeRectParams = crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams;
    #[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
    pub type NativeBorderParams = crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams;
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MeshBuilderNative {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshBuilderNative_NativeBorderParams {
    pub rect: crate::UnityEngine::Rect,
    pub leftColor: crate::UnityEngine::Color,
    pub topColor: crate::UnityEngine::Color,
    pub rightColor: crate::UnityEngine::Color,
    pub bottomColor: crate::UnityEngine::Color,
    pub leftWidth: f32,
    pub topWidth: f32,
    pub rightWidth: f32,
    pub bottomWidth: f32,
    pub topLeftRadius: crate::UnityEngine::Vector2,
    pub topRightRadius: crate::UnityEngine::Vector2,
    pub bottomRightRadius: crate::UnityEngine::Vector2,
    pub bottomLeftRadius: crate::UnityEngine::Vector2,
    pub leftColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
    pub topColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
    pub rightColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
    pub bottomColorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams =>
    "UnityEngine.UIElements"."MeshBuilderNative/NativeBorderParams"
);
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeBorderParams")]
impl crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams {}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshBuilderNative_NativeColorPage {
    pub isValid: i32,
    pub pageAndID: crate::UnityEngine::Color32,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage =>
    "UnityEngine.UIElements"."MeshBuilderNative/NativeColorPage"
);
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeColorPage")]
impl crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage {}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshBuilderNative_NativeRectParams {
    pub rect: crate::UnityEngine::Rect,
    pub subRect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
    pub uvRegion: crate::UnityEngine::Rect,
    pub color: crate::UnityEngine::Color,
    pub scaleMode: crate::UnityEngine::ScaleMode,
    pub topLeftRadius: crate::UnityEngine::Vector2,
    pub topRightRadius: crate::UnityEngine::Vector2,
    pub bottomRightRadius: crate::UnityEngine::Vector2,
    pub bottomLeftRadius: crate::UnityEngine::Vector2,
    pub contentSize: crate::UnityEngine::Vector2,
    pub textureSize: crate::UnityEngine::Vector2,
    pub texturePixelsPerPoint: f32,
    pub leftSlice: i32,
    pub topSlice: i32,
    pub rightSlice: i32,
    pub bottomSlice: i32,
    pub sliceScale: f32,
    pub rectInset: crate::UnityEngine::Vector4,
    pub colorPage: crate::UnityEngine::UIElements::MeshBuilderNative_NativeColorPage,
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams =>
    "UnityEngine.UIElements"."MeshBuilderNative/NativeRectParams"
);
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshBuilderNative+NativeRectParams")]
impl crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams {}
