#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule+InputMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRInputModule_InputMode {
    Buttons = 1i32,
    Mouse = 0i32,
}
#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule+InputMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::OVRInputModule_InputMode => "UnityEngine.EventSystems"
    ."OVRInputModule/InputMode"
);
#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::PointerInputModule,
    pub rayTransform: *mut crate::UnityEngine::Transform,
    pub m_Cursor: *mut OVRCursor,
    pub joyPadClickButton: crate::GlobalNamespace::OVRInput_Button,
    pub gazeClickKey: crate::UnityEngine::KeyCode,
    pub performSphereCastForGazepointer: bool,
    pub useRightStickScroll: bool,
    pub rightStickDeadZone: f32,
    pub useSwipeScroll: bool,
    pub swipeDragThreshold: f32,
    pub swipeDragScale: f32,
    pub InvertSwipeXAxis: bool,
    pub activeGraphicRaycaster: *mut OVRRaycaster,
    pub angleDragThreshold: f32,
    pub m_SpherecastRadius: f32,
    pub m_NextAction: f32,
    pub m_LastMousePosition: crate::UnityEngine::Vector2,
    pub m_MousePosition: crate::UnityEngine::Vector2,
    pub m_HorizontalAxis: *mut crate::System::String,
    pub m_VerticalAxis: *mut crate::System::String,
    pub m_SubmitButton: *mut crate::System::String,
    pub m_CancelButton: *mut crate::System::String,
    pub m_InputActionsPerSecond: f32,
    pub m_AllowActivationOnMobileDevice: bool,
    pub m_VRRayPointerData: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::EventSystems::OVRPointerEventData,
    >,
    pub m_MouseState: *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
}
#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::OVRInputModule =>
    "UnityEngine.EventSystems"."OVRInputModule"
);
#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::OVRInputModule {
    type Target = crate::UnityEngine::EventSystems::PointerInputModule;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::OVRInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule")]
impl crate::UnityEngine::EventSystems::OVRInputModule {
    #[cfg(feature = "UnityEngine+EventSystems+OVRInputModule+InputMode")]
    pub type InputMode = crate::UnityEngine::EventSystems::OVRInputModule_InputMode;
    pub fn ActivateModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ActivateModule", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllowMoveEventProcessing(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AllowMoveEventProcessing", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn ClearSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelection", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyFromTo_OVRPointerEventData_OVRPointerEventData0(
        &mut self,
        from: *mut crate::UnityEngine::EventSystems::OVRPointerEventData,
        to: *mut crate::UnityEngine::EventSystems::OVRPointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromTo", (from, to))?;
        Ok(__cordl_ret)
    }
    pub fn CopyFromTo_PointerEventData_PointerEventData1(
        &mut self,
        from: *mut crate::UnityEngine::EventSystems::PointerEventData,
        to: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromTo", (from, to))?;
        Ok(__cordl_ret)
    }
    pub fn DeactivateModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeactivateModule", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCanvasPointerData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseState = __cordl_object
            .invoke("GetCanvasPointerData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetExtraScrollDelta(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetExtraScrollDelta", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGazeButtonState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventSystems::PointerEventData_FramePressState = __cordl_object
            .invoke("GetGazeButtonState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGazePointerData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseState = __cordl_object
            .invoke("GetGazePointerData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPointerData(
        &mut self,
        id: i32,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::EventSystems::OVRPointerEventData,
        >,
        create: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPointerData", (id, data, create))?;
        Ok(__cordl_ret)
    }
    pub fn GetRawMoveVector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetRawMoveVector", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsModuleSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsModuleSupported", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Process(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessDrag(
        &mut self,
        pointerEvent: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDrag", (pointerEvent))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMouseEvent(
        &mut self,
        mouseData: *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMouseEvent", (mouseData))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMousePress(
        &mut self,
        data: *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMousePress", (data))?;
        Ok(__cordl_ret)
    }
    pub fn SendMoveEventToSelectedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendMoveEventToSelectedObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendSubmitEventToSelectedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendSubmitEventToSelectedObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendUpdateEventToSelectedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendUpdateEventToSelectedObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShouldActivateModule(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldActivateModule", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShouldStartDrag(
        &mut self,
        pointerEvent: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldStartDrag", (pointerEvent))?;
        Ok(__cordl_ret)
    }
    pub fn SwipeAdjustedPosition(
        &mut self,
        originalPosition: crate::UnityEngine::Vector2,
        pointerEvent: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("SwipeAdjustedPosition", (originalPosition, pointerEvent))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateModule(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateModule", ())?;
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
    pub fn get_allowActivationOnMobileDevice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_allowActivationOnMobileDevice", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cancelButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_cancelButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_horizontalAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_horizontalAxis", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_inputActionsPerSecond(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_inputActionsPerSecond", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_inputMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::EventSystems::OVRInputModule_InputMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventSystems::OVRInputModule_InputMode = __cordl_object
            .invoke("get_inputMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_submitButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_submitButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_verticalAxis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_verticalAxis", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_allowActivationOnMobileDevice(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_allowActivationOnMobileDevice", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_cancelButton(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cancelButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_horizontalAxis(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_horizontalAxis", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_inputActionsPerSecond(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inputActionsPerSecond", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_submitButton(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_submitButton", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_verticalAxis(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_verticalAxis", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+OVRInputModule")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::OVRInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
