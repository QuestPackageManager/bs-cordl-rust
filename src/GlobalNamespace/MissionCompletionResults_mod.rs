#[cfg(feature = "MissionCompletionResults")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionCompletionResults {
    __cordl_parent: crate::System::Object,
    pub levelCompletionResults: *mut LevelCompletionResults,
    pub missionObjectiveResults: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MissionObjectiveResult,
    >,
}
#[cfg(feature = "MissionCompletionResults")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionCompletionResults => ""
    ."MissionCompletionResults"
);
#[cfg(feature = "MissionCompletionResults")]
impl std::ops::Deref for MissionCompletionResults {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionCompletionResults")]
impl std::ops::DerefMut for MissionCompletionResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionCompletionResults")]
impl MissionCompletionResults {
    pub fn New(
        levelCompletionResults: *mut LevelCompletionResults,
        missionObjectiveResults: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionObjectiveResult,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (levelCompletionResults, missionObjectiveResults))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        levelCompletionResults: *mut LevelCompletionResults,
        missionObjectiveResults: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionObjectiveResult,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (levelCompletionResults, missionObjectiveResults))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsMissionComplete(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsMissionComplete", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionCompletionResults")]
impl quest_hook::libil2cpp::ObjectType for MissionCompletionResults {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
