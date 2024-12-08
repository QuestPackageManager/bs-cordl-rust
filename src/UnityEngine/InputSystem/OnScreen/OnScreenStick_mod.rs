#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick+Behaviour")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnScreenStick_Behaviour {
    ExactPositionWithDynamicOrigin = 2i32,
    ExactPositionWithStaticOrigin = 1i32,
    RelativePositionWithStaticOrigin = 0i32,
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick+Behaviour")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::OnScreen::OnScreenStick_Behaviour =>
    "UnityEngine.InputSystem.OnScreen"."OnScreenStick/Behaviour"
);
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
#[repr(C)]
#[derive(Debug)]
pub struct OnScreenStick {
    __cordl_parent: crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
    pub m_MovementRange: f32,
    pub m_DynamicOriginRange: f32,
    pub m_ControlPath: *mut crate::System::String,
    pub m_Behaviour: crate::UnityEngine::InputSystem::OnScreen::OnScreenStick_Behaviour,
    pub m_UseIsolatedInputActions: bool,
    pub m_PointerDownAction: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_PointerMoveAction: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_StartPos: crate::UnityEngine::Vector3,
    pub m_PointerDownPos: crate::UnityEngine::Vector2,
    pub m_RaycastResults: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::EventSystems::RaycastResult,
    >,
    pub m_PointerEventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::OnScreen::OnScreenStick =>
    "UnityEngine.InputSystem.OnScreen"."OnScreenStick"
);
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    type Target = crate::UnityEngine::InputSystem::OnScreen::OnScreenControl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    pub const kDynamicOriginClickable: &'static str = "DynamicOriginClickable";
    #[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick+Behaviour")]
    pub type Behaviour = crate::UnityEngine::InputSystem::OnScreen::OnScreenStick_Behaviour;
    pub fn BeginInteraction(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        uiCamera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginInteraction", (pointerPosition, uiCamera))?;
        Ok(__cordl_ret)
    }
    pub fn DrawGizmoCircle(
        &mut self,
        center: crate::UnityEngine::Vector2,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawGizmoCircle", (center, radius))?;
        Ok(__cordl_ret)
    }
    pub fn EndInteraction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInteraction", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCameraFromCanvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Camera> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Camera = __cordl_object
            .invoke("GetCameraFromCanvas", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveStick(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        uiCamera: *mut crate::UnityEngine::Camera,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveStick", (pointerPosition, uiCamera))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDrag(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrag", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnDrawGizmosSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrawGizmosSelected", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDown_InputAction_CallbackContext1(
        &mut self,
        ctx: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerDown_PointerEventData0(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerMove(
        &mut self,
        ctx: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerMove", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUp_InputAction_CallbackContext1(
        &mut self,
        ctx: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerUp_PointerEventData0(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateDynamicOriginClickableArea(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDynamicOriginClickableArea", ())?;
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
    pub fn get_behaviour(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::OnScreen::OnScreenStick_Behaviour,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::OnScreen::OnScreenStick_Behaviour = __cordl_object
            .invoke("get_behaviour", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_controlPathInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_controlPathInternal", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dynamicOriginRange(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_dynamicOriginRange", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_movementRange(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_movementRange", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useIsolatedInputActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useIsolatedInputActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_behaviour(
        &mut self,
        value: crate::UnityEngine::InputSystem::OnScreen::OnScreenStick_Behaviour,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_behaviour", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_controlPathInternal(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controlPathInternal", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dynamicOriginRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dynamicOriginRange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_movementRange(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_movementRange", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useIsolatedInputActions(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useIsolatedInputActions", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
