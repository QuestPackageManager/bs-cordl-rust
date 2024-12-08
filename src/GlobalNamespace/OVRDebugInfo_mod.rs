#[cfg(feature = "OVRDebugInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRDebugInfo {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub debugUIManager: *mut crate::UnityEngine::GameObject,
    pub debugUIObject: *mut crate::UnityEngine::GameObject,
    pub riftPresent: *mut crate::UnityEngine::GameObject,
    pub fps: *mut crate::UnityEngine::GameObject,
    pub ipd: *mut crate::UnityEngine::GameObject,
    pub fov: *mut crate::UnityEngine::GameObject,
    pub height: *mut crate::UnityEngine::GameObject,
    pub depth: *mut crate::UnityEngine::GameObject,
    pub resolutionEyeTexture: *mut crate::UnityEngine::GameObject,
    pub latencies: *mut crate::UnityEngine::GameObject,
    pub texts: *mut crate::UnityEngine::GameObject,
    pub strRiftPresent: *mut crate::System::String,
    pub strFPS: *mut crate::System::String,
    pub strIPD: *mut crate::System::String,
    pub strFOV: *mut crate::System::String,
    pub strHeight: *mut crate::System::String,
    pub strDepth: *mut crate::System::String,
    pub strResolutionEyeTexture: *mut crate::System::String,
    pub strLatencies: *mut crate::System::String,
    pub updateInterval: f32,
    pub accum: f32,
    pub frames: i32,
    pub timeLeft: f32,
    pub initUIComponent: bool,
    pub isInited: bool,
    pub offsetY: f32,
    pub riftPresentTimeout: f32,
    pub showVRVars: bool,
}
#[cfg(feature = "OVRDebugInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRDebugInfo => ""
    ."OVRDebugInfo"
);
#[cfg(feature = "OVRDebugInfo")]
impl std::ops::Deref for crate::GlobalNamespace::OVRDebugInfo {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDebugInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRDebugInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDebugInfo")]
impl crate::GlobalNamespace::OVRDebugInfo {
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
    pub fn ComponentComposition(
        &mut self,
        GO: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("ComponentComposition", (GO))?;
        Ok(__cordl_ret)
    }
    pub fn InitUIComponents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitUIComponents", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn RiftPresentGUI(
        &mut self,
        guiMainOBj: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RiftPresentGUI", (guiMainOBj))?;
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
    pub fn UpdateDeviceDetection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateDeviceDetection", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateEyeDepthOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateEyeDepthOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateEyeHeightOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateEyeHeightOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateFOV(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFOV", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateFPS(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFPS", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateIPD(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateIPD", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateLatencyValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateLatencyValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateResolutionEyeTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateResolutionEyeTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateStrings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateStrings", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVariable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVariable", ())?;
        Ok(__cordl_ret)
    }
    pub fn VariableObjectManager(
        &mut self,
        gameObject: *mut crate::UnityEngine::GameObject,
        name: *mut crate::System::String,
        posY: f32,
        str: *mut crate::System::String,
        fontSize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("VariableObjectManager", (gameObject, name, posY, str, fontSize))?;
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
#[cfg(feature = "OVRDebugInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRDebugInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
