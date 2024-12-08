#[cfg(feature = "CenterStageScreenController")]
#[repr(C)]
#[derive(Debug)]
pub struct CenterStageScreenController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _defaultMenuLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _lobbyLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _countdownMenuLightsPreset: *mut crate::GlobalNamespace::MenuLightsPresetSO,
    pub _beatmapSelectionView: *mut crate::GlobalNamespace::BeatmapSelectionView,
    pub _modifiersSelectionView: *mut crate::GlobalNamespace::ModifiersSelectionView,
    pub _countdownController: *mut crate::GlobalNamespace::CountdownController,
    pub _multiplayerLobbyCenterScreenLayoutAnimator: *mut crate::GlobalNamespace::MultiplayerLobbyCenterScreenLayoutAnimator,
    pub _lobbyGameStateController: *mut crate::GlobalNamespace::ILobbyGameStateController,
    pub _menuLightsManager: *mut crate::GlobalNamespace::MenuLightsManager,
    pub _countdownShown_k__BackingField: bool,
    pub _countdownEndTime: i64,
}
#[cfg(feature = "CenterStageScreenController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CenterStageScreenController =>
    ""."CenterStageScreenController"
);
#[cfg(feature = "CenterStageScreenController")]
impl std::ops::Deref for crate::GlobalNamespace::CenterStageScreenController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CenterStageScreenController")]
impl std::ops::DerefMut for crate::GlobalNamespace::CenterStageScreenController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CenterStageScreenController")]
impl crate::GlobalNamespace::CenterStageScreenController {
    pub fn HandleLobbyGameStateControllerSelectedLevelGameplaySetupDataChanged(
        &mut self,
        levelGameplaySetupData: *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleLobbyGameStateControllerSelectedLevelGameplaySetupDataChanged",
                (levelGameplaySetupData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", ())?;
        Ok(__cordl_ret)
    }
    pub fn HideCountdown(
        &mut self,
        instant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideCountdown", (instant))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetCountdownEndTime(
        &mut self,
        countdownEndTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCountdownEndTime", (countdownEndTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetNextGameplaySetupData(
        &mut self,
        levelGameplaySetupData: *mut crate::GlobalNamespace::ILevelGameplaySetupData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNextGameplaySetupData", (levelGameplaySetupData))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        showModifiers: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (showModifiers))?;
        Ok(__cordl_ret)
    }
    pub fn Show(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Show", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShowCountdown(
        &mut self,
        countdownEndTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowCountdown", (countdownEndTime))?;
        Ok(__cordl_ret)
    }
    pub fn ShowCountdownColorPreset(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowCountdownColorPreset", (animated))?;
        Ok(__cordl_ret)
    }
    pub fn ShowLobbyColorPreset(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowLobbyColorPreset", (animated))?;
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
    pub fn get_countdownShown(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_countdownShown", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_countdownShown(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_countdownShown", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "CenterStageScreenController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CenterStageScreenController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
