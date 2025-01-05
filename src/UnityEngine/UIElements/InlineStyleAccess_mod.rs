#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
#[repr(C)]
#[derive(Debug)]
pub struct InlineStyleAccess {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleValueCollection,
    >,
    pub m_ValuesManaged: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged,
    >,
    pub _ve_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub m_HasInlineCursor: bool,
    pub m_InlineCursor: crate::UnityEngine::UIElements::StyleCursor,
    pub m_HasInlineTextShadow: bool,
    pub m_InlineTextShadow: crate::UnityEngine::UIElements::StyleTextShadow,
    pub m_HasInlineTransformOrigin: bool,
    pub m_InlineTransformOrigin: crate::UnityEngine::UIElements::StyleTransformOrigin,
    pub m_HasInlineTranslate: bool,
    pub m_InlineTranslateOperation: crate::UnityEngine::UIElements::StyleTranslate,
    pub m_HasInlineRotate: bool,
    pub m_InlineRotateOperation: crate::UnityEngine::UIElements::StyleRotate,
    pub m_HasInlineScale: bool,
    pub m_InlineScale: crate::UnityEngine::UIElements::StyleScale,
    pub m_HasInlineBackgroundSize: bool,
    pub m_InlineBackgroundSize: crate::UnityEngine::UIElements::StyleBackgroundSize,
    pub m_InlineRule: crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule,
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::InlineStyleAccess =>
    "UnityEngine.UIElements"."InlineStyleAccess"
);
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl std::ops::Deref for crate::UnityEngine::UIElements::InlineStyleAccess {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StyleValueCollection,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::InlineStyleAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl crate::UnityEngine::UIElements::InlineStyleAccess {
    #[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
    pub type InlineRule = crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule;
    pub fn ApplyFromComputedStyle(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyFromComputedStyle", (id, newStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyInlineStyles(
        &mut self,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInlineStyles", (computedStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleTranslate(
        &mut self,
        translate: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleTranslate", (translate))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleValue(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleSheets::StyleValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValueSet(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsValueSet", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (ve))?;
        Ok(__cordl_object.into())
    }
    pub fn RemoveInlineStyle(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("RemoveInlineStyle", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineRule(
        &mut self,
        sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
        rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInlineRule", (sheet, rule))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInlineTranslate(
        &mut self,
        inlineValue: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetInlineTranslate", (inlineValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleColor2(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetStyleValue", (id, inlineValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleEnum_1_3<T>(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleEnum_1<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetStyleValue", (id, inlineValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleFloat1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetStyleValue", (id, inlineValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleFont5(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleFont,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetStyleValue", (id, inlineValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleFontDefinition4(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleFontDefinition,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetStyleValue", (id, inlineValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStyleValue_StyleLength0(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        inlineValue: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SetStyleValue", (id, inlineValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineBackgroundSize(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleBackgroundSize,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetInlineBackgroundSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineCursor(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleCursor,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetInlineCursor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineRotate(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleRotate,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetInlineRotate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineScale(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleScale,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetInlineScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineTextShadow(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleTextShadow,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetInlineTextShadow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineTransformOrigin(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleTransformOrigin,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetInlineTransformOrigin", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInlineTranslate(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::StyleTranslate,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetInlineTranslate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_backgroundSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleBackgroundSize,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleBackgroundSize = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_backgroundSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_cursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleCursor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleCursor = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_cursor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_display(
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
        > = __cordl_object.invoke("UnityEngine.UIElements.IStyle.get_display", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_paddingTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_paddingTop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_rotate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleRotate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleRotate = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_rotate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleScale> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleScale = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_scale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_textShadow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTextShadow> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTextShadow = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_textShadow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_transformOrigin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StyleTransformOrigin,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTransformOrigin = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_transformOrigin", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_translate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleTranslate> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleTranslate = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_translate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_get_width(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::StyleLength> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::StyleLength = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_backgroundColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_backgroundColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderBottomColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomLeftRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStyle.set_borderBottomLeftRadius",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomRightRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStyle.set_borderBottomRightRadius",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderBottomWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderBottomWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderLeftColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderLeftColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderLeftWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderLeftWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderRightColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderRightColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderRightWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderRightWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderTopColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopLeftRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderTopLeftRadius", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopRightRadius(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderTopRightRadius", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_borderTopWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_borderTopWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_bottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_bottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_color(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_color", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_display(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::DisplayStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_display", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexBasis(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_flexBasis", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexDirection(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::FlexDirection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_flexDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexGrow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_flexGrow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_flexShrink(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_flexShrink", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_fontSize(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_fontSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_height(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_height", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_left(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_left", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginBottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_marginBottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginLeft(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_marginLeft", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginRight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_marginRight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_marginTop(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_marginTop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_maxHeight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_maxHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_maxWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_maxWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_minWidth(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_minWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_opacity(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFloat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_opacity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_overflow(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Overflow,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_overflow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingBottom(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_paddingBottom", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingLeft(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_paddingLeft", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingRight(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_paddingRight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_paddingTop(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_paddingTop", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_position(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Position,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_position", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_right(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_right", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_top(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_top", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_translate(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleTranslate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_translate", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityBackgroundImageTintColor(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleColor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnityEngine.UIElements.IStyle.set_unityBackgroundImageTintColor",
                (value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityFont(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFont,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_unityFont", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_unityFontDefinition(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleFontDefinition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_unityFontDefinition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_visibility(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleEnum_1<
            crate::UnityEngine::UIElements::Visibility,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_visibility", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnityEngine_UIElements_IStyle_set_width(
        &mut self,
        value: crate::UnityEngine::UIElements::StyleLength,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnityEngine.UIElements.IStyle.set_width", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_ve", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ve(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ve", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::InlineStyleAccess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IStyle>>
for crate::UnityEngine::UIElements::InlineStyleAccess {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IStyle> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IStyle>>
for crate::UnityEngine::UIElements::InlineStyleAccess {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IStyle> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InlineStyleAccess_InlineRule {
    pub sheet: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleSheet>,
    pub rule: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StyleRule>,
    pub propertyIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::InlineStyleAccess_InlineRule => "UnityEngine.UIElements"
    ."InlineStyleAccess/InlineRule"
);
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+InlineStyleAccess+InlineRule")]
impl crate::UnityEngine::UIElements::InlineStyleAccess_InlineRule {}
