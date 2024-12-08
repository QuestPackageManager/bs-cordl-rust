#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelNoTransitionInstallerData {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _beatmapLevel: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut BeatmapLevelSO,
    >,
    pub _beatmapCharacteristic: *mut BeatmapCharacteristicSO,
    pub _beatmapDifficulty: BeatmapDifficulty,
    pub _colorScheme: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut ColorSchemeSO,
    >,
    pub _environmentInfo: *mut crate::UnityEngine::AddressableAssets::AssetReferenceT_1<
        *mut EnvironmentInfoSO,
    >,
    pub _gameplayModifiers: *mut GameplayModifiers,
    pub _playerSpecificSettings: *mut PlayerSpecificSettings,
    pub _practiceSettings: *mut PracticeSettings,
    pub _backButtonText: *mut crate::System::String,
    pub _useTestNoteCutSoundEffects: bool,
    pub _selectedBeatmapLevel: *mut BeatmapLevelSO,
    pub _selectedColorScheme: *mut ColorSchemeSO,
    pub _selectedEnvironmentInfo: *mut EnvironmentInfoSO,
}
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for StandardLevelNoTransitionInstallerData => ""
    ."StandardLevelNoTransitionInstallerData"
);
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl std::ops::Deref for StandardLevelNoTransitionInstallerData {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl std::ops::DerefMut for StandardLevelNoTransitionInstallerData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl StandardLevelNoTransitionInstallerData {
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
    pub fn get_colorScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ColorSchemeSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ColorSchemeSO = __cordl_object
            .invoke("get_colorScheme", ())?;
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
    pub fn get_practiceSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PracticeSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PracticeSettings = __cordl_object
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
    pub fn set_playerSpecificSettings(
        &mut self,
        value: *mut PlayerSpecificSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerSpecificSettings", (value))?;
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
    pub fn set_beatmapCharacteristic(
        &mut self,
        value: *mut BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapCharacteristic", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_gameplayModifiers(
        &mut self,
        value: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayModifiers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapDifficulty(
        &mut self,
        value: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapDifficulty", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_playerSpecificSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut PlayerSpecificSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut PlayerSpecificSettings = __cordl_object
            .invoke("get_playerSpecificSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_environmentInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut EnvironmentInfoSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut EnvironmentInfoSO = __cordl_object
            .invoke("get_environmentInfo", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn get_beatmapLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelSO> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelSO = __cordl_object
            .invoke("get_beatmapLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_practiceSettings(
        &mut self,
        value: *mut PracticeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_practiceSettings", (value))?;
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
#[cfg(feature = "StandardLevelNoTransitionInstallerData")]
impl quest_hook::libil2cpp::ObjectType for StandardLevelNoTransitionInstallerData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
