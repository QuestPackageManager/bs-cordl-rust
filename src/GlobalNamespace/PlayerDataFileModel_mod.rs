#[cfg(feature = "PlayerDataFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerDataFileModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _playerDataFileManager: *mut crate::GlobalNamespace::PlayerDataFileManagerSO,
    pub _fileStorage: *mut crate::GlobalNamespace::IFileStorage,
    pub _beatmapCharacteristicCollection: *mut crate::GlobalNamespace::BeatmapCharacteristicCollection,
    pub _colorSchemesSettings: *mut crate::GlobalNamespace::ColorSchemesSettings,
    pub _environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
}
#[cfg(feature = "PlayerDataFileModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlayerDataFileModel => ""
    ."PlayerDataFileModel"
);
#[cfg(feature = "PlayerDataFileModel")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerDataFileModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataFileModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerDataFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataFileModel")]
impl crate::GlobalNamespace::PlayerDataFileModel {
    pub const kPlayerDataFileName: &'static str = "PlayerData.dat";
    #[cfg(feature = "PlayerDataFileModel+_LoadAsync_d__13")]
    pub type _LoadAsync_d__13 = crate::GlobalNamespace::PlayerDataFileModel__LoadAsync_d__13;
    pub fn ColorOverrideTypeFromSaveData(
        &mut self,
        c: crate::GlobalNamespace::PlayerSaveData_ColorOverrideType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType = __cordl_object
            .invoke("ColorOverrideTypeFromSaveData", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn ColorOverrideTypeToSaveData(
        &mut self,
        c: crate::GlobalNamespace::ColorSchemesSettings_ColorOverrideType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::PlayerSaveData_ColorOverrideType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlayerSaveData_ColorOverrideType = __cordl_object
            .invoke("ColorOverrideTypeToSaveData", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultOverrideEnvironmentSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OverrideEnvironmentSettings>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::OverrideEnvironmentSettings,
        > = __cordl_object.invoke("CreateDefaultOverrideEnvironmentSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDefaultPlayerData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData> = __cordl_object
            .invoke("CreateDefaultPlayerData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapCharacteristicFromV_1_0_1LevelId(
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetBeatmapCharacteristicFromV_1_0_1LevelId",
                (beatmapCharacteristicCollection, levelId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentInfoBySerializedName(
        &mut self,
        environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentInfoSO,
        > = __cordl_object
            .invoke("GetEnvironmentInfoBySerializedName", (environmentName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelIdFromV_1_0_1LevelId(
        oldLevelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetLevelIdFromV_1_0_1LevelId",
                (oldLevelId, beatmapCharacteristic),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerSaveData(
        &mut self,
        playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSaveData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveData,
        > = __cordl_object.invoke("GetPlayerSaveData", (playerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn Load(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData> = __cordl_object
            .invoke("Load", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::PlayerData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::PlayerData,
            >,
        > = __cordl_object.invoke("LoadAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadCorrectedSongPackMask(
        &mut self,
        songMaskPackBytes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("LoadCorrectedSongPackMask", (songMaskPackBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromCurrentVersion(
        &mut self,
        playerSaveData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSaveData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData> = __cordl_object
            .invoke("LoadFromCurrentVersion", (playerSaveData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromJSONString(
        &mut self,
        jsonString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData> = __cordl_object
            .invoke("LoadFromJSONString", (jsonString))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromVersionV1_0_1(
        &mut self,
        playerDataModelSaveData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSaveDataV1_0_1,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData> = __cordl_object
            .invoke("LoadFromVersionV1_0_1", (playerDataModelSaveData))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadOrCreateFromJsonString(
        &mut self,
        jsonString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData> = __cordl_object
            .invoke("LoadOrCreateFromJsonString", (jsonString))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        playerDataFileManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataFileManagerSO,
        >,
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    playerDataFileManager,
                    fileStorage,
                    beatmapCharacteristicCollection,
                    colorSchemesSettings,
                    environmentsListModel,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Save(
        &mut self,
        playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (playerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveAsync(
        &mut self,
        playerData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("SaveAsync", (playerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        playerDataFileManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataFileManagerSO,
        >,
        fileStorage: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFileStorage>,
        beatmapCharacteristicCollection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicCollection,
        >,
        colorSchemesSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorSchemesSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    playerDataFileManager,
                    fileStorage,
                    beatmapCharacteristicCollection,
                    colorSchemesSettings,
                    environmentsListModel,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerDataFileModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PlayerDataFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
