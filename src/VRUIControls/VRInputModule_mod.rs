#[cfg(feature = "VRUIControls+VRInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct VRInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::BaseInputModule,
    pub _vrPointer: *mut crate::VRUIControls::VRPointer,
    pub _rumblePreset: *mut crate::Libraries::HM::HMLib::VR::HapticPresetSO,
    pub _hapticFeedbackManager: *mut crate::GlobalNamespace::HapticFeedbackManager,
    pub onProcessMousePressEvent: *mut crate::System::Action_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _pointerData: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::EventSystems::PointerEventData,
    >,
    pub _componentList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::Component,
    >,
    pub _mouseState: *mut crate::VRUIControls::MouseState,
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
        Ok(__cordl_ret)
    }
    pub fn DeselectIfSelectionChanged(
        &mut self,
        currentOverGo: *mut crate::UnityEngine::GameObject,
        pointerEvent: *mut crate::UnityEngine::EventSystems::BaseEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeselectIfSelectionChanged", (currentOverGo, pointerEvent))?;
        Ok(__cordl_ret)
    }
    pub fn GetLastPointerEventData(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::PointerEventData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::PointerEventData = __cordl_object
            .invoke("GetLastPointerEventData", (id))?;
        Ok(__cordl_ret)
    }
    pub fn GetMousePointerEventData(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::VRUIControls::MouseState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::VRUIControls::MouseState = __cordl_object
            .invoke("GetMousePointerEventData", (id))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn HandlePointerExitAndEnter(
        &mut self,
        currentPointerData: *mut crate::UnityEngine::EventSystems::PointerEventData,
        newEnterTarget: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePointerExitAndEnter", (currentPointerData, newEnterTarget))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn ProcessMousePress(
        &mut self,
        data: *mut crate::VRUIControls::MouseButtonEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMousePress", (data))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMove(
        &mut self,
        pointerEvent: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMove", (pointerEvent))?;
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
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn add_onProcessMousePressEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_onProcessMousePressEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_onProcessMousePressEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_onProcessMousePressEvent", (value))?;
        Ok(__cordl_ret)
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
