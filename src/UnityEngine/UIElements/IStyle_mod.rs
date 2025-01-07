#[cfg(feature = "UnityEngine+UIElements+IStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct IStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IStyle")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UIElements::IStyle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "IStyle";
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
#[cfg(feature = "UnityEngine+UIElements+IStyle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IStyle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStyle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStyle")]
impl crate::UnityEngine::UIElements::IStyle {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_backgroundSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleBackgroundSize,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackgroundSize = __cordl_object
            .invoke("get_backgroundSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleCursor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleCursor = __cordl_object
            .invoke("get_cursor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_display(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::DisplayStyle,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::DisplayStyle,
        > = __cordl_object.invoke("get_display", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength = __cordl_object
            .invoke("get_paddingTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleRotate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleRotate = __cordl_object
            .invoke("get_rotate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleScale> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleScale = __cordl_object
            .invoke("get_scale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textShadow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTextShadow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTextShadow = __cordl_object
            .invoke("get_textShadow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transformOrigin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleTransformOrigin,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTransformOrigin = __cordl_object
            .invoke("get_transformOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTranslate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTranslate = __cordl_object
            .invoke("get_translate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength = __cordl_object
            .invoke("get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backgroundColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_backgroundColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderBottomColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderBottomColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderBottomLeftRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderBottomLeftRadius", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderBottomRightRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderBottomRightRadius", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderBottomWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderBottomWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderLeftColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderLeftColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderLeftWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderLeftWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderRightColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderRightColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderRightWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderRightWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderTopColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderTopColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderTopLeftRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderTopLeftRadius", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderTopRightRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderTopRightRadius", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_borderTopWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_borderTopWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_display(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::DisplayStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_display", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flexBasis(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flexBasis", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flexDirection(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::FlexDirection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flexDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flexGrow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flexGrow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flexShrink(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flexShrink", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fontSize(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fontSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_height(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_height", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_left(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_left", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_marginBottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_marginBottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_marginLeft(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_marginLeft", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_marginRight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_marginRight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_marginTop(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_marginTop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxHeight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_opacity(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_opacity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overflow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Overflow,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overflow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_paddingBottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_paddingBottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_paddingLeft(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_paddingLeft", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_paddingRight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_paddingRight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_paddingTop(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_paddingTop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Position,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_position", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_right(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_right", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_top(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_top", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_translate(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_translate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_unityBackgroundImageTintColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_unityBackgroundImageTintColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_unityFont(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFont,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_unityFont", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_unityFontDefinition(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFontDefinition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_unityFontDefinition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_visibility(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Visibility,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_visibility", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_width(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_width", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStyle")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::IStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
