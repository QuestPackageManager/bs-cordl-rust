#[cfg(feature = "UnityEngine+UIElements+Slider")]
#[repr(C)]
#[derive(Debug)]
pub struct Slider {
    __cordl_parent: crate::UnityEngine::UIElements::BaseSlider_1<f32>,
}
#[cfg(feature = "UnityEngine+UIElements+Slider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Slider =>
    "UnityEngine.UIElements"."Slider"
);
#[cfg(feature = "UnityEngine+UIElements+Slider")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Slider {
    type Target = crate::UnityEngine::UIElements::BaseSlider_1<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Slider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider")]
impl crate::UnityEngine::UIElements::Slider {
    #[cfg(feature = "UnityEngine+UIElements+Slider+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::Slider_UxmlTraits;
    #[cfg(feature = "UnityEngine+UIElements+Slider+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::Slider_UxmlFactory;
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
        Ok(__cordl_ret)
    }
    pub fn ComputeValueFromKey(
        &mut self,
        sliderKey: crate::UnityEngine::UIElements::BaseSlider_1_SliderKey<f32>,
        isShift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeValueFromKey", (sliderKey, isShift))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInputDeviceDelta", (delta, speed, startValue))?;
        Ok(__cordl_ret)
    }
    pub fn ParseStringToValue(
        &mut self,
        previousValue: *mut crate::System::String,
        newValue: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ParseStringToValue", (previousValue, newValue))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_f32_SliderDirection_f32_1(
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_f32_f32_SliderDirection_f32_2(
        &mut self,
        label: *mut crate::System::String,
        start: f32,
        end: f32,
        direction: crate::UnityEngine::UIElements::SliderDirection,
        pageSize: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, start, end, direction, pageSize))?;
        Ok(__cordl_ret)
    }
    pub fn SliderLerpUnclamped(
        &mut self,
        a: f32,
        b: f32,
        interpolant: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("SliderLerpUnclamped", (a, b, interpolant))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_f32_f32_SliderDirection_f32_1(
        start: f32,
        end: f32,
        direction: crate::UnityEngine::UIElements::SliderDirection,
        pageSize: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start, end, direction, pageSize))?;
        Ok(__cordl_object)
    }
    pub fn New_String_f32_f32_SliderDirection_f32_2(
        label: *mut crate::System::String,
        start: f32,
        end: f32,
        direction: crate::UnityEngine::UIElements::SliderDirection,
        pageSize: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, start, end, direction, pageSize))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::Slider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct Slider_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::Slider,
        *mut crate::UnityEngine::UIElements::Slider_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Slider_UxmlFactory =>
    "UnityEngine.UIElements"."Slider/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Slider_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::Slider,
        *mut crate::UnityEngine::UIElements::Slider_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Slider_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlFactory")]
impl crate::UnityEngine::UIElements::Slider_UxmlFactory {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Slider_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct Slider_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseSlider_1_UxmlTraits<f32>,
    pub m_LowValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_HighValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_PageSize: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_ShowInputField: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Direction: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::SliderDirection,
    >,
    pub m_Inverted: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Slider_UxmlTraits =>
    "UnityEngine.UIElements"."Slider/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::Slider_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseSlider_1_UxmlTraits<f32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::Slider_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlTraits")]
impl crate::UnityEngine::UIElements::Slider_UxmlTraits {
    pub fn Init(
        &mut self,
        ve: *mut crate::UnityEngine::UIElements::VisualElement,
        bag: *mut crate::UnityEngine::UIElements::IUxmlAttributes,
        cc: crate::UnityEngine::UIElements::CreationContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (ve, bag, cc))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+Slider+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::Slider_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
