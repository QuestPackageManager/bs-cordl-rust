#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider")]
#[repr(C)]
#[derive(Debug)]
pub struct MinMaxSlider {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1<
        crate::UnityEngine::Vector2,
    >,
    pub _dragElement_k__BackingField: *mut crate::UnityEngine::UIElements::VisualElement,
    pub _dragMinThumb_k__BackingField: *mut crate::UnityEngine::UIElements::VisualElement,
    pub _dragMaxThumb_k__BackingField: *mut crate::UnityEngine::UIElements::VisualElement,
    pub _clampedDragger_k__BackingField: *mut crate::UnityEngine::UIElements::ClampedDragger_1<
        f32,
    >,
    pub m_DragElementStartPos: crate::UnityEngine::Vector2,
    pub m_ValueStartPos: crate::UnityEngine::Vector2,
    pub m_DragMinThumbRect: crate::UnityEngine::Rect,
    pub m_DragMaxThumbRect: crate::UnityEngine::Rect,
    pub m_DragState: crate::UnityEngine::UIElements::MinMaxSlider_DragState,
    pub m_MinLimit: f32,
    pub m_MaxLimit: f32,
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MinMaxSlider =>
    "UnityEngine.UIElements"."MinMaxSlider"
);
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MinMaxSlider {
    type Target = crate::UnityEngine::UIElements::BaseField_1<
        crate::UnityEngine::Vector2,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MinMaxSlider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider")]
impl crate::UnityEngine::UIElements::MinMaxSlider {
    #[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+DragState")]
    pub type DragState = crate::UnityEngine::UIElements::MinMaxSlider_DragState;
    #[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlFactory")]
    pub type UxmlFactory = crate::UnityEngine::UIElements::MinMaxSlider_UxmlFactory;
    #[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlTraits")]
    pub type UxmlTraits = crate::UnityEngine::UIElements::MinMaxSlider_UxmlTraits;
    pub fn ClampValues(
        &mut self,
        valueToClamp: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("ClampValues", (valueToClamp))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeValueFromDraggingThumb(
        &mut self,
        dragElementStartPos: f32,
        dragElementEndPos: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ComputeValueFromDraggingThumb",
                (dragElementStartPos, dragElementEndPos),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ComputeValueFromPosition(
        &mut self,
        positionToConvert: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ComputeValueFromPosition", (positionToConvert))?;
        Ok(__cordl_ret)
    }
    pub fn ExecuteDefaultAction(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::EventBase,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteDefaultAction", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String_f32_f32_f32_f32_1(
        label: *mut crate::System::String,
        minValue: f32,
        maxValue: f32,
        minLimit: f32,
        maxLimit: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (label, minValue, maxValue, minLimit, maxLimit))?;
        Ok(__cordl_object)
    }
    pub fn RegisterEditingCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterEditingCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSliderValueFromClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSliderValueFromClick", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSliderValueFromDrag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSliderValueFromDrag", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetValueWithoutNotify(
        &mut self,
        newValue: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValueWithoutNotify", (newValue))?;
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
    pub fn UnregisterEditingCallbacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterEditingCallbacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDragElementPosition_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDragElementPosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDragElementPosition_GeometryChangedEvent0(
        &mut self,
        evt: *mut crate::UnityEngine::UIElements::GeometryChangedEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDragElementPosition", (evt))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDragThumbsRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDragThumbsRect", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateMixedValueContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMixedValueContent", ())?;
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
    pub fn _ctor_String_f32_f32_f32_f32_1(
        &mut self,
        label: *mut crate::System::String,
        minValue: f32,
        maxValue: f32,
        minLimit: f32,
        maxLimit: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (label, minValue, maxValue, minLimit, maxLimit))?;
        Ok(__cordl_ret)
    }
    pub fn get_clampedDragger(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ClampedDragger_1<f32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ClampedDragger_1<f32> = __cordl_object
            .invoke("get_clampedDragger", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dragElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_dragElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dragMaxThumb(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_dragMaxThumb", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dragMinThumb(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_dragMinThumb", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_highLimit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_highLimit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lowLimit(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lowLimit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_minValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_value", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_clampedDragger(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::ClampedDragger_1<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clampedDragger", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dragElement(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dragElement", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dragMaxThumb(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dragMaxThumb", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dragMinThumb(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dragMinThumb", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_highLimit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_highLimit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lowLimit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lowLimit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_maxValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_minValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_value(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_value", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::MinMaxSlider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+DragState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinMaxSlider_DragState {
    MaxThumb = 3i32,
    MiddleThumb = 2i32,
    MinThumb = 1i32,
    NoThumb = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+DragState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MinMaxSlider_DragState
    => "UnityEngine.UIElements"."MinMaxSlider/DragState"
);
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct MinMaxSlider_UxmlFactory {
    __cordl_parent: crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::MinMaxSlider,
        *mut crate::UnityEngine::UIElements::MinMaxSlider_UxmlTraits,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::MinMaxSlider_UxmlFactory => "UnityEngine.UIElements"
    ."MinMaxSlider/UxmlFactory"
);
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlFactory")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MinMaxSlider_UxmlFactory {
    type Target = crate::UnityEngine::UIElements::UxmlFactory_2<
        *mut crate::UnityEngine::UIElements::MinMaxSlider,
        *mut crate::UnityEngine::UIElements::MinMaxSlider_UxmlTraits,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlFactory")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MinMaxSlider_UxmlFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlFactory")]
impl crate::UnityEngine::UIElements::MinMaxSlider_UxmlFactory {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MinMaxSlider_UxmlFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlTraits")]
#[repr(C)]
#[derive(Debug)]
pub struct MinMaxSlider_UxmlTraits {
    __cordl_parent: crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::Vector2,
    >,
    pub m_MinValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_MaxValue: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_LowLimit: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
    pub m_HighLimit: *mut crate::UnityEngine::UIElements::UxmlFloatAttributeDescription,
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlTraits")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MinMaxSlider_UxmlTraits
    => "UnityEngine.UIElements"."MinMaxSlider/UxmlTraits"
);
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlTraits")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MinMaxSlider_UxmlTraits {
    type Target = crate::UnityEngine::UIElements::BaseField_1_UxmlTraits<
        crate::UnityEngine::Vector2,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlTraits")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MinMaxSlider_UxmlTraits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlTraits")]
impl crate::UnityEngine::UIElements::MinMaxSlider_UxmlTraits {
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "UnityEngine+UIElements+MinMaxSlider+UxmlTraits")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MinMaxSlider_UxmlTraits {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
