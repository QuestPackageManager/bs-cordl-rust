#[cfg(feature = "MissionCompletionResults")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionCompletionResults {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub levelCompletionResults: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelCompletionResults,
    >,
    pub missionObjectiveResults: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjectiveResult>,
        >,
    >,
}
#[cfg(feature = "MissionCompletionResults")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionCompletionResults => ""
    ."MissionCompletionResults"
);
#[cfg(feature = "MissionCompletionResults")]
impl std::ops::Deref for crate::GlobalNamespace::MissionCompletionResults {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionCompletionResults")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionCompletionResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionCompletionResults")]
impl crate::GlobalNamespace::MissionCompletionResults {
    pub fn New(
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        missionObjectiveResults: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjectiveResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelCompletionResults, missionObjectiveResults))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        missionObjectiveResults: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionObjectiveResult>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelCompletionResults, missionObjectiveResults))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsMissionComplete(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsMissionComplete", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionCompletionResults")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionCompletionResults {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
