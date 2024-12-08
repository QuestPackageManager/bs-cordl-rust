#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshGenerationContextUtils_BorderParams {
    pub rect: crate::UnityEngine::Rect,
    pub playmodeTintColor: crate::UnityEngine::Color,
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
    pub material: *mut crate::UnityEngine::Material,
    pub leftColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub topColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub rightColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub bottomColorPage: crate::UnityEngine::UIElements::ColorPage,
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams =>
    "UnityEngine.UIElements"."MeshGenerationContextUtils/BorderParams"
);
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
impl crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams {
    pub fn ToNativeParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToNativeParams",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshGenerationContextUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MeshGenerationContextUtils => "UnityEngine.UIElements"
    ."MeshGenerationContextUtils"
);
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MeshGenerationContextUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MeshGenerationContextUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
impl crate::UnityEngine::UIElements::MeshGenerationContextUtils {
    #[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
    pub type RectangleParams = crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams;
    #[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
    pub type BorderParams = crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams;
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MeshGenerationContextUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshGenerationContextUtils_RectangleParams {
    pub rect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
    pub color: crate::UnityEngine::Color,
    pub subRect: crate::UnityEngine::Rect,
    pub backgroundPositionX: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundPositionY: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundRepeat: crate::UnityEngine::UIElements::BackgroundRepeat,
    pub backgroundSize: crate::UnityEngine::UIElements::BackgroundSize,
    pub texture: *mut crate::UnityEngine::Texture,
    pub sprite: *mut crate::UnityEngine::Sprite,
    pub vectorImage: *mut crate::UnityEngine::UIElements::VectorImage,
    pub material: *mut crate::UnityEngine::Material,
    pub scaleMode: crate::UnityEngine::ScaleMode,
    pub playmodeTintColor: crate::UnityEngine::Color,
    pub topLeftRadius: crate::UnityEngine::Vector2,
    pub topRightRadius: crate::UnityEngine::Vector2,
    pub bottomRightRadius: crate::UnityEngine::Vector2,
    pub bottomLeftRadius: crate::UnityEngine::Vector2,
    pub contentSize: crate::UnityEngine::Vector2,
    pub textureSize: crate::UnityEngine::Vector2,
    pub leftSlice: i32,
    pub topSlice: i32,
    pub rightSlice: i32,
    pub bottomSlice: i32,
    pub sliceScale: f32,
    pub spriteGeomRect: crate::UnityEngine::Rect,
    pub rectInset: crate::UnityEngine::Vector4,
    pub colorPage: crate::UnityEngine::UIElements::ColorPage,
    pub meshFlags: crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams =>
    "UnityEngine.UIElements"."MeshGenerationContextUtils/RectangleParams"
);
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
impl crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams {
    pub fn HasRadius(&mut self, epsilon: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasRadius",
            (epsilon),
        )?;
        Ok(__cordl_ret)
    }
    pub fn HasSlices(&mut self, epsilon: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasSlices",
            (epsilon),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToNativeParams(
        &mut self,
        uvRegion: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToNativeParams",
            (uvRegion),
        )?;
        Ok(__cordl_ret)
    }
}
