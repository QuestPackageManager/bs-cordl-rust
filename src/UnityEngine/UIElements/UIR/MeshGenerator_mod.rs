#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MeshGenerator_BackgroundRepeatInstance {
    pub rect: crate::UnityEngine::Rect,
    pub backgroundRepeatRect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshGenerator/BackgroundRepeatInstance";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+BackgroundRepeatInstance")]
impl crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance {}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MeshGenerator_BorderParams {
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
    pub leftColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub topColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub rightColorPage: crate::UnityEngine::UIElements::ColorPage,
    pub bottomColorPage: crate::UnityEngine::UIElements::ColorPage,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshGenerator/BorderParams";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+BorderParams")]
impl crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams {
    pub fn ToNativeParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams,
                        0usize,
                    >("ToNativeParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToNativeParams", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::MeshBuilderNative_NativeBorderParams =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MeshGenerator_RectangleParams {
    pub rect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
    pub color: crate::UnityEngine::Color,
    pub subRect: crate::UnityEngine::Rect,
    pub backgroundRepeatRect: crate::UnityEngine::Rect,
    pub backgroundRepeatInstanceList: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::NativePagedList_1<
            crate::UnityEngine::UIElements::UIR::MeshGenerator_BackgroundRepeatInstance,
        >,
    >,
    pub backgroundRepeatInstanceListStartIndex: i32,
    pub backgroundRepeatInstanceListEndIndex: i32,
    pub backgroundPositionX: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundPositionY: crate::UnityEngine::UIElements::BackgroundPosition,
    pub backgroundRepeat: crate::UnityEngine::UIElements::BackgroundRepeat,
    pub backgroundSize: crate::UnityEngine::UIElements::BackgroundSize,
    pub texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub vectorImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshGenerator/RectangleParams";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+RectangleParams")]
impl crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams {
    pub fn AdjustSpriteUVsForScaleMode(
        containerRect: crate::UnityEngine::Rect,
        srcRect: crate::UnityEngine::Rect,
        spriteGeomRect: crate::UnityEngine::Rect,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        scaleMode: crate::UnityEngine::ScaleMode,
        rectOut: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        uvOut: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rect,
                        crate::UnityEngine::Rect,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        crate::UnityEngine::ScaleMode,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                    ), quest_hook::libil2cpp::Void, 7usize>(
                        "AdjustSpriteUVsForScaleMode"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AdjustSpriteUVsForScaleMode",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    containerRect,
                    srcRect,
                    spriteGeomRect,
                    sprite,
                    scaleMode,
                    rectOut,
                    uvOut,
                ),
            )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rect,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::ScaleMode,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "AdjustUVsForScaleMode"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AdjustUVsForScaleMode",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (rect, uv, texture, scaleMode, rectOut, uvOut))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPackingRotation(
        uv: crate::UnityEngine::Rect,
        rotation: crate::UnityEngine::SpritePackingRotation,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rect,
                        crate::UnityEngine::SpritePackingRotation,
                    ), crate::UnityEngine::Rect, 2usize>("ApplyPackingRotation")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ApplyPackingRotation",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect =
            unsafe { cordl_method_info.invoke_unchecked((), (uv, rotation))? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeGeomRect(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>),
                        crate::UnityEngine::Rect,
                        1usize,
                    >("ComputeGeomRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeGeomRect", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect =
            unsafe { cordl_method_info.invoke_unchecked((), (sprite))? };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeUVRect(
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>),
                        crate::UnityEngine::Rect,
                        1usize,
                    >("ComputeUVRect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ComputeUVRect", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect =
            unsafe { cordl_method_info.invoke_unchecked((), (sprite))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasRadius(&mut self, epsilon: f32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), bool, 1usize>("HasRadius")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasRadius",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (epsilon))? };
        Ok(__cordl_ret.into())
    }
    pub fn HasSlices(&mut self, epsilon: f32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(f32), bool, 1usize>("HasSlices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HasSlices",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, (epsilon))? };
        Ok(__cordl_ret.into())
    }
    pub fn MakeSprite(
        containerRect: crate::UnityEngine::Rect,
        subRect: crate::UnityEngine::Rect,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        scaleMode: crate::UnityEngine::ScaleMode,
        playModeTintColor: crate::UnityEngine::Color,
        hasRadius: bool,
        slices: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
        useForRepeat: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rect,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                        crate::UnityEngine::ScaleMode,
                        crate::UnityEngine::Color,
                        bool,
                        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
                        bool,
                    ), crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams, 8usize>(
                        "MakeSprite",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MakeSprite",
                            8usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    containerRect,
                    subRect,
                    sprite,
                    scaleMode,
                    playModeTintColor,
                    hasRadius,
                    slices,
                    useForRepeat,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeTextured(
        rect: crate::UnityEngine::Rect,
        uv: crate::UnityEngine::Rect,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        scaleMode: crate::UnityEngine::ScaleMode,
        playModeTintColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rect,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                        crate::UnityEngine::ScaleMode,
                        crate::UnityEngine::Color,
                    ), crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams, 5usize>(
                        "MakeTextured",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MakeTextured",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams = unsafe {
            cordl_method_info
                .invoke_unchecked((), (rect, uv, texture, scaleMode, playModeTintColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeVectorTextured(
        rect: crate::UnityEngine::Rect,
        uv: crate::UnityEngine::Rect,
        vectorImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
        scaleMode: crate::UnityEngine::ScaleMode,
        playModeTintColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rect,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
                        crate::UnityEngine::ScaleMode,
                        crate::UnityEngine::Color,
                    ), crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams, 5usize>(
                        "MakeVectorTextured",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MakeVectorTextured",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::MeshGenerator_RectangleParams = unsafe {
            cordl_method_info
                .invoke_unchecked((), (rect, uv, vectorImage, scaleMode, playModeTintColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RectIntersection(
        a: crate::UnityEngine::Rect,
        b: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rect, crate::UnityEngine::Rect),
                        crate::UnityEngine::Rect,
                        2usize,
                    >("RectIntersection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RectIntersection", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rect =
            unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToNativeParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
                        0usize,
                    >("ToNativeParams")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToNativeParams", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MeshGenerator_RepeatRectUV {
    pub rect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RepeatRectUV
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshGenerator/RepeatRectUV";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RepeatRectUV
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RepeatRectUV
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RepeatRectUV
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RepeatRectUV
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_RepeatRectUV
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+RepeatRectUV")]
impl crate::UnityEngine::UIElements::UIR::MeshGenerator_RepeatRectUV {}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MeshGenerator_TessellationJob {
    pub allocator: crate::UnityEngine::UIElements::TempMeshAllocator,
    pub jobParameters: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshGenerator/TessellationJob";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
impl crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob {
    pub fn DrawBorder(
        &mut self,
        node: crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
        borderParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("DrawBorder")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawBorder",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (node, borderParams))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRectangle(
        &mut self,
        node: crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        >,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawRectangle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRectangle",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (node, rectParams, tex))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawSprite(
        &mut self,
        node: crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        >,
        sprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawSprite")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawSprite",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (node, rectParams, sprite))? };
        Ok(__cordl_ret.into())
    }
    pub fn DrawVectorImage(
        &mut self,
        node: crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        >,
        vi: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
                    ), quest_hook::libil2cpp::Void, 3usize>("DrawVectorImage")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawVectorImage",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (node, rectParams, vi))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (i))? };
        Ok(__cordl_ret.into())
    }
    pub fn ExtractHandle<T>(
        &mut self,
        handlePtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), T, 1usize>("ExtractHandle")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ExtractHandle",
                            1usize
                        )
                    })
            });
        let __cordl_ret: T = unsafe { cordl_method_info.invoke_unchecked(self, (handlePtr))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
{
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+TessellationJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJob
{
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct MeshGenerator_TessellationJobParameters {
    pub isBorderJob: bool,
    pub rectParams: crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
    pub borderParams: crate::UnityEngine::UIElements::UIR::MeshGenerator_BorderParams,
    pub node: crate::UnityEngine::UIElements::UnsafeMeshGenerationNode,
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshGenerator/TessellationJobParameters";
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters
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
#[cfg(feature = "cordl_class_UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshGenerator+TessellationJobParameters")]
impl crate::UnityEngine::UIElements::UIR::MeshGenerator_TessellationJobParameters {}
