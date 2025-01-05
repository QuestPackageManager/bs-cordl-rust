#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct InitialStyle {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::InitialStyle =>
    "UnityEngine.UIElements.StyleSheets"."InitialStyle"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    pub fn Acquire() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ComputedStyle,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Acquire", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Get() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::ComputedStyle>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignContent() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Align,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Align = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_alignContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignItems() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Align,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Align = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_alignItems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignSelf() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Align,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Align = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_alignSelf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundImage() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Background,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Background = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundImage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundPositionX() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundPositionX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundPositionY() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundPositionY", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundRepeat() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundRepeat = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundRepeat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundSize() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundSize,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_backgroundSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderBottomColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomLeftRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderBottomLeftRadius", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomRightRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderBottomRightRadius", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomWidth() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderBottomWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderLeftColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderLeftColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderLeftWidth() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderLeftWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderRightColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderRightColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderRightWidth() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderRightWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderTopColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopLeftRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderTopLeftRadius", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopRightRadius() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderTopRightRadius", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopWidth() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_borderTopWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bottom() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Cursor,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Cursor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_cursor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_display() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DisplayStyle,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::DisplayStyle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_display", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexBasis() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_flexBasis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexDirection() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::FlexDirection,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::FlexDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_flexDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexGrow() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_flexGrow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexShrink() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_flexShrink", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexWrap() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Wrap,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Wrap = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_flexWrap", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fontSize() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_fontSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_height() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_height", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_justifyContent() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Justify,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Justify = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_justifyContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_left() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_left", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_letterSpacing() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_letterSpacing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginBottom() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_marginBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginLeft() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_marginLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginRight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_marginRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginTop() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_marginTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxHeight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxWidth() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_maxWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minHeight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_minHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minWidth() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_minWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_opacity() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_opacity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overflow() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::OverflowInternal,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::OverflowInternal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_overflow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingBottom() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_paddingBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingLeft() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_paddingLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingRight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_paddingRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingTop() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_paddingTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Position,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Position = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_right() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_right", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotate() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Rotate,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_rotate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scale() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Scale,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_scale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textOverflow() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TextOverflow,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::TextOverflow = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_textOverflow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textShadow() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TextShadow,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::TextShadow = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_textShadow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_top() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_top", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transformOrigin() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TransformOrigin,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::TransformOrigin = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_transformOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionDelay() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimeValue>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TimeValue,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_transitionDelay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionDuration() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimeValue>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TimeValue,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_transitionDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionProperty() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StylePropertyName>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StylePropertyName,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_transitionProperty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionTimingFunction() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EasingFunction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EasingFunction,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_transitionTimingFunction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translate() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Translate,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_translate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityBackgroundImageTintColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityBackgroundImageTintColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFont() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityFont", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFontDefinition() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::FontDefinition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityFontDefinition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFontStyleAndWeight() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::FontStyle,
    > {
        let __cordl_ret: crate::UnityEngine::FontStyle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityFontStyleAndWeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityOverflowClipBox() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::OverflowClipBox,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::OverflowClipBox = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityOverflowClipBox", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityParagraphSpacing() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityParagraphSpacing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceBottom() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unitySliceBottom", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceLeft() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unitySliceLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceRight() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unitySliceRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceScale() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unitySliceScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceTop() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unitySliceTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextAlign() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextAnchor,
    > {
        let __cordl_ret: crate::UnityEngine::TextAnchor = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityTextAlign", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOutlineColor() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Color,
    > {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityTextOutlineColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOutlineWidth() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityTextOutlineWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOverflowPosition() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TextOverflowPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::TextOverflowPosition = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_unityTextOverflowPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visibility() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Visibility,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Visibility = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_visibility", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteSpace() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::WhiteSpace,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::WhiteSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_whiteSpace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wordSpacing() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_wordSpacing", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+InitialStyle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleSheets::InitialStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
