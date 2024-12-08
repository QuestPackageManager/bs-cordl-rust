#[cfg(feature = "LevelBar")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelBar {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _songArtworkImageView: *mut crate::HMUI::ImageView,
    pub _songNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _authorNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _showSongSubName: bool,
    pub _singleLineSongInfoContainer: *mut crate::UnityEngine::GameObject,
    pub _multiLineSongInfoContainer: *mut crate::UnityEngine::GameObject,
    pub _multiLineSongNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _multiLineAuthorNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _showDifficultyAndCharacteristic: bool,
    pub _difficultyText: *mut crate::TMPro::TextMeshProUGUI,
    pub _characteristicIconImageView: *mut crate::HMUI::ImageView,
    pub _useArtworkBackground: bool,
    pub _artworkBackgroundImage: *mut crate::HMUI::ImageView,
    pub _defaultArtworkImage: *mut crate::UnityEngine::Sprite,
    pub _beatmapLevelsModel: *mut crate::GlobalNamespace::BeatmapLevelsModel,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
}
#[cfg(feature = "LevelBar")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelBar => ""."LevelBar"
);
#[cfg(feature = "LevelBar")]
impl std::ops::Deref for crate::GlobalNamespace::LevelBar {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelBar")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelBar")]
impl crate::GlobalNamespace::LevelBar {
    #[cfg(feature = "LevelBar+_SetupData_d__20")]
    pub type _SetupData_d__20 = crate::GlobalNamespace::LevelBar__SetupData_d__20;
    #[cfg(feature = "LevelBar+_Setup_d__18")]
    pub type _Setup_d__18 = crate::GlobalNamespace::LevelBar__Setup_d__18;
    #[cfg(feature = "LevelBar+_Setup_d__19")]
    pub type _Setup_d__19 = crate::GlobalNamespace::LevelBar__Setup_d__19;
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
    pub fn SetupData(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "SetupData",
                (beatmapLevel, beatmapDifficulty, beatmapCharacteristic),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Setup_BeatmapKey0(
        &mut self,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn Setup_BeatmapLevel_BeatmapDifficulty_BeatmapCharacteristicSO1(
        &mut self,
        beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
        beatmapDifficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (beatmapLevel, beatmapDifficulty, beatmapCharacteristic))?;
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
    pub fn set_hide(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hide", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LevelBar")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LevelBar {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
