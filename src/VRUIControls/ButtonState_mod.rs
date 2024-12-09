#[cfg(feature = "VRUIControls+ButtonState")]
#[repr(C)]
#[derive(Debug)]
pub struct ButtonState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _button: crate::UnityEngine::EventSystems::PointerEventData_InputButton,
    pub _eventData: *mut crate::VRUIControls::MouseButtonEventData,
    pub _pressedValue: f32,
}
#[cfg(feature = "VRUIControls+ButtonState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::ButtonState => "VRUIControls"
    ."ButtonState"
);
#[cfg(feature = "VRUIControls+ButtonState")]
impl std::ops::Deref for crate::VRUIControls::ButtonState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+ButtonState")]
impl std::ops::DerefMut for crate::VRUIControls::ButtonState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+ButtonState")]
impl crate::VRUIControls::ButtonState {
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
    pub fn get_button(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::EventSystems::PointerEventData_InputButton,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventSystems::PointerEventData_InputButton = __cordl_object
            .invoke("get_button", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_eventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::VRUIControls::MouseButtonEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::VRUIControls::MouseButtonEventData = __cordl_object
            .invoke("get_eventData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pressedValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_pressedValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_button(
        &mut self,
        value: crate::UnityEngine::EventSystems::PointerEventData_InputButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_button", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_eventData(
        &mut self,
        value: *mut crate::VRUIControls::MouseButtonEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pressedValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pressedValue", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "VRUIControls+ButtonState")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::ButtonState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
