#[cfg(feature = "MissionResultsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionResultsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _failedBannerGo: *mut crate::UnityEngine::GameObject,
    pub _clearedBannerGo: *mut crate::UnityEngine::GameObject,
    pub _missionNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _songNameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _successIcon: *mut crate::UnityEngine::Sprite,
    pub _successIconGlow: *mut crate::UnityEngine::Sprite,
    pub _successColor: crate::UnityEngine::Color,
    pub _failIcon: *mut crate::UnityEngine::Sprite,
    pub _failIconGlow: *mut crate::UnityEngine::Sprite,
    pub _failColor: crate::UnityEngine::Color,
    pub _resultObjectiveListItemList: *mut crate::GlobalNamespace::ResultObjectiveListItemsList,
    pub _continueButton: *mut crate::UnityEngine::UI::Button,
    pub _retryButton: *mut crate::UnityEngine::UI::Button,
    pub _levelClearedAudioClip: *mut crate::UnityEngine::AudioClip,
    pub _fireworksController: *mut crate::GlobalNamespace::FireworksController,
    pub _songPreviewPlayer: *mut crate::GlobalNamespace::SongPreviewPlayer,
    pub continueButtonPressedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MissionResultsViewController,
    >,
    pub retryButtonPressedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MissionResultsViewController,
    >,
    pub _missionNode: *mut crate::GlobalNamespace::MissionNode,
    pub _missionCompletionResults: *mut crate::GlobalNamespace::MissionCompletionResults,
    pub _startFireworksAfterDelayCoroutine: *mut crate::UnityEngine::Coroutine,
}
#[cfg(feature = "MissionResultsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MissionResultsViewController =>
    ""."MissionResultsViewController"
);
#[cfg(feature = "MissionResultsViewController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionResultsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionResultsViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionResultsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionResultsViewController")]
impl crate::GlobalNamespace::MissionResultsViewController {
    #[cfg(feature = "MissionResultsViewController+_StartFireworksAfterDelay_d__28")]
    pub type _StartFireworksAfterDelay_d__28 = crate::GlobalNamespace::MissionResultsViewController__StartFireworksAfterDelay_d__28;
    #[cfg(feature = "MissionResultsViewController+__c__DisplayClass29_0")]
    pub type __c__DisplayClass29_0 = crate::GlobalNamespace::MissionResultsViewController___c__DisplayClass29_0;
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
    pub fn Init(
        &mut self,
        missionNode: *mut crate::GlobalNamespace::MissionNode,
        missionCompletionResults: *mut crate::GlobalNamespace::MissionCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (missionNode, missionCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RetryButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RetryButtonPressed", ())?;
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
    pub fn add_continueButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MissionResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_continueButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_retryButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MissionResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_retryButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_continueButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MissionResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_continueButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_retryButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MissionResultsViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_retryButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionResultsViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionResultsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
