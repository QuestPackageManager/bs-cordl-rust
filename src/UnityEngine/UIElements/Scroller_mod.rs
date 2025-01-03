#[cfg(feature = "UnityEngine+UIElements+Scroller")]
#[repr(C)]
#[derive(Debug)]
pub struct Scroller {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement,
    pub valueChanged: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    pub _slider_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Slider,
    >,
    pub _lowButton_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::RepeatButton,
    >,
    pub _highButton_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::RepeatButton,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Scroller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Scroller =>
    "UnityEngine.UIElements"."Scroller"
);
#[cfg(feature = "UnityEngine+UIElements+Scroller")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Scroller {
    type Target = crate::UnityEngine::UIElements::VisualElement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Scroller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller")]
impl crate::UnityEngine::UIElements::Scroller {
    #[cfg(feature = "UnityEngine+UIElements+Scroller+ScrollerSlider")]
    pub type ScrollerSlider = crate::UnityEngine::UIElements::Scroller_ScrollerSlider;
    #[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::Scroller_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::Scroller_UxmlTraits;
    pub fn Adjust(
        &mut self,
        factor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Adjust", (factor))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_f32_f32_Action_1_SliderDirection1(
        lowValue: f32,
        highValue: f32,
        valueChanged: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
        direction: crate::UnityEngine::UIElements::SliderDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lowValue, highValue, valueChanged, direction))?;
        Ok(__cordl_object.into())
    }
    pub fn OnSliderValueChange(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::ChangeEvent_1<f32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSliderValueChange", (evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollPageDown_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollPageDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollPageDown_f32_1(
        &mut self,
        factor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollPageDown", (factor))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollPageUp_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollPageUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ScrollPageUp_f32_1(
        &mut self,
        factor: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollPageUp", (factor))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_Action_1_SliderDirection1(
        &mut self,
        lowValue: f32,
        highValue: f32,
        valueChanged: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
        direction: crate::UnityEngine::UIElements::SliderDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lowValue, highValue, valueChanged, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_valueChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_valueChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::RepeatButton>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::RepeatButton,
        > = __cordl_object.invoke("get_highButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_highValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lowButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::RepeatButton>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::RepeatButton,
        > = __cordl_object.invoke("get_lowButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lowValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lowValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_slider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Slider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::Slider,
        > = __cordl_object.invoke("get_slider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_valueChanged(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action_1<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_valueChanged", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_direction(
        &mut self,
        value: crate::UnityEngine::UIElements::SliderDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_direction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_highButton(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::RepeatButton>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_highButton", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_highValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_highValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lowButton(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::RepeatButton>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lowButton", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lowValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lowValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_slider(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::Slider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_slider", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_value", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Scroller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+ScrollerSlider")]
#[repr(C)]
#[derive(Debug)]
pub struct Scroller_ScrollerSlider {
    __cordl_parent: crate::UnityEngine::UIElements::Slider,
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+ScrollerSlider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Scroller_ScrollerSlider
    => "UnityEngine.UIElements"."Scroller/ScrollerSlider"
);
#[cfg(feature = "UnityEngine+UIElements+Scroller+ScrollerSlider")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Scroller_ScrollerSlider {
    type Target = crate::UnityEngine::UIElements::Slider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+ScrollerSlider")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Scroller_ScrollerSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+ScrollerSlider")]
impl crate::UnityEngine::UIElements::Scroller_ScrollerSlider {
    pub fn New(
        start: f32,
        end: f32,
        direction: crate::UnityEngine::UIElements::SliderDirection,
        pageSize: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start, end, direction, pageSize))?;
        Ok(__cordl_object.into())
    }
    pub fn SliderNormalizeValue(
        &mut self,
        currentValue: f32,
        lowerValue: f32,
        higherValue: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("SliderNormalizeValue", (currentValue, lowerValue, higherValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        start: f32,
        end: f32,
        direction: crate::UnityEngine::UIElements::SliderDirection,
        pageSize: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start, end, direction, pageSize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+ScrollerSlider")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Scroller_ScrollerSlider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Scroller_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::Scroller,
        *mut crate::UnityEngine::UIElements::Scroller_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Scroller_UxmlFactory =>
    "UnityEngine.UIElements"."Scroller/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Scroller_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::Scroller,
        *mut crate::UnityEngine::UIElements::Scroller_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Scroller_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlFactory")]
impl crate::UnityEngine::UIElements::Scroller_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Scroller_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct Scroller_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::VisualElement_UxmlTraits,
    pub m_LowValue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >,
    pub m_HighValue: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >,
    pub m_Direction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
            crate::UnityEngine::UIElements::SliderDirection,
        >,
    >,
    pub m_Value: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Scroller_UxmlTraits =>
    "UnityEngine.UIElements"."Scroller/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Scroller_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::VisualElement_UxmlTraits;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Scroller_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlTraits")]
impl crate::UnityEngine::UIElements::Scroller_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
        bag: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::IUxmlAttributes>,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
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
#[cfg(feature = "UnityEngine+UIElements+Scroller+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Scroller_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
