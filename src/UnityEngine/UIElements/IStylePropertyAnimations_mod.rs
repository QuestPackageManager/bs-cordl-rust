#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
#[repr(C)]
#[derive(Debug)]
pub struct IStylePropertyAnimations {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::IStylePropertyAnimations => "UnityEngine.UIElements"
    ."IStylePropertyAnimations"
);
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IStylePropertyAnimations {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IStylePropertyAnimations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
impl crate::UnityEngine::UIElements::IStylePropertyAnimations {
    pub fn Start_f32_f32_0(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: f32,
        to: f32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_i32_i32_1(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: i32,
        to: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Length_Length2(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Length,
        to: crate::UnityEngine::UIElements::Length,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Color_Color3(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::Color,
        to: crate::UnityEngine::Color,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Background_Background4(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Background,
        to: crate::UnityEngine::UIElements::Background,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_FontDefinition_FontDefinition5(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::FontDefinition,
        to: crate::UnityEngine::UIElements::FontDefinition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Font_Font6(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: *mut crate::UnityEngine::Font,
        to: *mut crate::UnityEngine::Font,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_TextShadow_TextShadow7(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::TextShadow,
        to: crate::UnityEngine::UIElements::TextShadow,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Scale_Scale8(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Scale,
        to: crate::UnityEngine::UIElements::Scale,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Translate_Translate9(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Translate,
        to: crate::UnityEngine::UIElements::Translate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_Rotate_Rotate10(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::Rotate,
        to: crate::UnityEngine::UIElements::Rotate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_TransformOrigin_TransformOrigin11(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::TransformOrigin,
        to: crate::UnityEngine::UIElements::TransformOrigin,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_BackgroundPosition_BackgroundPosition12(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundPosition,
        to: crate::UnityEngine::UIElements::BackgroundPosition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_BackgroundRepeat_BackgroundRepeat13(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundRepeat,
        to: crate::UnityEngine::UIElements::BackgroundRepeat,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn Start_BackgroundSize_BackgroundSize14(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: crate::UnityEngine::UIElements::BackgroundSize,
        to: crate::UnityEngine::UIElements::BackgroundSize,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Start", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn get_completedAnimationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_completedAnimationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_runningAnimationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_runningAnimationCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn StartEnum(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        from: i32,
        to: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: *mut crate::System::Func_2<f32, f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartEnum", (id, from, to, durationMs, delayMs, easingCurve))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAnimation(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimation", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllAnimations(
        &mut self,
        outPropertyIds: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllAnimations", (outPropertyIds))?;
        Ok(__cordl_ret)
    }
    pub fn CancelAnimation(
        &mut self,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAnimation", (id))?;
        Ok(__cordl_ret)
    }
    pub fn get_runningAnimationCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_runningAnimationCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_completedAnimationCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_completedAnimationCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CancelAllAnimations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IStylePropertyAnimations")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IStylePropertyAnimations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
