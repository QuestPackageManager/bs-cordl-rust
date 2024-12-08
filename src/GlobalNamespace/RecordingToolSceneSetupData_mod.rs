#[cfg(feature = "RecordingToolSceneSetupData")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolSceneSetupData {
    __cordl_parent: SceneSetupData,
    pub _nextScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
}
#[cfg(feature = "RecordingToolSceneSetupData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RecordingToolSceneSetupData => ""
    ."RecordingToolSceneSetupData"
);
#[cfg(feature = "RecordingToolSceneSetupData")]
impl std::ops::Deref for RecordingToolSceneSetupData {
    type Target = SceneSetupData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSceneSetupData")]
impl std::ops::DerefMut for RecordingToolSceneSetupData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolSceneSetupData")]
impl RecordingToolSceneSetupData {
    pub fn New(
        nextScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (nextScenesTransitionSetupData))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        nextScenesTransitionSetupData: *mut ScenesTransitionSetupDataSO,
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
    ) -> quest_hook::libil2cpp::Result<*mut ScenesTransitionSetupDataSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ScenesTransitionSetupDataSO = __cordl_object
            .invoke("get_nextScenesTransitionSetupData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RecordingToolSceneSetupData")]
impl quest_hook::libil2cpp::ObjectType for RecordingToolSceneSetupData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
