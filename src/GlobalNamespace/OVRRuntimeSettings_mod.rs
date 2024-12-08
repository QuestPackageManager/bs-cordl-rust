#[cfg(feature = "OVRRuntimeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRRuntimeSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub colorSpace: crate::GlobalNamespace::OVRManager_ColorSpace,
    pub hasSentConsentEvent: bool,
    pub hasSetTelemetryEnabled: bool,
    pub telemetryEnabled: bool,
}
#[cfg(feature = "OVRRuntimeSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRRuntimeSettings => ""
    ."OVRRuntimeSettings"
);
#[cfg(feature = "OVRRuntimeSettings")]
impl std::ops::Deref for crate::GlobalNamespace::OVRRuntimeSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRRuntimeSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRRuntimeSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRRuntimeSettings")]
impl crate::GlobalNamespace::OVRRuntimeSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_HasSetTelemetryEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasSetTelemetryEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TelemetryEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_TelemetryEnabled", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRRuntimeSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRRuntimeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
