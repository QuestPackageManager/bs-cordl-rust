#[cfg(feature = "VRUIControls+MouseState")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _trackedButtons: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::VRUIControls::ButtonState,
        >,
    >,
}
#[cfg(feature = "VRUIControls+MouseState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::MouseState => "VRUIControls"
    ."MouseState"
);
#[cfg(feature = "VRUIControls+MouseState")]
impl std::ops::Deref for crate::VRUIControls::MouseState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+MouseState")]
impl std::ops::DerefMut for crate::VRUIControls::MouseState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+MouseState")]
impl crate::VRUIControls::MouseState {
    pub fn AnyPressesThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AnyPressesThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AnyReleasesThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AnyReleasesThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonState(
        &mut self,
        button: crate::UnityEngine::EventSystems::PointerEventData_InputButton,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::VRUIControls::ButtonState>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::VRUIControls::ButtonState> = __cordl_object
            .invoke("GetButtonState", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetButtonState(
        &mut self,
        button: crate::UnityEngine::EventSystems::PointerEventData_InputButton,
        stateForMouseButton: crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetButtonState", (button, stateForMouseButton, data))?;
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
#[cfg(feature = "VRUIControls+MouseState")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::MouseState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
