#[cfg(feature = "MissionDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionDataSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _beatmapCharacteristic: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub _beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _missionObjectives: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::MissionObjective>,
    >,
    pub _gameplayModifiers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiers,
    >,
    pub _missionHelp: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionHelpSO>,
}
#[cfg(feature = "MissionDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionDataSO => ""
    ."MissionDataSO"
);
#[cfg(feature = "MissionDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::MissionDataSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionDataSO")]
impl crate::GlobalNamespace::MissionDataSO {
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
    pub fn get_beatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        > = __cordl_object.invoke("get_beatmapCharacteristic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapDifficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_beatmapDifficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        > = __cordl_object.invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missionHelp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MissionHelpSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionHelpSO,
        > = __cordl_object.invoke("get_missionHelp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_missionObjectives(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::MissionObjective,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::MissionObjective,
            >,
        > = __cordl_object.invoke("get_missionObjectives", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionDataSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MissionDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
