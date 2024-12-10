#[cfg(feature = "UnityEngine+UIElements+SliderInt")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderInt {
    __cordl_parent: crate::UnityEngine::UIElements::BaseSlider_1<i32>,
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SliderInt =>
    "UnityEngine.UIElements"."SliderInt"
);
#[cfg(feature = "UnityEngine+UIElements+SliderInt")]
impl std::ops::Deref for crate::UnityEngine::UIElements::SliderInt {
    type Target = crate::UnityEngine::UIElements::BaseSlider_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::SliderInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt")]
impl crate::UnityEngine::UIElements::SliderInt {
    #[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::SliderInt_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::SliderInt_UxmlTraits;
    pub fn ApplyInputDeviceDelta(
        &mut self,
        delta: crate::UnityEngine::Vector3,
        speed: crate::UnityEngine::UIElements::DeltaSpeed,
        startValue: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInputDeviceDelta", (delta, speed, startValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeValueAndDirectionFromClick(
        &mut self,
        sliderLength: f32,
        dragElementLength: f32,
        dragElementPos: f32,
        dragElementLastPos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ComputeValueAndDirectionFromClick",
                (sliderLength, dragElementLength, dragElementPos, dragElementLastPos),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeValueFromKey(
        &mut self,
        sliderKey: crate::UnityEngine::UIElements::BaseSlider_1_SliderKey<i32>,
        isShift: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputeValueFromKey", (sliderKey, isShift))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString_i32_i32_SliderDirection_f32_1(
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        direction: crate::UnityEngine::UIElements::SliderDirection,
        pageSize: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, start, end, direction, pageSize))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseStringToValue(
        &mut self,
        previousValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ParseStringToValue", (previousValue, newValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn SliderLerpUnclamped(
        &mut self,
        a: i32,
        b: i32,
        interpolant: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("SliderLerpUnclamped", (a, b, interpolant))?;
        Ok(__cordl_ret.into())
    }
    pub fn SliderNormalizeValue(
        &mut self,
        currentValue: i32,
        lowerValue: i32,
        higherValue: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("SliderNormalizeValue", (currentValue, lowerValue, higherValue))?;
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
    pub fn _ctor_Il2CppString_i32_i32_SliderDirection_f32_1(
        &mut self,
        label: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        direction: crate::UnityEngine::UIElements::SliderDirection,
        pageSize: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, start, end, direction, pageSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pageSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pageSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pageSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pageSize", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::SliderInt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderInt_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::SliderInt,
        *mut crate::UnityEngine::UIElements::SliderInt_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SliderInt_UxmlFactory
    => "UnityEngine.UIElements"."SliderInt/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::SliderInt_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::SliderInt,
        *mut crate::UnityEngine::UIElements::SliderInt_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::SliderInt_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlFactory")]
impl crate::UnityEngine::UIElements::SliderInt_UxmlFactory {
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
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::SliderInt_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderInt_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseSlider_1_UxmlTraits<i32>,
    pub m_LowValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_HighValue: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_PageSize: *mut crate::UnityEngine::UIElements::UxmlIntAttributeDescription,
    pub m_ShowInputField: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
    pub m_Direction: *mut crate::UnityEngine::UIElements::UxmlEnumAttributeDescription_1<
        crate::UnityEngine::UIElements::SliderDirection,
    >,
    pub m_Inverted: *mut crate::UnityEngine::UIElements::UxmlBoolAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::SliderInt_UxmlTraits =>
    "UnityEngine.UIElements"."SliderInt/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::SliderInt_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseSlider_1_UxmlTraits<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::SliderInt_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlTraits")]
impl crate::UnityEngine::UIElements::SliderInt_UxmlTraits {
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
#[cfg(feature = "UnityEngine+UIElements+SliderInt+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::SliderInt_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
