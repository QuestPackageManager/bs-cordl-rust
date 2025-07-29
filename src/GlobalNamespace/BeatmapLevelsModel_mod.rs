#[cfg(feature = "cordl_class_BeatmapLevelsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ostAndExtrasBeatmapLevelsRepository: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsRepository,
    >,
    pub dlcBeatmapLevelsRepository: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsRepository,
    >,
    pub allPacksCount: i32,
    pub levelLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLevelLoader,
    >,
    pub _packDefinitions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
        >,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPlayerDataModel,
    >,
    pub _localizationModel: quest_hook::libil2cpp::Gc<
        crate::BGLib::Polyglot::LocalizationModel,
    >,
    pub _entitlements: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
    >,
    pub _allLoadedBeatmapLevelsRepository: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsRepository,
    >,
    pub _allExistingBeatmapLevelsRepository: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsRepository,
    >,
    pub newPackWasCreatedFromDefinitionEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatmapLevelsModel")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatmapLevelsModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLevelsModel";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelsModel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelsModel")]
impl crate::GlobalNamespace::BeatmapLevelsModel {
    pub const kExplicitSongLocalizationKey: &'static str = "EXPLICIT_SONG";
    pub fn CheckBeatmapLevelDataExistsAsync(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapLevelDataVersion: crate::GlobalNamespace::BeatmapLevelDataVersion,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapLevelDataVersion,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<bool>,
                        >,
                        3usize,
                    >("CheckBeatmapLevelDataExistsAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckBeatmapLevelDataExistsAsync", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (levelID, beatmapLevelDataVersion, cancellationToken),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearLoadedBeatmapLevelsCaches(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ClearLoadedBeatmapLevelsCaches")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearLoadedBeatmapLevelsCaches", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PackDefinitionSO,
                            >,
                            crate::GlobalNamespace::PlayerSensitivityFlag,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        crate::System::ValueTuple_2<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelPack,
                            >,
                        >,
                        3usize,
                    >("CreateBeatmapLevelPack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateBeatmapLevelPack", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (pack, desiredSensitivityFlag, censoredLocalizedSongName),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevelPack,
                                >,
                            >,
                        >,
                        0usize,
                    >("GetAllPacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetAllPacks", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapLevel(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                        1usize,
                    >("GetBeatmapLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetBeatmapLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (levelId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelPack(
        &mut self,
        levelPackId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                        1usize,
                    >("GetLevelPack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLevelPack", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (levelPackId))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLevelPackForLevelId(
        &mut self,
        levelId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevelPack,
                        >,
                        1usize,
                    >("GetLevelPackForLevelId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLevelPackForLevelId", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (levelId))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadAllBeatmapLevelPacks(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("LoadAllBeatmapLevelPacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadAllBeatmapLevelPacks", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::BeatmapLevelDataVersion,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
                            >,
                        >,
                        3usize,
                    >("LoadBeatmapLevelDataAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadBeatmapLevelDataAsync", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::LoadBeatmapLevelDataResult,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (levelID, beatmapLevelDataVersion, cancellationToken),
                )?
        };
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
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ReloadAllBeatmapLevelPacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReloadAllBeatmapLevelPacks", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReloadCustomLevelPackCollectionAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapLevelsRepository,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Threading::CancellationToken),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevelsRepository,
                                >,
                            >,
                        >,
                        1usize,
                    >("ReloadCustomLevelPackCollectionAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReloadCustomLevelPackCollectionAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapLevelsRepository,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (cancellationToken))? };
        Ok(__cordl_ret.into())
    }
    pub fn SelectPacks(
        &mut self,
        include: crate::GlobalNamespace::PackDefinitionSO_Tags,
        exclude: crate::GlobalNamespace::PackDefinitionSO_Tags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::PackDefinitionSO_Tags,
                            crate::GlobalNamespace::PackDefinitionSO_Tags,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::PackDefinitionSO,
                                >,
                            >,
                        >,
                        2usize,
                    >("SelectPacks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SelectPacks", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (include, exclude))? };
        Ok(__cordl_ret.into())
    }
    pub fn _CreateBeatmapLevelPack_g__ShouldBeKeptIntact_31_2(
        desiredSensitivityFlag: crate::GlobalNamespace::PlayerSensitivityFlag,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::GlobalNamespace::PlayerSensitivityFlag,
                            crate::GlobalNamespace::PlayerSensitivityFlag,
                        ),
                        bool,
                        2usize,
                    >("<CreateBeatmapLevelPack>g__ShouldBeKeptIntact|31_2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<CreateBeatmapLevelPack>g__ShouldBeKeptIntact|31_2", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked((), (desiredSensitivityFlag, contentRating))?
        };
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
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapLevelsEntitlementModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLevelLoader,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IPlayerDataModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::BGLib::Polyglot::LocalizationModel,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IEnumerable_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::PackDefinitionSO,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        entitlementChecker,
                        beatmapLevelLoader,
                        playerDataModel,
                        localizationModel,
                        packDefinitions,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_newPackWasCreatedFromDefinitionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevelPack,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_newPackWasCreatedFromDefinitionEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_newPackWasCreatedFromDefinitionEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_entitlements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IEntitlementModel>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IEntitlementModel,
                        >,
                        0usize,
                    >("get_entitlements")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_entitlements", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEntitlementModel,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_packDefinitions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::PackDefinitionSO,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_packDefinitions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_packDefinitions", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PackDefinitionSO>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_newPackWasCreatedFromDefinitionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapLevelPack,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_newPackWasCreatedFromDefinitionEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_newPackWasCreatedFromDefinitionEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapLevelsModel")]
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
