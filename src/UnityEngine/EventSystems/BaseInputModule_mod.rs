#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
#[repr(C)]
#[derive(Debug)]
pub struct BaseInputModule {
    __cordl_parent: crate::UnityEngine::EventSystems::UIBehaviour,
    pub m_RaycastResultCache: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::EventSystems::RaycastResult,
    >,
    pub m_SendPointerHoverToParent: bool,
    pub m_AxisEventData: *mut crate::UnityEngine::EventSystems::AxisEventData,
    pub m_EventSystem: *mut crate::UnityEngine::EventSystems::EventSystem,
    pub m_BaseEventData: *mut crate::UnityEngine::EventSystems::BaseEventData,
    pub m_InputOverride: *mut crate::UnityEngine::EventSystems::BaseInput,
    pub m_DefaultInput: *mut crate::UnityEngine::EventSystems::BaseInput,
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventSystems::BaseInputModule =>
    "UnityEngine.EventSystems"."BaseInputModule"
);
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::BaseInputModule {
    type Target = crate::UnityEngine::EventSystems::UIBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::BaseInputModule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl crate::UnityEngine::EventSystems::BaseInputModule {
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
    pub fn get_sendPointerHoverToParent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_sendPointerHoverToParent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAxisEventData(
        &mut self,
        x: f32,
        y: f32,
        moveDeadZone: f32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::AxisEventData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::AxisEventData = __cordl_object
            .invoke("GetAxisEventData", (x, y, moveDeadZone))?;
        Ok(__cordl_ret)
    }
    pub fn IsModuleSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsModuleSupported", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_sendPointerHoverToParent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sendPointerHoverToParent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_inputOverride(
        &mut self,
        value: *mut crate::UnityEngine::EventSystems::BaseInput,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_inputOverride", (value))?;
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
    pub fn get_eventSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::EventSystem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::EventSystem = __cordl_object
            .invoke("get_eventSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_input(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::BaseInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::BaseInput = __cordl_object
            .invoke("get_input", ())?;
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
    pub fn ShouldActivateModule(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldActivateModule", ())?;
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
    pub fn get_inputOverride(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::BaseInput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::BaseInput = __cordl_object
            .invoke("get_inputOverride", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBaseEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::EventSystems::BaseEventData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::EventSystems::BaseEventData = __cordl_object
            .invoke("GetBaseEventData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+EventSystems+BaseInputModule")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::BaseInputModule {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
