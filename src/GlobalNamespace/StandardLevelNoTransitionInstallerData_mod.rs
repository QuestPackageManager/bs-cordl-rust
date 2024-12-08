#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelNoTransitionInstallerData {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _beatmapLevel: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::GlobalNamespace::BeatmapLevelSO,
    >,
    pub _beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub _beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _colorScheme: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::GlobalNamespace::ColorSchemeSO,
    >,
    pub _environmentInfo: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut crate::GlobalNamespace::EnvironmentInfoSO,
    >,
    pub _gameplayModifiers: *mut crate::GlobalNamespace::GameplayModifiers,
    pub _playerSpecificSettings: *mut crate::GlobalNamespace::PlayerSpecificSettings,
    pub _practiceSettings: *mut crate::GlobalNamespace::PracticeSettings,
    pub _backButtonText: *mut crate::System::String,
    pub _useTestNoteCutSoundEffects: bool,
    pub _selectedBeatmapLevel: *mut crate::GlobalNamespace::BeatmapLevelSO,
    pub _selectedColorScheme: *mut crate::GlobalNamespace::ColorSchemeSO,
    pub _selectedEnvironmentInfo: *mut crate::GlobalNamespace::EnvironmentInfoSO,
}
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelNoTransitionInstallerData => ""
    ."StandardLevelNoTransitionInstallerData"
);
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelNoTransitionInstallerData {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandardLevelNoTransitionInstallerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl crate::GlobalNamespace::StandardLevelNoTransitionInstallerData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_backButtonText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_backButtonText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapCharacteristicSO = __cordl_object
            .invoke("get_beatmapCharacteristic", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapDifficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_beatmapDifficulty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapLevelSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapLevelSO = __cordl_object
            .invoke("get_beatmapLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ColorSchemeSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ColorSchemeSO = __cordl_object
            .invoke("get_colorScheme", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::EnvironmentInfoSO = __cordl_object
            .invoke("get_environmentInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::GameplayModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::GameplayModifiers = __cordl_object
            .invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::PlayerSpecificSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PlayerSpecificSettings = __cordl_object
            .invoke("get_playerSpecificSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_practiceSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::PracticeSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::PracticeSettings = __cordl_object
            .invoke("get_practiceSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_useTestNoteCutSoundEffects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useTestNoteCutSoundEffects", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_backButtonText(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_backButtonText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapCharacteristic(
        &mut self,
        value: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapCharacteristic", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapDifficulty(
        &mut self,
        value: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapDifficulty", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_gameplayModifiers(
        &mut self,
        value: *mut crate::GlobalNamespace::GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayModifiers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerSpecificSettings(
        &mut self,
        value: *mut crate::GlobalNamespace::PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerSpecificSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_practiceSettings(
        &mut self,
        value: *mut crate::GlobalNamespace::PracticeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_practiceSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_useTestNoteCutSoundEffects(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useTestNoteCutSoundEffects", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelNoTransitionInstallerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
