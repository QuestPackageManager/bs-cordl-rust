#[cfg(feature = "IUnityXRHapticsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IUnityXRHapticsHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IUnityXRHapticsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IUnityXRHapticsHandler => ""
    ."IUnityXRHapticsHandler"
);
#[cfg(feature = "IUnityXRHapticsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::IUnityXRHapticsHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IUnityXRHapticsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::IUnityXRHapticsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IUnityXRHapticsHandler")]
impl crate::GlobalNamespace::IUnityXRHapticsHandler {
    pub fn StopHaptics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopHaptics", ())?;
        Ok(__cordl_ret)
    }
    pub fn TriggerHapticPulse(
        &mut self,
        strength: f32,
        duration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerHapticPulse", (strength, duration))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IUnityXRHapticsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IUnityXRHapticsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
