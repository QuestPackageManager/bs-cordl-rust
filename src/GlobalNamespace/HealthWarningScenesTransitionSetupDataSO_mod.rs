#[cfg(feature = "HealthWarningScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO,
}
#[cfg(feature = "HealthWarningScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO => ""
    ."HealthWarningScenesTransitionSetupDataSO"
);
#[cfg(feature = "HealthWarningScenesTransitionSetupDataSO")]
impl std::ops::Deref
for crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningScenesTransitionSetupDataSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO {
    pub fn Init(
        &mut self,
        healthWarningSceneSetupData: *mut crate::GlobalNamespace::HealthWarningSceneSetupData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (healthWarningSceneSetupData))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "HealthWarningScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HealthWarningScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
