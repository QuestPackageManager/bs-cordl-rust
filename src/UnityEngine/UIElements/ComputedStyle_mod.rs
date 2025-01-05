#[cfg(feature = "UnityEngine+UIElements+ComputedStyle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    pub inheritedData: crate::UnityEngine::UIElements::StyleDataRef_1<
        crate::UnityEngine::UIElements::InheritedData,
    >,
    pub layoutData: crate::UnityEngine::UIElements::StyleDataRef_1<
        crate::UnityEngine::UIElements::LayoutData,
    >,
    pub rareData: crate::UnityEngine::UIElements::StyleDataRef_1<
        crate::UnityEngine::UIElements::RareData,
    >,
    pub transformData: crate::UnityEngine::UIElements::StyleDataRef_1<
        crate::UnityEngine::UIElements::TransformData,
    >,
    pub transitionData: crate::UnityEngine::UIElements::StyleDataRef_1<
        crate::UnityEngine::UIElements::TransitionData,
    >,
    pub visualData: crate::UnityEngine::UIElements::StyleDataRef_1<
        crate::UnityEngine::UIElements::VisualData,
    >,
    pub yogaNode: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    pub customProperties: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        crate::UnityEngine::UIElements::StyleSheets::StylePropertyValue,
    >,
    pub matchingRulesHash: i64,
    pub dpiScaling: f32,
    pub computedTransitions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::ComputedTransitionProperty,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+ComputedStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ComputedStyle =>
    "UnityEngine.UIElements"."ComputedStyle"
);
#[cfg(feature = "UnityEngine+UIElements+ComputedStyle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::ComputedStyle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ComputedStyle")]
impl crate::UnityEngine::UIElements::ComputedStyle {
    pub fn Acquire(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::ComputedStyle> {
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Acquire",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyAllPropertyInitial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyAllPropertyInitial",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyCustomStyleProperty(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyCustomStyleProperty",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyFromComputedStyle(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyFromComputedStyle",
            (id, other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyGlobalKeyword_Gc_ByRefMut0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyGlobalKeyword",
            (reader, parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyGlobalKeyword_StylePropertyId_StyleKeyword_ByRefMut1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        keyword: crate::UnityEngine::UIElements::StyleKeyword,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyGlobalKeyword",
            (id, keyword, parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyInitialValue_Gc0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyInitialValue",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyInitialValue_StylePropertyId1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyInitialValue",
            (id),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyProperties(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyProperties",
            (reader, parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_Background7(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::Background,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_BackgroundPosition3(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::BackgroundPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_BackgroundRepeat4(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::BackgroundRepeat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_BackgroundSize5(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::BackgroundSize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_Color6(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_FontDefinition9(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::FontDefinition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_Gc8(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_Length0(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_Rotate13(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::Rotate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_Scale14(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::Scale,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_TextShadow10(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::TextShadow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_TransformOrigin12(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::TransformOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_Translate11(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: crate::UnityEngine::UIElements::Translate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_f32_1(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyPropertyAnimation_i32_2(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        newValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyPropertyAnimation",
            (ve, id, newValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleBackgroundSize(
        &mut self,
        backgroundSizeValue: crate::UnityEngine::UIElements::BackgroundSize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleBackgroundSize",
            (backgroundSizeValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleCursor(
        &mut self,
        cursor: crate::UnityEngine::UIElements::Cursor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleCursor",
            (cursor),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleRotate(
        &mut self,
        rotateValue: crate::UnityEngine::UIElements::Rotate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleRotate",
            (rotateValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleScale(
        &mut self,
        scaleValue: crate::UnityEngine::UIElements::Scale,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleScale",
            (scaleValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleTextShadow(
        &mut self,
        st: crate::UnityEngine::UIElements::TextShadow,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleTextShadow",
            (st),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleTransformOrigin(
        &mut self,
        st: crate::UnityEngine::UIElements::TransformOrigin,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleTransformOrigin",
            (st),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleTranslate(
        &mut self,
        translateValue: crate::UnityEngine::UIElements::Translate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleTranslate",
            (translateValue),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleValue(
        &mut self,
        sv: crate::UnityEngine::UIElements::StyleSheets::StyleValue,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleValue",
            (sv, parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyStyleValueManaged(
        &mut self,
        sv: crate::UnityEngine::UIElements::StyleSheets::StyleValueManaged,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyStyleValueManaged",
            (sv, parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyUnsetValue_Gc0(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyUnsetValue",
            (reader, parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyUnsetValue_StylePropertyId1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyUnsetValue",
            (id, parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareChanges(
        x: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        y: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::ComputedStyle>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::VersionChangeType,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::VersionChangeType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CompareChanges", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFrom(
        &mut self,
        other: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyFrom",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::ComputedStyle> {
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (parentStyle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInitial() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::ComputedStyle,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::ComputedStyle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInitial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeApply(
        &mut self,
        parentStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FinalizeApply",
            (parentStyle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Release",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveCustomStyleProperty(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyReader,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveCustomStyleProperty",
            (reader),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetComputedTransitions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ResetComputedTransitions",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAnimation(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        oldStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        newStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StartAnimation",
                (element, id, oldStyle, newStyle, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAnimationAllProperty(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        oldStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        newStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StartAnimationAllProperty",
                (element, oldStyle, newStyle, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAnimationInline(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        sv: crate::UnityEngine::UIElements::StyleSheets::StyleValue,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StartAnimationInline",
                (element, id, computedStyle, sv, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartAnimationInlineTranslate(
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        computedStyle: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::ComputedStyle,
        >,
        translate: crate::UnityEngine::UIElements::StyleTranslate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "StartAnimationInlineTranslate",
                (element, computedStyle, translate, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SyncWithLayout(
        &mut self,
        targetNode: quest_hook::libil2cpp::Gc<crate::UnityEngine::Yoga::YogaNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SyncWithLayout",
            (targetNode),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Align> {
        let __cordl_ret: crate::UnityEngine::UIElements::Align = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_alignContent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Align> {
        let __cordl_ret: crate::UnityEngine::UIElements::Align = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_alignItems",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Align> {
        let __cordl_ret: crate::UnityEngine::UIElements::Align = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_alignSelf",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_backgroundColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Background> {
        let __cordl_ret: crate::UnityEngine::UIElements::Background = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_backgroundImage",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundPositionX(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_backgroundPositionX",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundPositionY(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundPosition = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_backgroundPositionY",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundRepeat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundRepeat = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_backgroundRepeat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_backgroundSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundSize> {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_backgroundSize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderBottomColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomLeftRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderBottomLeftRadius",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomRightRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderBottomRightRadius",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderBottomWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderBottomWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderLeftColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderLeftColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderLeftWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderLeftWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderRightColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderRightColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderRightWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderRightWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderTopColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopLeftRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderTopLeftRadius",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopRightRadius(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderTopRightRadius",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_borderTopWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_borderTopWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bottom",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_color",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Cursor> {
        let __cordl_ret: crate::UnityEngine::UIElements::Cursor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_cursor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_customPropertiesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_customPropertiesCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_display(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::DisplayStyle> {
        let __cordl_ret: crate::UnityEngine::UIElements::DisplayStyle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_display",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexBasis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flexBasis",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FlexDirection> {
        let __cordl_ret: crate::UnityEngine::UIElements::FlexDirection = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flexDirection",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexGrow(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flexGrow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexShrink(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flexShrink",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flexWrap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Wrap> {
        let __cordl_ret: crate::UnityEngine::UIElements::Wrap = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_flexWrap",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fontSize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_fontSize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasTransition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hasTransition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_height(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_height",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_justifyContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Justify> {
        let __cordl_ret: crate::UnityEngine::UIElements::Justify = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_justifyContent",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_left(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_left",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_letterSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_letterSpacing",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginBottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_marginBottom",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_marginLeft",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_marginRight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_marginTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_marginTop",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxHeight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minHeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_minHeight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minWidth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_minWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_opacity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_opacity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overflow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::OverflowInternal,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::OverflowInternal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_overflow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingBottom(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_paddingBottom",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_paddingLeft",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_paddingRight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_paddingTop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_paddingTop",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Position> {
        let __cordl_ret: crate::UnityEngine::UIElements::Position = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_right(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_right",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Rotate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rotate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Scale> {
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_scale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textOverflow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextOverflow> {
        let __cordl_ret: crate::UnityEngine::UIElements::TextOverflow = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_textOverflow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_textShadow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextShadow> {
        let __cordl_ret: crate::UnityEngine::UIElements::TextShadow = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_textShadow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_top(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_top",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transformOrigin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TransformOrigin> {
        let __cordl_ret: crate::UnityEngine::UIElements::TransformOrigin = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transformOrigin",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionDelay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimeValue>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TimeValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transitionDelay",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionDuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimeValue>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::TimeValue,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transitionDuration",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::StylePropertyName>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StylePropertyName,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transitionProperty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transitionTimingFunction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EasingFunction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EasingFunction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transitionTimingFunction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_translate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Translate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_translate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityBackgroundImageTintColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityBackgroundImageTintColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFont(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityFont",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFontDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::FontDefinition> {
        let __cordl_ret: crate::UnityEngine::UIElements::FontDefinition = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityFontDefinition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityFontStyleAndWeight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::FontStyle> {
        let __cordl_ret: crate::UnityEngine::FontStyle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityFontStyleAndWeight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityOverflowClipBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::OverflowClipBox> {
        let __cordl_ret: crate::UnityEngine::UIElements::OverflowClipBox = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityOverflowClipBox",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityParagraphSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityParagraphSpacing",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceBottom(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unitySliceBottom",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceLeft(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unitySliceLeft",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceRight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unitySliceRight",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unitySliceScale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unitySliceTop(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unitySliceTop",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextAlign(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextAnchor> {
        let __cordl_ret: crate::UnityEngine::TextAnchor = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityTextAlign",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOutlineColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityTextOutlineColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOutlineWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityTextOutlineWidth",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unityTextOverflowPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::TextOverflowPosition,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::TextOverflowPosition = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unityTextOverflowPosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Visibility> {
        let __cordl_ret: crate::UnityEngine::UIElements::Visibility = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_visibility",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_whiteSpace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::WhiteSpace> {
        let __cordl_ret: crate::UnityEngine::UIElements::WhiteSpace = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_whiteSpace",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_width",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wordSpacing(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wordSpacing",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
