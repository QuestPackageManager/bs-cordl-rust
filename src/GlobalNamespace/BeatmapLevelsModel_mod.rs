#[cfg(feature = "BeatmapLevelsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ostAndExtrasBeatmapLevelsRepository: *mut crate::GlobalNamespace::BeatmapLevelsRepository,
    pub dlcBeatmapLevelsRepository: *mut crate::GlobalNamespace::BeatmapLevelsRepository,
    pub allPacksCount: i32,
    pub levelLoader: *mut crate::GlobalNamespace::IBeatmapLevelLoader,
    pub _packDefinitions: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PackDefinitionSO,
    >,
    pub _playerDataModel: *mut crate::GlobalNamespace::IPlayerDataModel,
    pub _localizationModel: *mut crate::BGLib::Polyglot::LocalizationModel,
    pub _entitlements: *mut crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    pub _allLoadedBeatmapLevelsRepository: *mut crate::GlobalNamespace::BeatmapLevelsRepository,
    pub _allExistingBeatmapLevelsRepository: *mut crate::GlobalNamespace::BeatmapLevelsRepository,
    pub newPackWasCreatedFromDefinitionEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::BeatmapLevelPack,
    >,
}
#[cfg(feature = "BeatmapLevelsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelsModel => ""
    ."BeatmapLevelsModel"
);
#[cfg(feature = "BeatmapLevelsModel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl crate::GlobalNamespace::BeatmapLevelsModel {
    pub const kExplicitSongLocalizationKey: &'static str = "EXPLICIT_SONG";
    #[cfg(feature = "BeatmapLevelsModel+_CheckBeatmapLevelDataExistsAsync_d__27")]
    pub type _CheckBeatmapLevelDataExistsAsync_d__27 = crate::GlobalNamespace::BeatmapLevelsModel__CheckBeatmapLevelDataExistsAsync_d__27;
    #[cfg(feature = "BeatmapLevelsModel+_LoadBeatmapLevelDataAsync_d__26")]
    pub type _LoadBeatmapLevelDataAsync_d__26 = crate::GlobalNamespace::BeatmapLevelsModel__LoadBeatmapLevelDataAsync_d__26;
    #[cfg(feature = "BeatmapLevelsModel+_SelectPacks_d__28")]
    pub type _SelectPacks_d__28 = crate::GlobalNamespace::BeatmapLevelsModel__SelectPacks_d__28;
    #[cfg(feature = "BeatmapLevelsModel+__c__DisplayClass31_0")]
    pub type __c__DisplayClass31_0 = crate::GlobalNamespace::BeatmapLevelsModel___c__DisplayClass31_0;
    pub fn CheckBeatmapLevelDataExistsAsync(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = __cordl_object
            .invoke(
                "CheckBeatmapLevelDataExistsAsync",
                (levelID, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearLoadedBeatmapLevelsCaches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearLoadedBeatmapLevelsCaches", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBeatmapLevelPack(
        &mut self,
        pack: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
        desiredSensitivityFlag: crate::GlobalNamespace::PlayerSensitivityFlag,
        censoredLocalizedSongName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ValueTuple_2<
            *mut crate::GlobalNamespace::BeatmapLevelPack,
            *mut crate::GlobalNamespace::BeatmapLevelPack,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<
            *mut crate::GlobalNamespace::BeatmapLevelPack,
            *mut crate::GlobalNamespace::BeatmapLevelPack,
        > = __cordl_object
            .invoke(
                "CreateBeatmapLevelPack",
                (pack, desiredSensitivityFlag, censoredLocalizedSongName),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        > = __cordl_object.invoke("GetAllPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevel(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = __cordl_object.invoke("GetBeatmapLevel", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelPack(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = __cordl_object.invoke("GetLevelPack", (levelPackId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelPackForLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = __cordl_object.invoke("GetLevelPackForLevelId", (levelId))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadAllBeatmapLevelPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadBeatmapLevelDataAsync(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        > = __cordl_object
            .invoke(
                "LoadBeatmapLevelDataAsync",
                (levelID, beatmapLevelDataVersion, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        entitlementChecker: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        beatmapLevelLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelLoader,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlayerDataModel,
        >,
        localizationModel: quest_hook::libil2cpp::Gc<
            crate::BGLib::Polyglot::LocalizationModel,
        >,
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn ReloadAllBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadAllBeatmapLevelPacks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReloadCustomLevelPackCollectionAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::BeatmapLevelsRepository,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::BeatmapLevelsRepository,
            >,
        > = __cordl_object
            .invoke("ReloadCustomLevelPackCollectionAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectPacks(
        &mut self,
        include: crate::GlobalNamespace::PackDefinitionSO_Tags,
        exclude: crate::GlobalNamespace::PackDefinitionSO_Tags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        > = __cordl_object.invoke("SelectPacks", (include, exclude))?;
        Ok(__cordl_ret.into())
    }
    pub fn _CreateBeatmapLevelPack_g__ShouldBeKeptIntact_31_2(
        desiredSensitivityFlag: crate::GlobalNamespace::PlayerSensitivityFlag,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "<CreateBeatmapLevelPack>g__ShouldBeKeptIntact|31_2",
                (desiredSensitivityFlag, contentRating),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        entitlementChecker: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
        >,
        beatmapLevelLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLevelLoader,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlayerDataModel,
        >,
        localizationModel: quest_hook::libil2cpp::Gc<
            crate::BGLib::Polyglot::LocalizationModel,
        >,
        packDefinitions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn add_newPackWasCreatedFromDefinitionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_newPackWasCreatedFromDefinitionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_entitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEntitlementModel>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEntitlementModel,
        > = __cordl_object.invoke("get_entitlements", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_packDefinitions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::PackDefinitionSO,
            >,
        > = __cordl_object.invoke("get_packDefinitions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_newPackWasCreatedFromDefinitionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_newPackWasCreatedFromDefinitionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevelsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::BeatmapLevelsModel {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::BeatmapLevelsModel {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
