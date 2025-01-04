#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer
    => "UnityEngine.UIElements"
    ."StylePropertyAnimationSystem/ElementPropertyPair/EqualityComparer"
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
impl crate::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer {
    pub fn Equals(
        &mut self,
        x: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
        y: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    >,
>
for crate::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
)]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    >,
>
for crate::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_CurrentTimeMs: i64,
    pub m_Floats: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFloat,
    >,
    pub m_Ints: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesInt,
    >,
    pub m_Lengths: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesLength,
    >,
    pub m_Colors: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesColor,
    >,
    pub m_Backgrounds: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackground,
    >,
    pub m_FontDefinitions: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFontDefinition,
    >,
    pub m_Fonts: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFont,
    >,
    pub m_TextShadows: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTextShadow,
    >,
    pub m_Scale: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesScale,
    >,
    pub m_Rotate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesRotate,
    >,
    pub m_Translate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTranslate,
    >,
    pub m_TransformOrigin: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTransformOrigin,
    >,
    pub m_BackgroundPosition: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundPosition,
    >,
    pub m_BackgroundRepeat: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundRepeat,
    >,
    pub m_BackgroundSize: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundSize,
    >,
    pub m_AllValues: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values,
        >,
    >,
    pub m_PropertyToValues: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            *mut crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem => "UnityEngine.UIElements"
    ."StylePropertyAnimationSystem"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StylePropertyAnimationSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem {
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+AnimationDataSet_2"
    )]
    pub type AnimationDataSet_2<
        TTimingData: quest_hook::libil2cpp::Type,
        TStyleData: quest_hook::libil2cpp::Type,
    > = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2<
        TTimingData,
        TStyleData,
    >;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair"
    )]
    pub type ElementPropertyPair = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+TransitionState"
    )]
    pub type TransitionState = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_TransitionState;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values")]
    pub type Values = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackground"
    )]
    pub type ValuesBackground = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackground;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundPosition"
    )]
    pub type ValuesBackgroundPosition = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundPosition;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundRepeat"
    )]
    pub type ValuesBackgroundRepeat = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundRepeat;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundSize"
    )]
    pub type ValuesBackgroundSize = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundSize;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesColor")]
    pub type ValuesColor = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesColor;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesDiscrete_1"
    )]
    pub type ValuesDiscrete_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        T,
    >;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFloat")]
    pub type ValuesFloat = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFloat;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFont")]
    pub type ValuesFont = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFont;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFontDefinition"
    )]
    pub type ValuesFontDefinition = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFontDefinition;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesInt")]
    pub type ValuesInt = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesInt;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesLength")]
    pub type ValuesLength = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesLength;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesRotate")]
    pub type ValuesRotate = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesRotate;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesScale")]
    pub type ValuesScale = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesScale;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTextShadow"
    )]
    pub type ValuesTextShadow = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTextShadow;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTransformOrigin"
    )]
    pub type ValuesTransformOrigin = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTransformOrigin;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTranslate"
    )]
    pub type ValuesTranslate = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTranslate;
    #[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1")]
    pub type Values_1<T: quest_hook::libil2cpp::Type> = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        T,
    >;
    pub fn CancelAllAnimations_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelAllAnimations_VisualElement1(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", (owner))?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelAnimation(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAnimation", (owner, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn CurrentTimeMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("CurrentTimeMs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAnimations(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        propertyIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllAnimations", (owner, propertyIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreate<T>(
        &mut self,
        values: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetOrCreate", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn StartTransition_BackgroundPosition_BackgroundPosition13(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundPosition,
        endValue: crate::UnityEngine::UIElements::BackgroundPosition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_BackgroundRepeat_BackgroundRepeat14(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundRepeat,
        endValue: crate::UnityEngine::UIElements::BackgroundRepeat,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_BackgroundSize_BackgroundSize15(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::BackgroundSize,
        endValue: crate::UnityEngine::UIElements::BackgroundSize,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Background_Background5(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Background,
        endValue: crate::UnityEngine::UIElements::Background,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Color_Color4(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::Color,
        endValue: crate::UnityEngine::Color,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_FontDefinition_FontDefinition6(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::FontDefinition,
        endValue: crate::UnityEngine::UIElements::FontDefinition,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Font_Font7(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        endValue: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Length_Length3(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Length,
        endValue: crate::UnityEngine::UIElements::Length,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Rotate_Rotate10(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Rotate,
        endValue: crate::UnityEngine::UIElements::Rotate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Scale_Scale9(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Scale,
        endValue: crate::UnityEngine::UIElements::Scale,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_T_T_StylePropertyAnimationSystem_Values_1_0<T>(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: T,
        endValue: T,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
        values: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (
                    owner,
                    prop,
                    startValue,
                    endValue,
                    durationMs,
                    delayMs,
                    easingCurve,
                    values,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_TextShadow_TextShadow8(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::TextShadow,
        endValue: crate::UnityEngine::UIElements::TextShadow,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_TransformOrigin_TransformOrigin12(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::TransformOrigin,
        endValue: crate::UnityEngine::UIElements::TransformOrigin,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_Translate_Translate11(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: crate::UnityEngine::UIElements::Translate,
        endValue: crate::UnityEngine::UIElements::Translate,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_f32_f32_1(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: f32,
        endValue: f32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition_i32_i32_2(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: i32,
        endValue: i32,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (owner, prop, startValue, endValue, durationMs, delayMs, easingCurve),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnimation(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimation", (owner, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTracking<T>(
        &mut self,
        values: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTracking", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
impl AsRef<crate::UnityEngine::UIElements::IStylePropertyAnimationSystem>
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem")]
impl AsMut<crate::UnityEngine::UIElements::IStylePropertyAnimationSystem>
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::IStylePropertyAnimationSystem {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+AnimationDataSet_2"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StylePropertyAnimationSystem_AnimationDataSet_2<
    TTimingData: quest_hook::libil2cpp::Type,
    TStyleData: quest_hook::libil2cpp::Type,
> {
    pub elements: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::UIElements::VisualElement,
        >,
    >,
    pub properties: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        >,
    >,
    pub timing: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<TTimingData>,
    >,
    pub style: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TStyleData>>,
    pub count: i32,
    pub indices: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
            i32,
        >,
    >,
    __cordl_phantom_TTimingData: std::marker::PhantomData<TTimingData>,
    __cordl_phantom_TStyleData: std::marker::PhantomData<TStyleData>,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+AnimationDataSet_2"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2 <
    TTimingData, TStyleData > => "UnityEngine.UIElements"
    ."StylePropertyAnimationSystem/AnimationDataSet`2<TTimingData,TStyleData>" <
    TTimingData, TStyleData >
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+AnimationDataSet_2"
)]
unsafe impl<
    TTimingData: quest_hook::libil2cpp::Type,
    TStyleData: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2<
    TTimingData,
    TStyleData,
> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+AnimationDataSet_2"
)]
impl<
    TTimingData: quest_hook::libil2cpp::Type,
    TStyleData: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2<
    TTimingData,
    TStyleData,
> {
    pub fn Add(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        timingData: TTimingData,
        styleData: TStyleData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Add",
            (owner, prop, timingData, styleData),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Create() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2<
            TTimingData,
            TStyleData,
        >,
    >
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2<
            TTimingData,
            TStyleData,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActivePropertiesForElement(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        outProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetActivePropertiesForElement",
            (ve, outProperties),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        index: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IndexOf",
            (ve, prop, index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalInit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "LocalInit",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        &mut self,
        cancelledIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Remove",
            (cancelledIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAll_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveAll",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAll_VisualElement0(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RemoveAll",
            (ve),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace(
        &mut self,
        index: i32,
        timingData: TTimingData,
        styleData: TStyleData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Replace",
            (index, timingData, styleData),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_capacity(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_capacity",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_capacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TTimingData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TStyleData: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_capacity",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StylePropertyAnimationSystem_ElementPropertyPair {
    pub element: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub property: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ElementPropertyPair"
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair"
)]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair {
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ElementPropertyPair+EqualityComparer"
    )]
    pub type EqualityComparer = crate::UnityEngine::UIElements::ElementPropertyPair_StylePropertyAnimationSystem_EqualityComparer;
    pub fn _ctor(
        &mut self,
        element: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        >,
        property: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (element, property),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+TransitionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StylePropertyAnimationSystem_TransitionState {
    #[default]
    Canceled = 8i32,
    Ended = 4i32,
    None = 0i32,
    Running = 1i32,
    Started = 2i32,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+TransitionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_TransitionState =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/TransitionState"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_Values {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_Values =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/Values"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values {
    pub fn CancelAllAnimations_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelAllAnimations_VisualElement1(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelAnimation(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAnimation", (ve, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAnimations(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        outPropertyIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllAnimations", (ve, outPropertyIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
        currentTimeMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (currentTimeMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnimation(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimation", (ve, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackground")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesBackground {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::Background,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackground")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackground =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesBackground"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackground")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackground {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::Background,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackground")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackground {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackground")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackground {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackground")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackground {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundPosition"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesBackgroundPosition {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::BackgroundPosition,
    >,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundPosition"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundPosition =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesBackgroundPosition"
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundPosition"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundPosition {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::BackgroundPosition,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundPosition"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundPosition"
)]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundPosition {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundPosition"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundPosition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundRepeat"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesBackgroundRepeat {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    >,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundRepeat"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundRepeat =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesBackgroundRepeat"
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundRepeat"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundRepeat {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::BackgroundRepeat,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundRepeat"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundRepeat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundRepeat"
)]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundRepeat {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundRepeat"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundRepeat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundSize"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesBackgroundSize {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::BackgroundSize,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            crate::UnityEngine::UIElements::BackgroundSize,
            crate::UnityEngine::UIElements::BackgroundSize,
            bool,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundSize"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundSize =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesBackgroundSize"
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundSize"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundSize {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::BackgroundSize,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundSize"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundSize"
)]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundSize {
    pub fn ConvertUnits(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        a: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundSize,
        >,
        b: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::BackgroundSize,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ConvertUnits", (owner, prop, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSame(
        a: crate::UnityEngine::UIElements::BackgroundSize,
        b: crate::UnityEngine::UIElements::BackgroundSize,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::UIElements::BackgroundSize,
        b: crate::UnityEngine::UIElements::BackgroundSize,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::BackgroundSize> {
        let __cordl_ret: crate::UnityEngine::UIElements::BackgroundSize = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::BackgroundSize,
                crate::UnityEngine::UIElements::BackgroundSize,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::BackgroundSize,
                crate::UnityEngine::UIElements::BackgroundSize,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesBackgroundSize"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesBackgroundSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesColor")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesColor {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::Color,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<crate::UnityEngine::Color, crate::UnityEngine::Color, bool>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesColor =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesColor"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesColor")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesColor {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::Color,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesColor")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesColor")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesColor {
    pub fn IsSame(
        c: crate::UnityEngine::Color,
        d: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (c, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::Color,
        b: crate::UnityEngine::Color,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::Color,
                crate::UnityEngine::Color,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::Color,
                crate::UnityEngine::Color,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesColor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesDiscrete_1")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesDiscrete_1<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        T,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<T, T, bool>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesDiscrete_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1 < T > =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesDiscrete`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesDiscrete_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<T> {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        T,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesDiscrete_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesDiscrete_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<T> {
    pub fn IsSame(a: T, b: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(a: T, b: T, t: f32) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, bool>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, bool>> = __cordl_object
            .invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesDiscrete_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFloat")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesFloat {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        f32,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<f32, f32, bool>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFloat")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFloat =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesFloat"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFloat")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFloat {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        f32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFloat")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFloat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFloat")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFloat {
    pub fn IsSame(a: f32, b: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(a: f32, b: f32, t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_3<f32, f32, bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<f32, f32, bool>,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFloat")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFloat {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFont")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesFont {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        *mut crate::UnityEngine::Font,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFont")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFont =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesFont"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFont")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFont {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        *mut crate::UnityEngine::Font,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFont")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFont {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFont")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFont {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFont")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFont {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFontDefinition"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesFontDefinition {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::FontDefinition,
    >,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFontDefinition"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFontDefinition =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesFontDefinition"
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFontDefinition"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFontDefinition {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesDiscrete_1<
        crate::UnityEngine::UIElements::FontDefinition,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFontDefinition"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFontDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFontDefinition"
)]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFontDefinition {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesFontDefinition"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesFontDefinition {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesInt")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesInt {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        i32,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<i32, i32, bool>,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesInt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesInt =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesInt"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesInt")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesInt {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        i32,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesInt")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesInt")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesInt {
    pub fn IsSame(a: i32, b: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(a: i32, b: i32, t: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_3<i32, i32, bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<i32, i32, bool>,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesInt")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesInt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesLength")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesLength {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Length,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            crate::UnityEngine::UIElements::Length,
            crate::UnityEngine::UIElements::Length,
            bool,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesLength")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesLength =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesLength"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesLength")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesLength {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Length,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesLength")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesLength")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesLength {
    pub fn ConvertUnits(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Length>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ConvertUnits", (owner, prop, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSame(
        a: crate::UnityEngine::UIElements::Length,
        b: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::UIElements::Length,
        b: crate::UnityEngine::UIElements::Length,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Length,
                crate::UnityEngine::UIElements::Length,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Length,
                crate::UnityEngine::UIElements::Length,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesLength")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesLength {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesRotate")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesRotate {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Rotate,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            crate::UnityEngine::UIElements::Rotate,
            crate::UnityEngine::UIElements::Rotate,
            bool,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesRotate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesRotate =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesRotate"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesRotate")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesRotate {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Rotate,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesRotate")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesRotate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesRotate")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesRotate {
    pub fn IsSame(
        a: crate::UnityEngine::UIElements::Rotate,
        b: crate::UnityEngine::UIElements::Rotate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::UIElements::Rotate,
        b: crate::UnityEngine::UIElements::Rotate,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Rotate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Rotate,
                crate::UnityEngine::UIElements::Rotate,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Rotate,
                crate::UnityEngine::UIElements::Rotate,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesRotate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesRotate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesScale")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesScale {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Scale,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            crate::UnityEngine::UIElements::Scale,
            crate::UnityEngine::UIElements::Scale,
            bool,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesScale")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesScale =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesScale"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesScale")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesScale {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Scale,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesScale")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesScale")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesScale {
    pub fn IsSame(
        a: crate::UnityEngine::UIElements::Scale,
        b: crate::UnityEngine::UIElements::Scale,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::UIElements::Scale,
        b: crate::UnityEngine::UIElements::Scale,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Scale> {
        let __cordl_ret: crate::UnityEngine::UIElements::Scale = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Scale,
                crate::UnityEngine::UIElements::Scale,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Scale,
                crate::UnityEngine::UIElements::Scale,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesScale")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesScale {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTextShadow")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesTextShadow {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::TextShadow,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            crate::UnityEngine::UIElements::TextShadow,
            crate::UnityEngine::UIElements::TextShadow,
            bool,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTextShadow")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTextShadow =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesTextShadow"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTextShadow")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTextShadow {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::TextShadow,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTextShadow")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTextShadow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTextShadow")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTextShadow {
    pub fn IsSame(
        a: crate::UnityEngine::UIElements::TextShadow,
        b: crate::UnityEngine::UIElements::TextShadow,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::UIElements::TextShadow,
        b: crate::UnityEngine::UIElements::TextShadow,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TextShadow> {
        let __cordl_ret: crate::UnityEngine::UIElements::TextShadow = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::TextShadow,
                crate::UnityEngine::UIElements::TextShadow,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::TextShadow,
                crate::UnityEngine::UIElements::TextShadow,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTextShadow")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTextShadow {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTransformOrigin"
)]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesTransformOrigin {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::TransformOrigin,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            crate::UnityEngine::UIElements::TransformOrigin,
            crate::UnityEngine::UIElements::TransformOrigin,
            bool,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTransformOrigin"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTransformOrigin =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesTransformOrigin"
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTransformOrigin"
)]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTransformOrigin {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::TransformOrigin,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTransformOrigin"
)]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTransformOrigin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTransformOrigin"
)]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTransformOrigin {
    pub fn ConvertUnits(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        a: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TransformOrigin,
        >,
        b: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TransformOrigin,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ConvertUnits", (owner, prop, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSame(
        a: crate::UnityEngine::UIElements::TransformOrigin,
        b: crate::UnityEngine::UIElements::TransformOrigin,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::UIElements::TransformOrigin,
        b: crate::UnityEngine::UIElements::TransformOrigin,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TransformOrigin> {
        let __cordl_ret: crate::UnityEngine::UIElements::TransformOrigin = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::TransformOrigin,
                crate::UnityEngine::UIElements::TransformOrigin,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::TransformOrigin,
                crate::UnityEngine::UIElements::TransformOrigin,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTransformOrigin"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTransformOrigin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTranslate")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_ValuesTranslate {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Translate,
    >,
    pub _SameFunc_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            crate::UnityEngine::UIElements::Translate,
            crate::UnityEngine::UIElements::Translate,
            bool,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTranslate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTranslate =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/ValuesTranslate"
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTranslate")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTranslate {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<
        crate::UnityEngine::UIElements::Translate,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTranslate")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTranslate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTranslate")]
impl crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTranslate {
    pub fn ConvertUnits(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Translate>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::UIElements::Translate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ConvertUnits", (owner, prop, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSame(
        a: crate::UnityEngine::UIElements::Translate,
        b: crate::UnityEngine::UIElements::Translate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSame", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::UIElements::Translate,
        b: crate::UnityEngine::UIElements::Translate,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Translate> {
        let __cordl_ret: crate::UnityEngine::UIElements::Translate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateComputedStyle_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateComputedStyle_i32_1(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateComputedStyle", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Translate,
                crate::UnityEngine::UIElements::Translate,
                bool,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                crate::UnityEngine::UIElements::Translate,
                crate::UnityEngine::UIElements::Translate,
                bool,
            >,
        > = __cordl_object.invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+ValuesTranslate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ValuesTranslate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1")]
#[repr(C)]
#[derive(Debug)]
pub struct StylePropertyAnimationSystem_Values_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values,
    pub m_CurrentTimeMs: i64,
    pub m_CurrentFrameEventsState: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
            T,
        >,
    >,
    pub m_NextFrameEventsState: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
            T,
        >,
    >,
    pub running: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2<
        crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TimingData<
            T,
        >,
        crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_StyleData<
            T,
        >,
    >,
    pub completed: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_AnimationDataSet_2<
        crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_EmptyData<
            T,
        >,
        T,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1 < T > =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/Values`1" < T >
);
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<T> {
    type Target = crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<T> {
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+EmptyData"
    )]
    pub type EmptyData = crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_EmptyData<
        T,
    >;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+StyleData"
    )]
    pub type StyleData = crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_StyleData<
        T,
    >;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TimingData"
    )]
    pub type TimingData = crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TimingData<
        T,
    >;
    #[cfg(
        feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TransitionEventsFrameState"
    )]
    pub type TransitionEventsFrameState = crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
        T,
    >;
    pub fn CancelAllAnimations_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelAllAnimations_VisualElement1(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAllAnimations", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelAnimation(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAnimation", (ve, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearEventQueue(
        &mut self,
        epp: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearEventQueue", (epp))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeReversingDelay(
        &mut self,
        delayMs: i32,
        newReversingShorteningFactor: f32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ComputeReversingDelay", (delayMs, newReversingShorteningFactor))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeReversingDuration(
        &mut self,
        newTransitionDurationMs: i32,
        newReversingShorteningFactor: f32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "ComputeReversingDuration",
                (newTransitionDurationMs, newReversingShorteningFactor),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeReversingShorteningFactor(
        &mut self,
        oldIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ComputeReversingShorteningFactor", (oldIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertUnits(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        a: quest_hook::libil2cpp::ByRefMut<T>,
        b: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ConvertUnits", (owner, prop, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceComputedStyleEndValue(
        &mut self,
        runningIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceComputedStyleEndValue", (runningIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllAnimations(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        outPropertyIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllAnimations", (ve, outPropertyIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessEventQueue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessEventQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueEvent(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
        epp: crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueEvent", (evt, epp))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueTransitionCancelEvent(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        runningIndex: i32,
        panelElapsedMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueTransitionCancelEvent", (ve, runningIndex, panelElapsedMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueTransitionEndEvent(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        runningIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueTransitionEndEvent", (ve, runningIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueTransitionRunEvent(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        runningIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueTransitionRunEvent", (ve, runningIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueTransitionStartEvent(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        runningIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("QueueTransitionStartEvent", (ve, runningIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTransitionCancelEvent(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        runningIndex: i32,
        panelElapsedMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendTransitionCancelEvent", (ve, runningIndex, panelElapsedMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartTransition(
        &mut self,
        owner: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        prop: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
        startValue: T,
        endValue: T,
        durationMs: i32,
        delayMs: i32,
        easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
        currentTimeMs: i64,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "StartTransition",
                (
                    owner,
                    prop,
                    startValue,
                    endValue,
                    durationMs,
                    delayMs,
                    easingCurve,
                    currentTimeMs,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SwapFrameStates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwapFrameStates", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        currentTimeMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (currentTimeMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnimation(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        id: crate::UnityEngine::UIElements::StyleSheets::StylePropertyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimation", (ve, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateProgress(
        &mut self,
        currentTimeMs: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateProgress", (currentTimeMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SameFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, bool>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Func_3<T, T, bool>> = __cordl_object
            .invoke("get_SameFunc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StylePropertyAnimationSystem_Values_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+EmptyData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Values_1_StylePropertyAnimationSystem_EmptyData<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+EmptyData"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_EmptyData < T > =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/Values`1/EmptyData<T>" < T >
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+EmptyData"
)]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_EmptyData<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+EmptyData"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_EmptyData<T> {}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+StyleData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Values_1_StylePropertyAnimationSystem_StyleData<
    T: quest_hook::libil2cpp::Type,
> {
    pub startValue: T,
    pub endValue: T,
    pub reversingAdjustedStartValue: T,
    pub currentValue: T,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+StyleData"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_StyleData < T > =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/Values`1/StyleData<T>" < T >
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+StyleData"
)]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_StyleData<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+StyleData"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_StyleData<T> {}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TimingData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Values_1_StylePropertyAnimationSystem_TimingData<
    T: quest_hook::libil2cpp::Type,
> {
    pub startTimeMs: i64,
    pub durationMs: i32,
    pub easingCurve: quest_hook::libil2cpp::Gc<crate::System::Func_2<f32, f32>>,
    pub easedProgress: f32,
    pub reversingShorteningFactor: f32,
    pub isStarted: bool,
    pub delayMs: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TimingData"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TimingData < T > =>
    "UnityEngine.UIElements"."StylePropertyAnimationSystem/Values`1/TimingData<T>" < T >
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TimingData"
)]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TimingData<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TimingData"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TimingData<T> {}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TransitionEventsFrameState"
)]
#[repr(C)]
#[derive(Debug)]
pub struct Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub elementPropertyStateDelta: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
            crate::UnityEngine::UIElements::StylePropertyAnimationSystem_TransitionState,
        >,
    >,
    pub elementPropertyQueuedEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::UnityEngine::UIElements::StylePropertyAnimationSystem_ElementPropertyPair,
            *mut crate::System::Collections::Generic::Queue_1<
                *mut crate::UnityEngine::UIElements::EventBase,
            >,
        >,
    >,
    pub panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IPanel>,
    pub m_ChangesCount: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TransitionEventsFrameState"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState
    < T > => "UnityEngine.UIElements"
    ."StylePropertyAnimationSystem/Values`1/TransitionEventsFrameState" < T >
);
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TransitionEventsFrameState"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
    T,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TransitionEventsFrameState"
)]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
    T,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TransitionEventsFrameState"
)]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
    T,
> {
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooledQueue() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Queue_1<
                *mut crate::UnityEngine::UIElements::EventBase,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Queue_1<
                *mut crate::UnityEngine::UIElements::EventBase,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetPooledQueue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StateChanged(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("StateChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterChange(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterChange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+StylePropertyAnimationSystem+Values_1+TransitionEventsFrameState"
)]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Values_1_StylePropertyAnimationSystem_TransitionEventsFrameState<
    T,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
