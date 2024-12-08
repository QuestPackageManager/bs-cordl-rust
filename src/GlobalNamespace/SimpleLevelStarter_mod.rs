#[cfg(feature = "SimpleLevelStarter")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleLevelStarter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _beatmapLevel: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::GlobalNamespace::BeatmapLevelSO,
    >,
    pub _beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub _beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _useTestNoteCutSoundEffects: bool,
    pub _overrideStrobeFilterSettingsToAllEffects: bool,
    pub _recordingTextAsset: *mut crate::UnityEngine::TextAsset,
    pub _prefabBindings: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Component,
    >,
    pub _forceOverrideEnvironment: bool,
    pub _button: *mut crate::UnityEngine::UI::Button,
    pub _menuTransitionsHelper: *mut crate::GlobalNamespace::MenuTransitionsHelper,
    pub _gameScenesManager: *mut crate::GlobalNamespace::GameScenesManager,
    pub _playerDataModel: *mut crate::GlobalNamespace::PlayerDataModel,
    pub _environmentsListModel: *mut crate::GlobalNamespace::EnvironmentsListModel,
    pub _buttonBinder: *mut crate::HMUI::ButtonBinder,
    pub _gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
}
#[cfg(feature = "SimpleLevelStarter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SimpleLevelStarter => ""
    ."SimpleLevelStarter"
);
#[cfg(feature = "SimpleLevelStarter")]
impl std::ops::Deref for crate::GlobalNamespace::SimpleLevelStarter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SimpleLevelStarter")]
impl std::ops::DerefMut for crate::GlobalNamespace::SimpleLevelStarter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SimpleLevelStarter")]
impl crate::GlobalNamespace::SimpleLevelStarter {
    #[cfg(feature = "SimpleLevelStarter+_StartLevel_d__17")]
    pub type _StartLevel_d__17 = crate::GlobalNamespace::SimpleLevelStarter__StartLevel_d__17;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn ButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ButtonPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLevelDidFinish(
        &mut self,
        standardLevelSceneSetupData: *mut crate::GlobalNamespace::StandardLevelScenesTransitionSetupDataSO,
        levelCompletionResults: *mut crate::GlobalNamespace::LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLevelDidFinish",
                (standardLevelSceneSetupData, levelCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InstallEarlyBindings(
        &mut self,
        scenesTransitionSetupData: *mut crate::GlobalNamespace::ScenesTransitionSetupDataSO,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallEarlyBindings", (scenesTransitionSetupData, container))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("StartLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn _StartLevel_g__AfterSceneSwitchCallback_17_0(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<StartLevel>g__AfterSceneSwitchCallback|17_0", (container))?;
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
}
#[cfg(feature = "SimpleLevelStarter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SimpleLevelStarter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
