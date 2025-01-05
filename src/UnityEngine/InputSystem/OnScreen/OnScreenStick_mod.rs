#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
#[repr(C)]
#[derive(Debug)]
pub struct OnScreenStick {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
    >,
    pub m_MovementRange: f32,
    pub m_DynamicOriginRange: f32,
    pub m_ControlPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Behaviour: crate::UnityEngine::InputSystem::OnScreen::OnScreenStick_Behaviour,
    pub m_UseIsolatedInputActions: bool,
    pub m_PointerDownAction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_PointerMoveAction: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_StartPos: crate::UnityEngine::Vector3,
    pub m_PointerDownPos: crate::UnityEngine::Vector2,
    pub m_RaycastResults: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::RaycastResult,
    >,
    pub m_PointerEventData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::PointerEventData,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::OnScreen::OnScreenStick =>
    "UnityEngine.InputSystem.OnScreen"."OnScreenStick"
);
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    type Target = quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::OnScreen::OnScreenControl,
    >;
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
        uiCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginInteraction", (pointerPosition, uiCamera))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn EndInteraction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInteraction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraFromCanvas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = __cordl_object
            .invoke("GetCameraFromCanvas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveStick(
        &mut self,
        pointerPosition: crate::UnityEngine::Vector2,
        uiCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveStick", (pointerPosition, uiCamera))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDrag(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrag", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDrawGizmosSelected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDrawGizmosSelected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerDown_Gc0(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerDown", (eventData))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerUp_Gc0(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerUp", (eventData))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateDynamicOriginClickableArea(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDynamicOriginClickableArea", ())?;
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
        Ok(__cordl_ret.into())
    }
    pub fn get_controlPathInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_controlPathInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_dynamicOriginRange(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_dynamicOriginRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_movementRange(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_movementRange", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useIsolatedInputActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useIsolatedInputActions", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_controlPathInternal(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controlPathInternal", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler>>
for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IDragHandler> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEventSystemHandler>,
> for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IEventSystemHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IEventSystemHandler>,
> for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IEventSystemHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerDownHandler>,
> for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerDownHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerDownHandler>,
> for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerDownHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerUpHandler>,
> for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerUpHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::IPointerUpHandler>,
> for crate::UnityEngine::InputSystem::OnScreen::OnScreenStick {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::IPointerUpHandler,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+OnScreen+OnScreenStick+Behaviour")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnScreenStick_Behaviour {
    #[default]
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
