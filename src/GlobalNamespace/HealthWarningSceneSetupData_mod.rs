#[cfg(feature = "HealthWarningSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct HealthWarningSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub _nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    >,
}
#[cfg(feature = "HealthWarningSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HealthWarningSceneSetupData =>
    ""."HealthWarningSceneSetupData"
);
#[cfg(feature = "HealthWarningSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::HealthWarningSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::HealthWarningSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HealthWarningSceneSetupData")]
impl crate::GlobalNamespace::HealthWarningSceneSetupData {
    pub fn New(
        nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        nextScenesTransitionSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_nextScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ScenesTransitionSetupDataSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        > = __cordl_object.invoke("get_nextScenesTransitionSetupData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HealthWarningSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HealthWarningSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
