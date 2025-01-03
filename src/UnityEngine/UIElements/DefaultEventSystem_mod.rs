#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEventSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Input: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::DefaultEventSystem_IInput,
    >,
    pub m_HorizontalAxis: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_VerticalAxis: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_SubmitButton: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_CancelButton: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_InputActionsPerSecond: f32,
    pub m_RepeatDelay: f32,
    pub m_SendingTouchEvents: bool,
    pub m_SendingPenEvent: bool,
    pub m_Event: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    pub m_FocusedPanel: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseRuntimePanel,
    >,
    pub m_PreviousFocusedPanel: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::BaseRuntimePanel,
    >,
    pub m_PreviousFocusedElement: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::Focusable,
    >,
    pub m_CurrentModifiers: crate::UnityEngine::EventModifiers,
    pub m_LastMousePressButton: i32,
    pub m_NextMousePressTime: f32,
    pub m_LastMouseClickCount: i32,
    pub m_LastMousePosition: crate::UnityEngine::Vector2,
    pub m_MouseProcessedAtLeastOnce: bool,
    pub m_ConsecutiveMoveCount: i32,
    pub m_LastMoveVector: crate::UnityEngine::Vector2,
    pub m_PrevActionTime: f32,
    pub m_IsMoveFromKeyboard: bool,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::DefaultEventSystem =>
    "UnityEngine.UIElements"."DefaultEventSystem"
);
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultEventSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultEventSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem")]
impl crate::UnityEngine::UIElements::DefaultEventSystem {
    #[cfg(
        feature = "UnityEngine+UIElements+DefaultEventSystem+FocusBasedEventSequenceContext"
    )]
    pub type FocusBasedEventSequenceContext = crate::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext;
    #[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+IInput")]
    type IInput = crate::UnityEngine::UIElements::DefaultEventSystem_IInput;
    #[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
    pub type Input = crate::UnityEngine::UIElements::DefaultEventSystem_Input;
    #[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
    pub type NoInput = crate::UnityEngine::UIElements::DefaultEventSystem_NoInput;
    #[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+UpdateMode")]
    pub type UpdateMode = crate::UnityEngine::UIElements::DefaultEventSystem_UpdateMode;
    pub fn FocusBasedEventSequence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext = __cordl_object
            .invoke("FocusBasedEventSequence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DefaultEventSystem_IInput,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DefaultEventSystem_IInput,
        > = __cordl_object.invoke("GetDefaultInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawMoveVector(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("GetRawMoveVector", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MakePenEvent(
        pen: crate::UnityEngine::PenData,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakePenEvent", (pen, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeTouchEvent(
        touch: crate::UnityEngine::Touch,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::EventBase,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeTouchEvent", (touch, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnFocusEvent(
        &mut self,
        panel: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::RuntimePanel>,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::FocusEvent>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFocusEvent", (panel, evt))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMouseEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMouseEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessPenEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessPenEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTabEvent(
        &mut self,
        e: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
        modifiers: crate::UnityEngine::EventModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessTabEvent", (e, modifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessTouchEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessTouchEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendFocusBasedEvent<TArg>(
        &mut self,
        evtFactory: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<TArg, *mut crate::UnityEngine::UIElements::EventBase>,
        >,
        arg: TArg,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TArg: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendFocusBasedEvent", (evtFactory, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendIMGUIEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendIMGUIEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendInputEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendInputEvents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendPositionBasedEvent<TArg>(
        &mut self,
        mousePosition: crate::UnityEngine::Vector3,
        delta: crate::UnityEngine::Vector3,
        pointerId: i32,
        targetDisplay: crate::System::Nullable_1<i32>,
        evtFactory: quest_hook::libil2cpp::Gc<
            crate::System::Func_4<
                crate::UnityEngine::Vector3,
                crate::UnityEngine::Vector3,
                TArg,
                *mut crate::UnityEngine::UIElements::EventBase,
            >,
        >,
        arg: TArg,
        deselectIfNoTarget: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TArg: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendPositionBasedEvent",
                (
                    mousePosition,
                    delta,
                    pointerId,
                    targetDisplay,
                    evtFactory,
                    arg,
                    deselectIfNoTarget,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldIgnoreEventsOnAppNotFocused(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldIgnoreEventsOnAppNotFocused", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSendMoveFromInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldSendMoveFromInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        updateMode: crate::UnityEngine::UIElements::DefaultEventSystem_UpdateMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (updateMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFocusedPanel(
        &mut self,
        runtimePanel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFocusedPanel", (runtimePanel))?;
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
    pub fn get_focusedPanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::BaseRuntimePanel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        > = __cordl_object.invoke("get_focusedPanel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_input(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DefaultEventSystem_IInput,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::DefaultEventSystem_IInput,
        > = __cordl_object.invoke("get_input", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isAppFocused(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isAppFocused", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_focusedPanel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::BaseRuntimePanel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_focusedPanel", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultEventSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+DefaultEventSystem+FocusBasedEventSequenceContext"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DefaultEventSystem_FocusBasedEventSequenceContext {
    pub es: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::DefaultEventSystem,
    >,
}
#[cfg(
    feature = "UnityEngine+UIElements+DefaultEventSystem+FocusBasedEventSequenceContext"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext =>
    "UnityEngine.UIElements"."DefaultEventSystem/FocusBasedEventSequenceContext"
);
#[cfg(
    feature = "UnityEngine+UIElements+DefaultEventSystem+FocusBasedEventSequenceContext"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+DefaultEventSystem+FocusBasedEventSequenceContext"
)]
impl crate::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        es: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::DefaultEventSystem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (es),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+DefaultEventSystem+FocusBasedEventSequenceContext"
)]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+DefaultEventSystem+FocusBasedEventSequenceContext"
)]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::DefaultEventSystem_FocusBasedEventSequenceContext {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+IInput")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEventSystem_IInput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+IInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DefaultEventSystem_IInput => "UnityEngine.UIElements"
    ."DefaultEventSystem/IInput"
);
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+IInput")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+IInput")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+IInput")]
impl crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
    pub fn ClearLastPenContactEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxisRaw(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetAxisRaw", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonDown(
        &mut self,
        button: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetButtonDown", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastPenContactEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PenData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PenData = __cordl_object
            .invoke("GetLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonDown(
        &mut self,
        button: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMouseButtonDown", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonUp(
        &mut self,
        button: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMouseButtonUp", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTouch(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Touch> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Touch = __cordl_object
            .invoke("GetTouch", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_anyKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_anyKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_doubleClickTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_doubleClickTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mouseButtonCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_mouseButtonCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_mousePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePresent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mousePresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_touchCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unscaledTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_unscaledTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+IInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEventSystem_Input {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DefaultEventSystem_Input => "UnityEngine.UIElements"
    ."DefaultEventSystem/Input"
);
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultEventSystem_Input {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultEventSystem_Input {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
impl crate::UnityEngine::UIElements::DefaultEventSystem_Input {
    pub fn ClearLastPenContactEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxisRaw(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetAxisRaw", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonDown(
        &mut self,
        button: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetButtonDown", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastPenContactEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PenData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PenData = __cordl_object
            .invoke("GetLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonDown(
        &mut self,
        button: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMouseButtonDown", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonUp(
        &mut self,
        button: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMouseButtonUp", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTouch(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Touch> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Touch = __cordl_object
            .invoke("GetTouch", (index))?;
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
    pub fn get_anyKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_anyKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_doubleClickTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_doubleClickTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mouseButtonCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_mouseButtonCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_mousePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePresent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mousePresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_touchCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unscaledTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_unscaledTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultEventSystem_Input {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
impl AsRef<crate::UnityEngine::UIElements::DefaultEventSystem_IInput>
for crate::UnityEngine::UIElements::DefaultEventSystem_Input {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+Input")]
impl AsMut<crate::UnityEngine::UIElements::DefaultEventSystem_IInput>
for crate::UnityEngine::UIElements::DefaultEventSystem_Input {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultEventSystem_NoInput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DefaultEventSystem_NoInput => "UnityEngine.UIElements"
    ."DefaultEventSystem/NoInput"
);
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
impl std::ops::Deref for crate::UnityEngine::UIElements::DefaultEventSystem_NoInput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::DefaultEventSystem_NoInput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
impl crate::UnityEngine::UIElements::DefaultEventSystem_NoInput {
    pub fn ClearLastPenContactEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAxisRaw(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetAxisRaw", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonDown(
        &mut self,
        button: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetButtonDown", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastPenContactEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PenData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PenData = __cordl_object
            .invoke("GetLastPenContactEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonDown(
        &mut self,
        button: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMouseButtonDown", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMouseButtonUp(
        &mut self,
        button: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetMouseButtonUp", (button))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTouch(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Touch> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Touch = __cordl_object
            .invoke("GetTouch", (index))?;
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
    pub fn get_anyKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_anyKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_doubleClickTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_doubleClickTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mouseButtonCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_mouseButtonCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_mousePosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mousePresent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_mousePresent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_touchCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_touchCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unscaledTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_unscaledTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::DefaultEventSystem_NoInput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
impl AsRef<crate::UnityEngine::UIElements::DefaultEventSystem_IInput>
for crate::UnityEngine::UIElements::DefaultEventSystem_NoInput {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+NoInput")]
impl AsMut<crate::UnityEngine::UIElements::DefaultEventSystem_IInput>
for crate::UnityEngine::UIElements::DefaultEventSystem_NoInput {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::UIElements::DefaultEventSystem_IInput {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+UpdateMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultEventSystem_UpdateMode {
    Always = 0i32,
    IgnoreIfAppNotFocused = 1i32,
}
#[cfg(feature = "UnityEngine+UIElements+DefaultEventSystem+UpdateMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::DefaultEventSystem_UpdateMode => "UnityEngine.UIElements"
    ."DefaultEventSystem/UpdateMode"
);
