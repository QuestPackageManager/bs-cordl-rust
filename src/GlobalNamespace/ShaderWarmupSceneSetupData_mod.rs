#[cfg(feature = "ShaderWarmupSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct ShaderWarmupSceneSetupData {
    __cordl_parent: crate::GlobalNamespace::SceneSetupData,
    pub _nextScenesTransitionSetupData_k__BackingField: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShaderWarmupSceneSetupData =>
    ""."ShaderWarmupSceneSetupData"
);
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl std::ops::Deref for crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    type Target = crate::GlobalNamespace::SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    pub fn New(
        nextScenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        nextScenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_ret)
    }
    pub fn get_nextScenesTransitionSetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO = __cordl_object
            .invoke("get_nextScenesTransitionSetupData", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_nextScenesTransitionSetupData(
        &mut self,
        value: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_nextScenesTransitionSetupData", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ShaderWarmupSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ShaderWarmupSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
