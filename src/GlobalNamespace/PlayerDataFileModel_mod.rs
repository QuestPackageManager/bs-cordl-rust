#[cfg(feature = "PlayerDataFileModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerDataFileModel {
    __cordl_parent: crate::System::Object,
    pub _playerDataFileManager: *mut PlayerDataFileManagerSO,
    pub _fileStorage: *mut IFileStorage,
    pub _beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
    pub _colorSchemesSettings: *mut ColorSchemesSettings,
    pub _environmentsListModel: *mut EnvironmentsListModel,
}
#[cfg(feature = "PlayerDataFileModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerDataFileModel => ""."PlayerDataFileModel"
);
#[cfg(feature = "PlayerDataFileModel")]
impl std::ops::Deref for PlayerDataFileModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataFileModel")]
impl std::ops::DerefMut for PlayerDataFileModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerDataFileModel")]
impl PlayerDataFileModel {
    pub const kPlayerDataFileName: &'static str = "PlayerData.dat";
    #[cfg(feature = "PlayerDataFileModel+_LoadAsync_d__13")]
    pub type _LoadAsync_d__13 = crate::GlobalNamespace::PlayerDataFileModel__LoadAsync_d__13;
    pub fn CreateDefaultPlayerData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerData = __cordl_object
            .invoke("CreateDefaultPlayerData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEnvironmentInfoBySerializedName(
        &mut self,
        environmentName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut EnvironmentInfoSO = __cordl_object
            .invoke("GetEnvironmentInfoBySerializedName", (environmentName))?;
        Ok(__cordl_ret)
    }
    pub fn LoadAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut PlayerData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<*mut PlayerData> = __cordl_object
            .invoke("LoadAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlayerSaveData(
        &mut self,
        playerData: *mut PlayerData,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerSaveData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSaveData = __cordl_object
            .invoke("GetPlayerSaveData", (playerData))?;
        Ok(__cordl_ret)
    }
    pub fn LoadOrCreateFromJsonString(
        &mut self,
        jsonString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerData = __cordl_object
            .invoke("LoadOrCreateFromJsonString", (jsonString))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        playerDataFileManager: *mut PlayerDataFileManagerSO,
        fileStorage: *mut IFileStorage,
        beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
        colorSchemesSettings: *mut ColorSchemesSettings,
        environmentsListModel: *mut EnvironmentsListModel,
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
        Ok(__cordl_ret)
    }
    pub fn Save(
        &mut self,
        playerData: *mut PlayerData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Save", (playerData))?;
        Ok(__cordl_ret)
    }
    pub fn LoadFromJSONString(
        &mut self,
        jsonString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerData = __cordl_object
            .invoke("LoadFromJSONString", (jsonString))?;
        Ok(__cordl_ret)
    }
    pub fn LoadFromVersionV1_0_1(
        &mut self,
        playerDataModelSaveData: *mut PlayerSaveDataV1_0_1,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerData = __cordl_object
            .invoke("LoadFromVersionV1_0_1", (playerDataModelSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn SaveAsync(
        &mut self,
        playerData: *mut PlayerData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("SaveAsync", (playerData))?;
        Ok(__cordl_ret)
    }
    pub fn Load(&mut self) -> quest_hook::libil2cpp::Result<*mut PlayerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerData = __cordl_object.invoke("Load", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadFromCurrentVersion(
        &mut self,
        playerSaveData: *mut PlayerSaveData,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerData = __cordl_object
            .invoke("LoadFromCurrentVersion", (playerSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn LoadCorrectedSongPackMask(
        &mut self,
        songMaskPackBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("LoadCorrectedSongPackMask", (songMaskPackBytes))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDefaultOverrideEnvironmentSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut OverrideEnvironmentSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OverrideEnvironmentSettings = __cordl_object
            .invoke("CreateDefaultOverrideEnvironmentSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        playerDataFileManager: *mut PlayerDataFileManagerSO,
        fileStorage: *mut IFileStorage,
        beatmapCharacteristicCollection: *mut BeatmapCharacteristicCollection,
        colorSchemesSettings: *mut ColorSchemesSettings,
        environmentsListModel: *mut EnvironmentsListModel,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerDataFileModel")]
impl quest_hook::libil2cpp::ObjectType for PlayerDataFileModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
