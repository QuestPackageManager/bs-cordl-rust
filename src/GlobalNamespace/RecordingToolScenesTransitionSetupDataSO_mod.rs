#[cfg(feature = "RecordingToolScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct RecordingToolScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO,
}
#[cfg(feature = "RecordingToolScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO => ""
    ."RecordingToolScenesTransitionSetupDataSO"
);
#[cfg(feature = "RecordingToolScenesTransitionSetupDataSO")]
impl std::ops::Deref
for crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::SingleFixedSceneScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolScenesTransitionSetupDataSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RecordingToolScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO {
    pub fn Init(
        &mut self,
        recordingToolSceneSetupData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::RecordingToolSceneSetupData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (recordingToolSceneSetupData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RecordingToolScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RecordingToolScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
