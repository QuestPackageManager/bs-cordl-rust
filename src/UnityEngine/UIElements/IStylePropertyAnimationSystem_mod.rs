#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct IStylePropertyAnimationSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IStylePropertyAnimationSystem => "UnityEngine.UIElements"
    ."IStylePropertyAnimationSystem"
);
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    pub fn CancelAllAnimations_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", ())?;
        Ok(__cordl_ret)
    }
    pub fn CancelAllAnimations_VisualElement1(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", (owner))?;
        Ok(__cordl_ret)
    }
    pub fn CancelAnimation(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAnimation", (owner, id))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllAnimations(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        propertyIds: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllAnimations", (owner, propertyIds))?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_BackgroundPosition_BackgroundPosition12(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundPosition,
        endValue: crate::UnityEngine::UIElements::BackgroundPosition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_BackgroundRepeat_BackgroundRepeat13(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundRepeat,
        endValue: crate::UnityEngine::UIElements::BackgroundRepeat,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_BackgroundSize_BackgroundSize14(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundSize,
        endValue: crate::UnityEngine::UIElements::BackgroundSize,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_Background_Background4(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Background,
        endValue: crate::UnityEngine::UIElements::Background,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_Color_Color3(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::Color,
        endValue: crate::UnityEngine::Color,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_FontDefinition_FontDefinition5(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::FontDefinition,
        endValue: crate::UnityEngine::UIElements::FontDefinition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_Font_Font6(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: *mut crate::UnityEngine::Font,
        endValue: *mut crate::UnityEngine::Font,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_Length_Length2(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Length,
        endValue: crate::UnityEngine::UIElements::Length,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_Rotate_Rotate11(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Rotate,
        endValue: crate::UnityEngine::UIElements::Rotate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_Scale_Scale8(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Scale,
        endValue: crate::UnityEngine::UIElements::Scale,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_TextShadow_TextShadow7(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::TextShadow,
        endValue: crate::UnityEngine::UIElements::TextShadow,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_TransformOrigin_TransformOrigin9(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::TransformOrigin,
        endValue: crate::UnityEngine::UIElements::TransformOrigin,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_Translate_Translate10(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Translate,
        endValue: crate::UnityEngine::UIElements::Translate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_f32_f32_0(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: f32,
        endValue: f32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartTransition_i32_i32_1(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: i32,
        endValue: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAnimation(
        &mut self,
        owner: *mut crate::UnityEngine::UIElements::VisualElement,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimation", (owner, id))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimationSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
