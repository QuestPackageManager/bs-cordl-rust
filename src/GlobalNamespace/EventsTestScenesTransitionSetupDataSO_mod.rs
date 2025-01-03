#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct EventsTestScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::ScenesTransitionSetupDataSO,
    pub _environmentInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub _eventsTestSceneInfo: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SceneInfo,
    >,
    pub _gameCoreSceneInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SceneInfo>,
}
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EventsTestScenesTransitionSetupDataSO => ""
    ."EventsTestScenesTransitionSetupDataSO"
);
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::EventsTestScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::ScenesTransitionSetupDataSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EventsTestScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::EventsTestScenesTransitionSetupDataSO {
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
#[cfg(feature = "EventsTestScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EventsTestScenesTransitionSetupDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
