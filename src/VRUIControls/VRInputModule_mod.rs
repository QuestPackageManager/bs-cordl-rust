#[cfg(feature = "VRUIControls+VRInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct VRInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseInputModule,
    pub _vrPointer: quest_hook::libil2cpp::Gc<crate::VRUIControls::VRPointer>,
    pub _rumblePreset: quest_hook::libil2cpp::Gc<
        crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    >,
    pub _hapticFeedbackManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HapticFeedbackManager,
    >,
    pub _useMouseForPressInput_k__BackingField: bool,
    pub onProcessMousePressEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::UnityEngine::GameObject>,
    >,
    pub pointerDidClickEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::UnityEngine::EventSystems::PointerEventData>,
    >,
    pub _pointerData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::UnityEngine::EventSystems::PointerEventData,
        >,
    >,
    pub _componentList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::UnityEngine::Component>,
    >,
    pub _mouseState: quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseState>,
}
#[cfg(feature = "VRUIControls+VRInputModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::VRInputModule => "VRUIControls"
    ."VRInputModule"
);
#[cfg(feature = "VRUIControls+VRInputModule")]
impl std::ops::Deref for crate::VRUIControls::VRInputModule {
    type Target = crate::UnityEngine::EventSystems::BaseInputModule;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl std::ops::DerefMut for crate::VRUIControls::VRInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl crate::VRUIControls::VRInputModule {
    pub const kMinPressValue: f32 = 0.9f32;
    pub const kMouseLeftId: i32 = -1i32;
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
    pub fn GetMousePointerEventData(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseState>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseState> = __cordl_object
            .invoke("GetMousePointerEventData", (id))?;
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
    pub fn HandlePointerExitAndEnter(
        &mut self,
        currentPointerData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
        newEnterTarget: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePointerExitAndEnter", (currentPointerData, newEnterTarget))?;
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
    pub fn Process(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", ())?;
        Ok(__cordl_ret.into())
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
    pub fn ProcessMousePress(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::VRUIControls::MouseButtonEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMousePress", (data))?;
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
    pub fn RaycastComparer(
        lhs: crate::UnityEngine::EventSystems::RaycastResult,
        rhs: crate::UnityEngine::EventSystems::RaycastResult,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RaycastComparer", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendUpdateEventToSelectedObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SendUpdateEventToSelectedObject", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldStartDrag(
        &mut self,
        pressPos: crate::UnityEngine::Vector2,
        currentPos: crate::UnityEngine::Vector2,
        threshold: f32,
        useDragThreshold: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ShouldStartDrag",
                (pressPos, currentPos, threshold, useDragThreshold),
            )?;
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
    pub fn add_onProcessMousePressEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onProcessMousePressEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_pointerDidClickEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::EventSystems::PointerEventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_pointerDidClickEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useMouseForPressInput(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useMouseForPressInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vrPointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::VRUIControls::VRPointer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::VRUIControls::VRPointer> = __cordl_object
            .invoke("get_vrPointer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_onProcessMousePressEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::UnityEngine::GameObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onProcessMousePressEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_pointerDidClickEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::UnityEngine::EventSystems::PointerEventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_pointerDidClickEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useMouseForPressInput(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useMouseForPressInput", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::VRInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl AsRef<crate::GlobalNamespace::IVRInputModule>
for crate::VRUIControls::VRInputModule {
    fn as_ref(&self) -> &crate::GlobalNamespace::IVRInputModule {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "VRUIControls+VRInputModule")]
impl AsMut<crate::GlobalNamespace::IVRInputModule>
for crate::VRUIControls::VRInputModule {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IVRInputModule {
        unsafe { std::mem::transmute(self) }
    }
}
