#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshGenerationContextUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MeshGenerationContextUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "MeshGenerationContextUtils";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MeshGenerationContextUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
    pub type BorderParams = crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams;
    #[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
    pub type RectangleParams = crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams;
    pub fn AdjustBackgroundSizeForBorders(
        visualElement: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AdjustBackgroundSizeForBorders", (visualElement, rectParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertBorderRadiusPercentToPoints(
        borderRectSize: crate::UnityEngine::Vector2,
        length: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertBorderRadiusPercentToPoints", (borderRectSize, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVisualElementRadii(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        topLeft: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        bottomLeft: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        topRight: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        bottomRight: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetVisualElementRadii",
                (ve, topLeft, bottomLeft, topRight, bottomRight),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Rectangle(
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rectangle", (mgc, rectParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn Text(
        mgc: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        >,
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Text", (mgc, te))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub leftColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub topColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub rightColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub bottomColorPage: crate::UnityEngine::UIElements::ColorPage,
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "BorderParams";
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
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams {
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
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+BorderParams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshGenerationContextUtils_RectangleParams {
    pub rect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
    pub color: crate::UnityEngine::Color,
    pub subRect: crate::UnityEngine::Rect,
    pub backgroundPositionX: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundPositionY: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundRepeat: crate::UnityEngine::UIElements::BackgroundRepeat,
    pub backgroundSize: crate::UnityEngine::UIElements::BackgroundSize,
    pub texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub vectorImage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VectorImage,
    >,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "RectangleParams";
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
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams {
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
#[cfg(feature = "UnityEngine+UIElements+MeshGenerationContextUtils+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn AdjustSpriteUVsForScaleMode(
        containerRect: crate::UnityEngine::Rect,
        srcRect: crate::UnityEngine::Rect,
        spriteGeomRect: crate::UnityEngine::Rect,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        scaleMode: crate::UnityEngine::ScaleMode,
        rectOut: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        uvOut: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AdjustSpriteUVsForScaleMode",
                (
                    containerRect,
                    srcRect,
                    spriteGeomRect,
                    sprite,
                    scaleMode,
                    rectOut,
                    uvOut,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AdjustUVsForScaleMode(
        rect: crate::UnityEngine::Rect,
        uv: crate::UnityEngine::Rect,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleMode: crate::UnityEngine::ScaleMode,
        rectOut: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        uvOut: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AdjustUVsForScaleMode",
                (rect, uv, texture, scaleMode, rectOut, uvOut),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPackingRotation(
        uv: crate::UnityEngine::Rect,
        rotation: crate::UnityEngine::SpritePackingRotation,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyPackingRotation", (uv, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeGeomRect(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeGeomRect", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeUVRect(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ComputeUVRect", (sprite))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasRadius(&mut self, epsilon: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasRadius",
            (epsilon),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn HasSlices(&mut self, epsilon: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "HasSlices",
            (epsilon),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeSprite(
        containerRect: crate::UnityEngine::Rect,
        subRect: crate::UnityEngine::Rect,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        scaleMode: crate::UnityEngine::ScaleMode,
        panelContext: crate::UnityEngine::UIElements::ContextType,
        hasRadius: bool,
        slices: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        useForRepeat: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MakeSprite",
                (
                    containerRect,
                    subRect,
                    sprite,
                    scaleMode,
                    panelContext,
                    hasRadius,
                    slices,
                    useForRepeat,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeTextured(
        rect: crate::UnityEngine::Rect,
        uv: crate::UnityEngine::Rect,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleMode: crate::UnityEngine::ScaleMode,
        panelContext: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeTextured", (rect, uv, texture, scaleMode, panelContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeVectorTextured(
        rect: crate::UnityEngine::Rect,
        uv: crate::UnityEngine::Rect,
        vectorImage: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VectorImage,
        >,
        scaleMode: crate::UnityEngine::ScaleMode,
        panelContext: crate::UnityEngine::UIElements::ContextType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MakeVectorTextured",
                (rect, uv, vectorImage, scaleMode, panelContext),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RectIntersection(
        a: crate::UnityEngine::Rect,
        b: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_ret: crate::UnityEngine::Rect = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RectIntersection", (a, b))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
