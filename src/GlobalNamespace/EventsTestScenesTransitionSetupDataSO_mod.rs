#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct EventsTestScenesTransitionSetupDataSO {
    __cordl_parent: ScenesTransitionSetupDataSO,
    pub _environmentInfo: *mut EnvironmentInfoSO,
    pub _eventsTestSceneInfo: *mut SceneInfo,
    pub _gameCoreSceneInfo: *mut SceneInfo,
}
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EventsTestScenesTransitionSetupDataSO => ""
    ."EventsTestScenesTransitionSetupDataSO"
);
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl std::ops::Deref for EventsTestScenesTransitionSetupDataSO {
    type Target = ScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for EventsTestScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl EventsTestScenesTransitionSetupDataSO {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType for EventsTestScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
