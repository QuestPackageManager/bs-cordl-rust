#[cfg(
    feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+CursorLockBehavior"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSystemUIInputModule_CursorLockBehavior {
    OutsideScreen = 0i32,
    ScreenCenter = 1i32,
}
#[cfg(
    feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+CursorLockBehavior"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::InputSystemUIInputModule_CursorLockBehavior =>
    "UnityEngine.InputSystem.UI"."InputSystemUIInputModule/CursorLockBehavior"
);
#[cfg(
    feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+InputActionReferenceState"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputSystemUIInputModule_InputActionReferenceState {
    pub refCount: i32,
    pub enabledByInputModule: bool,
}
#[cfg(
    feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+InputActionReferenceState"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::InputSystemUIInputModule_InputActionReferenceState =>
    "UnityEngine.InputSystem.UI"."InputSystemUIInputModule/InputActionReferenceState"
);
#[cfg(
    feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+InputActionReferenceState"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_InputActionReferenceState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+InputActionReferenceState"
)]
impl crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_InputActionReferenceState {}
#[cfg(feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct InputSystemUIInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseInputModule,
    pub m_MoveRepeatDelay: f32,
    pub m_MoveRepeatRate: f32,
    pub m_TrackedDeviceDragThresholdMultiplier: f32,
    pub m_XRTrackingOrigin: *mut crate::UnityEngine::Transform,
    pub m_ActionsAsset: *mut crate::UnityEngine::InputSystem::InputActionAsset,
    pub m_PointAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_MoveAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_SubmitAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_CancelAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_LeftClickAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_MiddleClickAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_RightClickAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_ScrollWheelAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_TrackedDevicePositionAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_TrackedDeviceOrientationAction: *mut crate::UnityEngine::InputSystem::InputActionReference,
    pub m_DeselectOnBackgroundClick: bool,
    pub m_PointerBehavior: crate::UnityEngine::InputSystem::UI::UIPointerBehavior,
    pub m_CursorLockBehavior: crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_CursorLockBehavior,
    pub m_ActionsHooked: bool,
    pub m_NeedToPurgeStalePointers: bool,
    pub m_OnPointDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnMoveDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnLeftClickDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnRightClickDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnMiddleClickDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnScrollWheelDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnTrackedDevicePositionDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnTrackedDeviceOrientationDelegate: *mut crate::System::Action_1<
        crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    >,
    pub m_OnControlsChangedDelegate: *mut crate::System::Action_1<
        *mut crate::System::Object,
    >,
    pub m_CurrentPointerId: i32,
    pub m_CurrentPointerIndex: i32,
    pub m_CurrentPointerType: crate::UnityEngine::InputSystem::UI::UIPointerType,
    pub m_PointerIds: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<i32>,
    pub m_PointerTouchControls: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        *mut crate::UnityEngine::InputSystem::InputControl,
    >,
    pub m_PointerStates: crate::UnityEngine::InputSystem::Utilities::InlinedArray_1<
        crate::UnityEngine::InputSystem::UI::PointerModel,
    >,
    pub m_NavigationState: crate::UnityEngine::InputSystem::UI::NavigationModel,
    pub m_LocalMultiPlayerRoot: *mut crate::UnityEngine::GameObject,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::InputSystemUIInputModule =>
    "UnityEngine.InputSystem.UI"."InputSystemUIInputModule"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule {
    type Target = crate::UnityEngine::EventSystems::BaseInputModule;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule")]
impl crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule {
    pub const kClickSpeed: f32 = 0.3f32;
    pub const kPixelPerLine: f32 = 20f32;
    #[cfg(
        feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+CursorLockBehavior"
    )]
    pub type CursorLockBehavior = crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_CursorLockBehavior;
    #[cfg(
        feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule+InputActionReferenceState"
    )]
    pub type InputActionReferenceState = crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_InputActionReferenceState;
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
    pub fn AllocatePointer(
        &mut self,
        pointerId: i32,
        displayIndex: i32,
        touchId: i32,
        pointerType: crate::UnityEngine::InputSystem::UI::UIPointerType,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        touchControl: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "AllocatePointer",
                (
                    pointerId,
                    displayIndex,
                    touchId,
                    pointerType,
                    control,
                    device,
                    touchControl,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AssignDefaultActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AssignDefaultActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckForRemovedDevice(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckForRemovedDevice", (context))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertUIToolkitPointerId(
        &mut self,
        sourcePointerData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ConvertUIToolkitPointerId", (sourcePointerData))?;
        Ok(__cordl_ret)
    }
    pub fn DisableAllActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableAllActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableAllActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableAllActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnableInputAction(
        &mut self,
        inputActionReference: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableInputAction", (inputActionReference))?;
        Ok(__cordl_ret)
    }
    pub fn FilterPointerStatesByType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FilterPointerStatesByType", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDisplayIndexFor(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDisplayIndexFor", (control))?;
        Ok(__cordl_ret)
    }
    pub fn GetLastRaycastResult(
        &mut self,
        pointerOrTouchId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventSystems::RaycastResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventSystems::RaycastResult = __cordl_object
            .invoke("GetLastRaycastResult", (pointerOrTouchId))?;
        Ok(__cordl_ret)
    }
    pub fn GetPointerStateForIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::UI::PointerModel,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::UI::PointerModel,
        > = __cordl_object.invoke("GetPointerStateForIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn GetPointerStateIndexFor_ByRefMut1(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPointerStateIndexFor", (context))?;
        Ok(__cordl_ret)
    }
    pub fn GetPointerStateIndexFor_InputControl__cordl_bool2(
        &mut self,
        control: *mut crate::UnityEngine::InputSystem::InputControl,
        createIfNotExists: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPointerStateIndexFor", (control, createIfNotExists))?;
        Ok(__cordl_ret)
    }
    pub fn GetPointerStateIndexFor_i32_0(
        &mut self,
        pointerOrTouchId: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetPointerStateIndexFor", (pointerOrTouchId))?;
        Ok(__cordl_ret)
    }
    pub fn HasNoActions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasNoActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn HookActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HookActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn IgnoreNextClick(
        &mut self,
        context: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
        wasPressed: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IgnoreNextClick", (context, wasPressed))?;
        Ok(__cordl_ret)
    }
    pub fn IsMoveAllowed(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::AxisEventData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMoveAllowed", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn IsPointerOverGameObject(
        &mut self,
        pointerOrTouchId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPointerOverGameObject", (pointerOrTouchId))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnControlsChanged(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnControlsChanged", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnLeftClickCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLeftClickCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnMiddleClickCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMiddleClickCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnMoveCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMoveCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnRightClickCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRightClickCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnScrollCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScrollCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnTrackedDeviceOrientationCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTrackedDeviceOrientationCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn OnTrackedDevicePositionCallback(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTrackedDevicePositionCallback", (context))?;
        Ok(__cordl_ret)
    }
    pub fn PerformRaycast(
        &mut self,
        eventData: *mut crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::EventSystems::RaycastResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventSystems::RaycastResult = __cordl_object
            .invoke("PerformRaycast", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn PointerShouldIgnoreTransform(
        &mut self,
        t: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PointerShouldIgnoreTransform", (t))?;
        Ok(__cordl_ret)
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
    pub fn ProcessNavigation(
        &mut self,
        navigationState: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::UI::NavigationModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessNavigation", (navigationState))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointer(
        &mut self,
        state: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::UI::PointerModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointer", (state))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointerButton(
        &mut self,
        button: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState,
        >,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerButton", (button, eventData))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointerButtonDrag(
        &mut self,
        button: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::UI::PointerModel_ButtonState,
        >,
        eventData: *mut crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerButtonDrag", (button, eventData))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointerMovement_ByRefMut_ExtendedPointerEventData0(
        &mut self,
        pointer: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::InputSystem::UI::PointerModel,
        >,
        eventData: *mut crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerMovement", (pointer, eventData))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessPointerMovement_ExtendedPointerEventData_GameObject1(
        &mut self,
        eventData: *mut crate::UnityEngine::InputSystem::UI::ExtendedPointerEventData,
        currentPointerTarget: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessPointerMovement", (eventData, currentPointerTarget))?;
        Ok(__cordl_ret)
    }
    pub fn PurgeStalePointers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PurgeStalePointers", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemovePointerAtIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePointerAtIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn ResetPointers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetPointers", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendPointerExitEventsAndRemovePointer(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendPointerExitEventsAndRemovePointer", (index))?;
        Ok(__cordl_ret)
    }
    pub fn SetActionCallbacks(
        &mut self,
        install: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActionCallbacks", (install))?;
        Ok(__cordl_ret)
    }
    pub fn SwapAction(
        &mut self,
        property: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::InputSystem::InputActionReference,
        >,
        newValue: *mut crate::UnityEngine::InputSystem::InputActionReference,
        actionsHooked: bool,
        actionCallback: *mut crate::System::Action_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SwapAction", (property, newValue, actionsHooked, actionCallback))?;
        Ok(__cordl_ret)
    }
    pub fn TryDisableInputAction(
        &mut self,
        inputActionReference: *mut crate::UnityEngine::InputSystem::InputActionReference,
        isComponentDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TryDisableInputAction",
                (inputActionReference, isComponentDisabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnassignActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnassignActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnhookActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnhookActions", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateReferenceForNewAsset(
        &mut self,
        actionReference: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("UpdateReferenceForNewAsset", (actionReference))?;
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
    pub fn get_actionsAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionAsset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionAsset = __cordl_object
            .invoke("get_actionsAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cancel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_cancel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cursorLockBehavior(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_CursorLockBehavior,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_CursorLockBehavior = __cordl_object
            .invoke("get_cursorLockBehavior", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_deselectOnBackgroundClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_deselectOnBackgroundClick", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_explictlyIgnoreFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_explictlyIgnoreFocus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leftClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_leftClick", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localMultiPlayerRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_localMultiPlayerRoot", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_middleClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_middleClick", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_move(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_move", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_moveRepeatDelay(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_moveRepeatDelay", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_moveRepeatRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_moveRepeatRate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_point(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_point", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pointerBehavior(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::UI::UIPointerBehavior,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::UI::UIPointerBehavior = __cordl_object
            .invoke("get_pointerBehavior", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_repeatDelay(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_repeatDelay", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_repeatRate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_repeatRate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rightClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_rightClick", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scrollWheel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_scrollWheel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_shouldIgnoreFocus(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_shouldIgnoreFocus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_submit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_submit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackedDeviceDragThresholdMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_trackedDeviceDragThresholdMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackedDeviceOrientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_trackedDeviceOrientation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackedDevicePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_trackedDevicePosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_trackedDeviceSelect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = __cordl_object
            .invoke("get_trackedDeviceSelect", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_xrTrackingOrigin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Transform = __cordl_object
            .invoke("get_xrTrackingOrigin", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_actionsAsset(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_actionsAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_cancel(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cancel", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_cursorLockBehavior(
        &mut self,
        value: crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule_CursorLockBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cursorLockBehavior", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_deselectOnBackgroundClick(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_deselectOnBackgroundClick", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_leftClick(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftClick", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_localMultiPlayerRoot(
        &mut self,
        value: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_localMultiPlayerRoot", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_middleClick(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_middleClick", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_move(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_move", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_moveRepeatDelay(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_moveRepeatDelay", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_moveRepeatRate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_moveRepeatRate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_point(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_point", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pointerBehavior(
        &mut self,
        value: crate::UnityEngine::InputSystem::UI::UIPointerBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pointerBehavior", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_repeatDelay(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_repeatDelay", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_repeatRate(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_repeatRate", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_rightClick(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightClick", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_scrollWheel(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scrollWheel", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_submit(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_submit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackedDeviceDragThresholdMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackedDeviceDragThresholdMultiplier", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackedDeviceOrientation(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackedDeviceOrientation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackedDevicePosition(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackedDevicePosition", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_trackedDeviceSelect(
        &mut self,
        value: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trackedDeviceSelect", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_xrTrackingOrigin(
        &mut self,
        value: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_xrTrackingOrigin", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+InputSystemUIInputModule")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::UI::InputSystemUIInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
