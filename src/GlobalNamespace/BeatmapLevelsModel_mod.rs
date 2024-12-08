#[cfg(feature = "BeatmapLevelsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsModel {
    __cordl_parent: crate::System::Object,
    pub ostAndExtrasBeatmapLevelsRepository: *mut BeatmapLevelsRepository,
    pub dlcBeatmapLevelsRepository: *mut BeatmapLevelsRepository,
    pub allPacksCount: i32,
    pub levelLoader: *mut IBeatmapLevelLoader,
    pub _packDefinitions: *mut quest_hook::libil2cpp::Il2CppArray<*mut PackDefinitionSO>,
    pub _playerDataModel: *mut IPlayerDataModel,
    pub _localizationModel: *mut crate::BGLib::Polyglot::LocalizationModel,
    pub _entitlements: *mut BeatmapLevelsEntitlementModel,
    pub _allLoadedBeatmapLevelsRepository: *mut BeatmapLevelsRepository,
    pub _allExistingBeatmapLevelsRepository: *mut BeatmapLevelsRepository,
}
#[cfg(feature = "BeatmapLevelsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelsModel => ""."BeatmapLevelsModel"
);
#[cfg(feature = "BeatmapLevelsModel")]
impl std::ops::Deref for BeatmapLevelsModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl std::ops::DerefMut for BeatmapLevelsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl BeatmapLevelsModel {
    pub const kExplicitSongLocalizationKey: &'static str = "EXPLICIT_SONG";
    #[cfg(feature = "BeatmapLevelsModel+_SelectPacks_d__25")]
    pub type _SelectPacks_d__25 = crate::GlobalNamespace::BeatmapLevelsModel__SelectPacks_d__25;
    #[cfg(feature = "BeatmapLevelsModel+_LoadBeatmapLevelDataAsync_d__23")]
    pub type _LoadBeatmapLevelDataAsync_d__23 = crate::GlobalNamespace::BeatmapLevelsModel__LoadBeatmapLevelDataAsync_d__23;
    #[cfg(feature = "BeatmapLevelsModel+_CheckBeatmapLevelDataExistsAsync_d__24")]
    pub type _CheckBeatmapLevelDataExistsAsync_d__24 = crate::GlobalNamespace::BeatmapLevelsModel__CheckBeatmapLevelDataExistsAsync_d__24;
    #[cfg(feature = "BeatmapLevelsModel+__c__DisplayClass27_0")]
    pub type __c__DisplayClass27_0 = crate::GlobalNamespace::BeatmapLevelsModel___c__DisplayClass27_0;
    pub fn GetLevelPackForLevelId(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelPack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelPack = __cordl_object
            .invoke("GetLevelPackForLevelId", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn CheckBeatmapLevelDataExistsAsync(
        &mut self,
        levelID: *mut crate::System::String,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke(
                "CheckBeatmapLevelDataExistsAsync",
                (levelID, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReloadCustomLevelPackCollectionAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut BeatmapLevelsRepository>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut BeatmapLevelsRepository,
        > = __cordl_object
            .invoke("ReloadCustomLevelPackCollectionAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn GetLevelPack(
        &mut self,
        levelPackId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelPack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelPack = __cordl_object
            .invoke("GetLevelPack", (levelPackId))?;
        Ok(__cordl_ret)
    }
    pub fn SelectPacks(
        &mut self,
        include: crate::GlobalNamespace::PackDefinitionSO_Tags,
        exclude: crate::GlobalNamespace::PackDefinitionSO_Tags,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut PackDefinitionSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut PackDefinitionSO,
        > = __cordl_object.invoke("SelectPacks", (include, exclude))?;
        Ok(__cordl_ret)
    }
    pub fn get_packDefinitions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut PackDefinitionSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut PackDefinitionSO,
        > = __cordl_object.invoke("get_packDefinitions", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        entitlementChecker: *mut BeatmapLevelsEntitlementModel,
        beatmapLevelLoader: *mut IBeatmapLevelLoader,
        playerDataModel: *mut IPlayerDataModel,
        localizationModel: *mut crate::BGLib::Polyglot::LocalizationModel,
        packDefinitions: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut PackDefinitionSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    entitlementChecker,
                    beatmapLevelLoader,
                    playerDataModel,
                    localizationModel,
                    packDefinitions,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapLevel(
        &mut self,
        levelId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevel = __cordl_object
            .invoke("GetBeatmapLevel", (levelId))?;
        Ok(__cordl_ret)
    }
    pub fn get_entitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IEntitlementModel> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IEntitlementModel = __cordl_object
            .invoke("get_entitlements", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadBeatmapLevelDataAsync(
        &mut self,
        levelID: *mut crate::System::String,
        beatmapLevelDataVersion: BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<LoadBeatmapLevelDataResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            LoadBeatmapLevelDataResult,
        > = __cordl_object
            .invoke(
                "LoadBeatmapLevelDataAsync",
                (levelID, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetAllPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut BeatmapLevelPack,
        > = __cordl_object.invoke("GetAllPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn LoadAllBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadAllBeatmapLevelPacks", ())?;
        Ok(__cordl_ret)
    }
    pub fn ClearLoadedBeatmapLevelsCaches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLoadedBeatmapLevelsCaches", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        entitlementChecker: *mut BeatmapLevelsEntitlementModel,
        beatmapLevelLoader: *mut IBeatmapLevelLoader,
        playerDataModel: *mut IPlayerDataModel,
        localizationModel: *mut crate::BGLib::Polyglot::LocalizationModel,
        packDefinitions: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut PackDefinitionSO,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    entitlementChecker,
                    beatmapLevelLoader,
                    playerDataModel,
                    localizationModel,
                    packDefinitions,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
