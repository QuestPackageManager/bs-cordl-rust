#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput")]
#[repr(C)]
#[derive(Debug)]
pub struct VirtualMouseInput {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub m_CursorMode: crate::UnityEngine::InputSystem::UI::VirtualMouseInput_CursorMode,
    pub m_CursorGraphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    pub m_CursorTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_CursorSpeed: f32,
    pub m_ScrollSpeed: f32,
    pub m_StickAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_LeftButtonAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_MiddleButtonAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_RightButtonAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_ForwardButtonAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_BackButtonAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_ScrollWheelAction: crate::UnityEngine::InputSystem::InputActionProperty,
    pub m_Canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    pub m_VirtualMouse: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::Mouse,
    >,
    pub m_SystemMouse: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Mouse>,
    pub m_AfterInputUpdateDelegate: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub m_ButtonActionTriggeredDelegate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_LastTime: f64,
    pub m_LastStickValue: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::UI::VirtualMouseInput
    => "UnityEngine.InputSystem.UI"."VirtualMouseInput"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::UI::VirtualMouseInput {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::UI::VirtualMouseInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput")]
impl crate::UnityEngine::InputSystem::UI::VirtualMouseInput {
    #[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput+CursorMode")]
    pub type CursorMode = crate::UnityEngine::InputSystem::UI::VirtualMouseInput_CursorMode;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnAfterInputUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAfterInputUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnButtonActionTriggered(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnButtonActionTriggered", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAction(
        field: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputActionProperty,
        >,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAction", (field, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetActionCallback(
        field: crate::UnityEngine::InputSystem::InputActionProperty,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
        install: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetActionCallback", (field, callback, install))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryEnableHardwareCursor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryEnableHardwareCursor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindCanvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFindCanvas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateMotion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateMotion", ())?;
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
    pub fn get_backButtonAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_backButtonAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursorGraphic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic> = __cordl_object
            .invoke("get_cursorGraphic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursorMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::UI::VirtualMouseInput_CursorMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::UI::VirtualMouseInput_CursorMode = __cordl_object
            .invoke("get_cursorMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursorSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cursorSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursorTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform> = __cordl_object
            .invoke("get_cursorTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_forwardButtonAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_forwardButtonAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftButtonAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_leftButtonAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_middleButtonAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_middleButtonAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightButtonAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_rightButtonAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scrollSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scrollWheelAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_scrollWheelAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stickAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputActionProperty,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputActionProperty = __cordl_object
            .invoke("get_stickAction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_virtualMouse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Mouse>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Mouse,
        > = __cordl_object.invoke("get_virtualMouse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_backButtonAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_backButtonAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cursorGraphic(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cursorGraphic", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cursorMode(
        &mut self,
        value: crate::UnityEngine::InputSystem::UI::VirtualMouseInput_CursorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cursorMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cursorSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cursorSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cursorTransform(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cursorTransform", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_forwardButtonAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_forwardButtonAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leftButtonAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftButtonAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_middleButtonAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_middleButtonAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rightButtonAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightButtonAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scrollSpeed(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollSpeed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scrollWheelAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollWheelAction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_stickAction(
        &mut self,
        value: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_stickAction", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::UI::VirtualMouseInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput+CursorMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VirtualMouseInput_CursorMode {
    #[default]
    HardwareCursorIfAvailable = 1i32,
    SoftwareCursor = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+VirtualMouseInput+CursorMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::VirtualMouseInput_CursorMode =>
    "UnityEngine.InputSystem.UI"."VirtualMouseInput/CursorMode"
);
