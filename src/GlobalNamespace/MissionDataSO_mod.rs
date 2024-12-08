#[cfg(feature = "MissionDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionDataSO {
    __cordl_parent: PersistentScriptableObject,
    pub _levelId: *mut crate::System::String,
    pub _songName: *mut crate::System::String,
    pub _beatmapCharacteristic: *mut BeatmapCharacteristicSO,
    pub _beatmapDifficulty: BeatmapDifficulty,
    pub _missionObjectives: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut MissionObjective,
    >,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub _missionHelp: *mut MissionHelpSO,
}
#[cfg(feature = "MissionDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionDataSO => ""."MissionDataSO"
);
#[cfg(feature = "MissionDataSO")]
impl std::ops::Deref for MissionDataSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataSO")]
impl std::ops::DerefMut for MissionDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataSO")]
impl MissionDataSO {
    pub fn get_beatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapCharacteristicSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapCharacteristicSO = __cordl_object
            .invoke("get_beatmapCharacteristic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameplayModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameplayModifiers = __cordl_object
            .invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missionObjectives(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut MissionObjective>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut MissionObjective,
        > = __cordl_object.invoke("get_missionObjectives", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapDifficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapDifficulty = __cordl_object
            .invoke("get_beatmapDifficulty", ())?;
        Ok(__cordl_ret)
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
    pub fn get_missionHelp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionHelpSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionHelpSO = __cordl_object
            .invoke("get_missionHelp", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_songName", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MissionDataSO")]
impl quest_hook::libil2cpp::ObjectType for MissionDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
