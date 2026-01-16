#[cfg(feature = "cordl_class_MultiplayerLevelScenesTransitionSetupDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelScenesTransitionSetupDataSO {
    __cordl_parent: crate::GlobalNamespace::LevelScenesTransitionSetupDataSO,
    pub didFinishEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerResultsData>,
        >,
    >,
    pub didDisconnectEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
            >,
            crate::GlobalNamespace::DisconnectedReason,
        >,
    >,
    pub _gameMode_k__BackingField: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _beatmapKey_k__BackingField: crate::GlobalNamespace::BeatmapKey,
    pub _beatmapLevel_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    pub _usingOverrideColorScheme_k__BackingField: bool,
    pub _colorScheme_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    pub _beatmapLevelData_k__BackingField:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
    pub _environmentInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentInfoSO>,
}
#[cfg(feature = "cordl_class_MultiplayerLevelScenesTransitionSetupDataSO")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerLevelScenesTransitionSetupDataSO";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO {
    type Target = crate::GlobalNamespace::LevelScenesTransitionSetupDataSO;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelScenesTransitionSetupDataSO")]
impl crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO {
    pub fn Finish(
        &mut self,
        resultsData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerResultsData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::MultiplayerResultsData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Finish")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Finish",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (resultsData))? };
        Ok(__cordl_ret.into())
    }
    pub fn FinishWithDisconnect(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::DisconnectedReason),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FinishWithDisconnect")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FinishWithDisconnect", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (disconnectedReason))? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        gameMode: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        beatmapLevelData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
        overrideColorScheme: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
        gameplayModifiers: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettings,
        >,
        environmentsListModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::EnvironmentsListModel,
        >,
        practiceSettings: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
        audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AudioClipAsyncLoader,
        >,
        settingsManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
        beatmapDataLoader: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataLoader>,
        gameplayAdditionalInformation: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayAdditionalInformation,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameplayModifiers>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerSpecificSettings>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::EnvironmentsListModel>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PracticeSettings>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioClipAsyncLoader>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::SettingsManager>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataLoader>,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::GameplayAdditionalInformation,
                        >,
                    ), quest_hook::libil2cpp::Void, 13usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Init",
                            13usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    gameMode,
                    beatmapKey,
                    beatmapLevel,
                    beatmapLevelData,
                    overrideColorScheme,
                    gameplayModifiers,
                    playerSpecificSettings,
                    environmentsListModel,
                    practiceSettings,
                    audioClipAsyncLoader,
                    settingsManager,
                    beatmapDataLoader,
                    gameplayAdditionalInformation,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitAndSetupScenes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("InitAndSetupScenes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "InitAndSetupScenes",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn add_didDisconnectEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                >,
                crate::GlobalNamespace::DisconnectedReason,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                            >,
                            crate::GlobalNamespace::DisconnectedReason,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_didDisconnectEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_didDisconnectEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerResultsData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerResultsData,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_didFinishEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_didFinishEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::BeatmapKey, 0usize>("get_beatmapKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_beatmapKey",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                        0usize,
                    >("get_beatmapLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_beatmapLevel", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapLevelData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IBeatmapLevelData,
                        >,
                        0usize,
                    >("get_beatmapLevelData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_beatmapLevelData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                        0usize,
                    >("get_colorScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_colorScheme", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_gameMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_gameMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_gameMode", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_usingOverrideColorScheme(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_usingOverrideColorScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_usingOverrideColorScheme",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didDisconnectEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                >,
                crate::GlobalNamespace::DisconnectedReason,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                            >,
                            crate::GlobalNamespace::DisconnectedReason,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_didDisconnectEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_didDisconnectEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerResultsData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_2<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::MultiplayerResultsData,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_didFinishEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_didFinishEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapKey(
        &mut self,
        value: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::BeatmapKey),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_beatmapKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_beatmapKey", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapLevel(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapLevel,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_beatmapLevel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_beatmapLevel", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_beatmapLevelData(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IBeatmapLevelData,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_beatmapLevelData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_beatmapLevelData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_colorScheme(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_colorScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_colorScheme", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_gameMode(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_gameMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_gameMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn set_usingOverrideColorScheme(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_usingOverrideColorScheme",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_usingOverrideColorScheme",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_MultiplayerLevelScenesTransitionSetupDataSO")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::MultiplayerLevelScenesTransitionSetupDataSO
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
