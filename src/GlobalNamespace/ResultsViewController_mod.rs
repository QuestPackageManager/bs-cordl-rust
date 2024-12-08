#[cfg(feature = "ResultsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct ResultsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _restartButton: *mut crate::UnityEngine::UI::Button,
    pub _continueButton: *mut crate::UnityEngine::UI::Button,
    pub _clearedPanel: *mut crate::UnityEngine::GameObject,
    pub _scoreText: *mut crate::TMPro::TextMeshProUGUI,
    pub _newHighScoreText: *mut crate::UnityEngine::GameObject,
    pub _rankText: *mut crate::TMPro::TextMeshProUGUI,
    pub _goodCutsPercentageText: *mut crate::TMPro::TextMeshProUGUI,
    pub _comboText: *mut crate::TMPro::TextMeshProUGUI,
    pub _clearedBannerGo: *mut crate::UnityEngine::GameObject,
    pub _failedBannerGo: *mut crate::UnityEngine::GameObject,
    pub _levelBar: *mut LevelBar,
    pub _levelClearedAudioClip: *mut crate::UnityEngine::AudioClip,
    pub _fireworksController: *mut FireworksController,
    pub _songPreviewPlayer: *mut SongPreviewPlayer,
    pub _menuDestinationRequest: *mut MenuDestination,
    pub continueButtonPressedEvent: *mut crate::System::Action_1<
        *mut ResultsViewController,
    >,
    pub restartButtonPressedEvent: *mut crate::System::Action_1<
        *mut ResultsViewController,
    >,
    pub _levelCompletionResults: *mut LevelCompletionResults,
    pub _transformedBeatmapData: *mut IReadonlyBeatmapData,
    pub _beatmapKey: BeatmapKey,
    pub _beatmapLevel: *mut BeatmapLevel,
    pub _startFireworksAfterDelayCoroutine: *mut crate::UnityEngine::Coroutine,
    pub _newHighScore: bool,
    pub _practice: bool,
}
#[cfg(feature = "ResultsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ResultsViewController => ""."ResultsViewController"
);
#[cfg(feature = "ResultsViewController")]
impl std::ops::Deref for ResultsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsViewController")]
impl std::ops::DerefMut for ResultsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ResultsViewController")]
impl ResultsViewController {
    #[cfg(feature = "ResultsViewController+_StartFireworksAfterDelay_d__33")]
    pub type _StartFireworksAfterDelay_d__33 = crate::GlobalNamespace::ResultsViewController__StartFireworksAfterDelay_d__33;
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMenuDestinationRequest(
        &mut self,
        menuDestination: *mut MenuDestination,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMenuDestinationRequest", (menuDestination))?;
        Ok(__cordl_ret)
    }
    pub fn SetDataToUI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDataToUI", ())?;
        Ok(__cordl_ret)
    }
    pub fn ContinueButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ContinueButtonPressed", ())?;
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
    pub fn StartFireworksAfterDelay(
        &mut self,
        delay: f32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("StartFireworksAfterDelay", (delay))?;
        Ok(__cordl_ret)
    }
    pub fn add_restartButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_restartButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        levelCompletionResults: *mut LevelCompletionResults,
        transformedBeatmapData: *mut IReadonlyBeatmapData,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
        practice: bool,
        newHighScore: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    levelCompletionResults,
                    transformedBeatmapData,
                    beatmapKey,
                    beatmapLevel,
                    practice,
                    newHighScore,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
        Ok(__cordl_ret)
    }
    pub fn remove_restartButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_restartButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_practice(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_practice", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_continueButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_continueButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_continueButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_continueButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn RestartButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestartButtonPressed", ())?;
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
#[cfg(feature = "ResultsViewController")]
impl quest_hook::libil2cpp::ObjectType for ResultsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
