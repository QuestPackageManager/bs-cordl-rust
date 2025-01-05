#[cfg(feature = "VRUIControls+VRPointer")]
#[repr(C)]
#[derive(Debug)]
pub struct VRPointer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _leftVRController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VRController,
    >,
    pub _rightVRController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VRController,
    >,
    pub _laserPointerPrefab: quest_hook::libil2cpp::Gc<
        crate::VRUIControls::VRLaserPointer,
    >,
    pub _cursorPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _defaultLaserPointerLength: f32,
    pub _laserPointerWidth: f32,
    pub lastUsedControllerChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
        >,
    >,
    pub _currentPointerData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::PointerEventData,
    >,
    pub _laserPointer: quest_hook::libil2cpp::Gc<crate::VRUIControls::VRLaserPointer>,
    pub _cursorTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _leftLaserPointer: quest_hook::libil2cpp::Gc<
        crate::VRUIControls::VRLaserPointer,
    >,
    pub _rightLaserPointer: quest_hook::libil2cpp::Gc<
        crate::VRUIControls::VRLaserPointer,
    >,
    pub _leftCursorTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _rightCursorTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _eventSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::EventSystem,
    >,
    pub _lastSelectedVrController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::VRController,
    >,
    pub _lastSelectedControllerWasRight: bool,
    pub _rightControllerWasReleased: bool,
    pub _leftControllerWasReleased: bool,
    pub _hasLaserPointers: bool,
    pub _hasCursors: bool,
}
#[cfg(feature = "VRUIControls+VRPointer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::VRUIControls::VRPointer => "VRUIControls"
    ."VRPointer"
);
#[cfg(feature = "VRUIControls+VRPointer")]
impl std::ops::Deref for crate::VRUIControls::VRPointer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRPointer")]
impl std::ops::DerefMut for crate::VRUIControls::VRPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+VRPointer")]
impl crate::VRUIControls::VRPointer {
    pub const kScrollMultiplier: f32 = 1f32;
    pub const kTriggerThreshold: f32 = 0.1f32;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateCursors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CreateCursors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLaserPointers(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CreateLaserPointers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnabledLastSelectedPointer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnabledLastSelectedPointer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HideCursors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideCursors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HideLaserPointers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideLaserPointers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HideLaserPointersAndCursors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideLaserPointersAndCursors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLeftControllerDown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsLeftControllerDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsRightControllerDown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsRightControllerDown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnApplicationFocus(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (hasFocus))?;
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
    pub fn Process(
        &mut self,
        pointerEventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Process", (pointerEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshLaserPointerAndLaserHit(
        &mut self,
        pointerData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshLaserPointerAndLaserHit", (pointerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectLeftController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectLeftController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectRightController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectRightController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupLaserPointer(
        &mut self,
        laserPointer: quest_hook::libil2cpp::Gc<crate::VRUIControls::VRLaserPointer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupLaserPointer", (laserPointer))?;
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
    pub fn add_lastUsedControllerChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_lastUsedControllerChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursorPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_cursorPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cursorTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_cursorTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flatCanvasWorldPosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_flatCanvasWorldPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastSelectedVrController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::VRController,
        > = __cordl_object.invoke("get_lastSelectedVrController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pointingOver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = __cordl_object
            .invoke("get_pointingOver", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_state", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_lastUsedControllerChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::VRController>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_lastUsedControllerChangedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRUIControls+VRPointer")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::VRPointer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
