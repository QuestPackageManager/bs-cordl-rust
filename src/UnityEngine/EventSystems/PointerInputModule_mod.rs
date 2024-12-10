#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseInputModule,
    pub m_PointerData: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::EventSystems::PointerEventData,
    >,
    pub m_MouseState: *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::PointerInputModule =>
    "UnityEngine.EventSystems"."PointerInputModule"
);
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::PointerInputModule {
    type Target = crate::UnityEngine::EventSystems::BaseInputModule;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::PointerInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule")]
impl crate::UnityEngine::EventSystems::PointerInputModule {
    pub const kFakeTouchesId: i32 = -4i32;
    pub const kMouseLeftId: i32 = -1i32;
    pub const kMouseMiddleId: i32 = -3i32;
    pub const kMouseRightId: i32 = -2i32;
    #[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+ButtonState")]
    pub type ButtonState = crate::UnityEngine::EventSystems::PointerInputModule_ButtonState;
    #[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseButtonEventData")]
    pub type MouseButtonEventData = crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData;
    #[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseState")]
    pub type MouseState = crate::UnityEngine::EventSystems::PointerInputModule_MouseState;
    pub fn ClearSelection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromTo(
        &mut self,
        from: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        to: quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromTo", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeselectIfSelectionChanged(
        &mut self,
        currentOverGo: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeselectIfSelectionChanged", (currentOverGo, pointerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastPointerEventData(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        > = __cordl_object.invoke("GetLastPointerEventData", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMousePointerEventData_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
        > = __cordl_object.invoke("GetMousePointerEventData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMousePointerEventData_i32_1(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_MouseState,
        > = __cordl_object.invoke("GetMousePointerEventData", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPointerData(
        &mut self,
        id: i32,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::EventSystems::PointerEventData,
        >,
        create: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetPointerData", (id, data, create))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTouchPointerEventData(
        &mut self,
        input: crate::UnityEngine::Touch,
        pressed: quest_hook::libil2cpp::ByRefMut<bool>,
        released: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::EventSystems::PointerEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        > = __cordl_object
            .invoke("GetTouchPointerEventData", (input, pressed, released))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPointerOverGameObject(
        &mut self,
        pointerId: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPointerOverGameObject", (pointerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessDrag(
        &mut self,
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessDrag", (pointerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessMove(
        &mut self,
        pointerEvent: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMove", (pointerEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemovePointerData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemovePointerData", (data))?;
        Ok(__cordl_ret.into())
    }
    pub fn StateForMouseButton(
        &mut self,
        buttonId: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::EventSystems::PointerEventData_FramePressState = __cordl_object
            .invoke("StateForMouseButton", (buttonId))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
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
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PointerInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+ButtonState")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerInputModule_ButtonState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Button: crate::UnityEngine::EventSystems::PointerEventData_InputButton,
    pub m_EventData: *mut crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData,
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+ButtonState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::PointerInputModule_ButtonState =>
    "UnityEngine.EventSystems"."PointerInputModule/ButtonState"
);
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+ButtonState")]
impl std::ops::Deref
for crate::UnityEngine::EventSystems::PointerInputModule_ButtonState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+ButtonState")]
impl std::ops::DerefMut
for crate::UnityEngine::EventSystems::PointerInputModule_ButtonState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+ButtonState")]
impl crate::UnityEngine::EventSystems::PointerInputModule_ButtonState {
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
        Ok(__cordl_ret.into())
    }
    pub fn get_eventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData,
        > = __cordl_object.invoke("get_eventData", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_eventData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_eventData", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+ButtonState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PointerInputModule_ButtonState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseButtonEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerInputModule_MouseButtonEventData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub buttonState: crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
    pub buttonData: *mut crate::UnityEngine::EventSystems::PointerEventData,
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseButtonEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData =>
    "UnityEngine.EventSystems"."PointerInputModule/MouseButtonEventData"
);
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseButtonEventData")]
impl std::ops::Deref
for crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseButtonEventData")]
impl std::ops::DerefMut
for crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseButtonEventData")]
impl crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PressedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PressedThisFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleasedThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReleasedThisFrame", ())?;
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
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseButtonEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PointerInputModule_MouseButtonEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseState")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerInputModule_MouseState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_TrackedButtons: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::EventSystems::PointerInputModule_ButtonState,
    >,
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::EventSystems::PointerInputModule_MouseState =>
    "UnityEngine.EventSystems"."PointerInputModule/MouseState"
);
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseState")]
impl std::ops::Deref
for crate::UnityEngine::EventSystems::PointerInputModule_MouseState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseState")]
impl std::ops::DerefMut
for crate::UnityEngine::EventSystems::PointerInputModule_MouseState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseState")]
impl crate::UnityEngine::EventSystems::PointerInputModule_MouseState {
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
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_ButtonState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerInputModule_ButtonState,
        > = __cordl_object.invoke("GetButtonState", (button))?;
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
#[cfg(feature = "UnityEngine+EventSystems+PointerInputModule+MouseState")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::PointerInputModule_MouseState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
