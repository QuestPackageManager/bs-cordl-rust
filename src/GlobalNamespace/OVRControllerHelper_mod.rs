#[cfg(feature = "OVRControllerHelper+ControllerType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRControllerHelper_ControllerType {
    Quest2 = 3i32,
    QuestAndRiftS = 1i32,
    Rift = 2i32,
    TouchPlus = 5i32,
    TouchPro = 4i32,
}
#[cfg(feature = "OVRControllerHelper+ControllerType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRControllerHelper_ControllerType => ""
    ."OVRControllerHelper/ControllerType"
);
#[cfg(feature = "OVRControllerHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRControllerHelper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_modelOculusTouchQuestAndRiftSLeftController: *mut crate::UnityEngine::GameObject,
    pub m_modelOculusTouchQuestAndRiftSRightController: *mut crate::UnityEngine::GameObject,
    pub m_modelOculusTouchRiftLeftController: *mut crate::UnityEngine::GameObject,
    pub m_modelOculusTouchRiftRightController: *mut crate::UnityEngine::GameObject,
    pub m_modelOculusTouchQuest2LeftController: *mut crate::UnityEngine::GameObject,
    pub m_modelOculusTouchQuest2RightController: *mut crate::UnityEngine::GameObject,
    pub m_modelMetaTouchProLeftController: *mut crate::UnityEngine::GameObject,
    pub m_modelMetaTouchProRightController: *mut crate::UnityEngine::GameObject,
    pub m_modelMetaTouchPlusLeftController: *mut crate::UnityEngine::GameObject,
    pub m_modelMetaTouchPlusRightController: *mut crate::UnityEngine::GameObject,
    pub m_controller: crate::GlobalNamespace::OVRInput_Controller,
    pub m_showState: crate::GlobalNamespace::OVRInput_InputDeviceShowState,
    pub showWhenHandsArePoweredByNaturalControllerPoses: bool,
    pub m_animator: *mut crate::UnityEngine::Animator,
    pub m_activeController: *mut crate::UnityEngine::GameObject,
    pub m_controllerModelsInitialized: bool,
    pub m_hasInputFocus: bool,
    pub m_hasInputFocusPrev: bool,
    pub activeControllerType: crate::GlobalNamespace::OVRControllerHelper_ControllerType,
    pub m_prevControllerConnected: bool,
    pub m_prevControllerConnectedCached: bool,
    pub m_prevControllerInHandState: crate::GlobalNamespace::OVRInput_ControllerInHandState,
}
#[cfg(feature = "OVRControllerHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRControllerHelper => ""."OVRControllerHelper"
);
#[cfg(feature = "OVRControllerHelper")]
impl std::ops::Deref for OVRControllerHelper {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerHelper")]
impl std::ops::DerefMut for OVRControllerHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRControllerHelper")]
impl OVRControllerHelper {
    #[cfg(feature = "OVRControllerHelper+ControllerType")]
    pub type ControllerType = crate::GlobalNamespace::OVRControllerHelper_ControllerType;
    pub fn InitializeControllerModels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeControllerModels", ())?;
        Ok(__cordl_ret)
    }
    pub fn InputFocusAquired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InputFocusAquired", ())?;
        Ok(__cordl_ret)
    }
    pub fn InputFocusLost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InputFocusLost", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
}
#[cfg(feature = "OVRControllerHelper")]
impl quest_hook::libil2cpp::ObjectType for OVRControllerHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
