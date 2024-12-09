#[cfg(feature = "OVRRuntimeController")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRRuntimeController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_controller: crate::GlobalNamespace::OVRInput_Controller,
    pub m_controllerModelShader: *mut crate::UnityEngine::Shader,
    pub m_supportAnimation: bool,
    pub m_controllerObject: *mut crate::UnityEngine::GameObject,
    pub m_controllerModelPath: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_modelSupported: bool,
    pub m_hasInputFocus: bool,
    pub m_hasInputFocusPrev: bool,
    pub m_controllerConnectedPrev: bool,
    pub m_animationNodes: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::OVRGLTFInputNode,
        *mut crate::GlobalNamespace::OVRGLTFAnimatinonNode,
    >,
}
#[cfg(feature = "OVRRuntimeController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRRuntimeController => ""
    ."OVRRuntimeController"
);
#[cfg(feature = "OVRRuntimeController")]
impl std::ops::Deref for crate::GlobalNamespace::OVRRuntimeController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRRuntimeController")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRRuntimeController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRRuntimeController")]
impl crate::GlobalNamespace::OVRRuntimeController {
    #[cfg(feature = "OVRRuntimeController+_UpdateControllerModel_d__16")]
    pub type _UpdateControllerModel_d__16 = crate::GlobalNamespace::OVRRuntimeController__UpdateControllerModel_d__16;
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
    pub fn IsModelSupported(
        &mut self,
        modelPath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsModelSupported", (modelPath))?;
        Ok(__cordl_ret)
    }
    pub fn LoadControllerModel(
        &mut self,
        modelPath: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadControllerModel", (modelPath))?;
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
    pub fn UpdateControllerAnimation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateControllerAnimation", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateControllerModel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("UpdateControllerModel", ())?;
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
#[cfg(feature = "OVRRuntimeController")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRRuntimeController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
