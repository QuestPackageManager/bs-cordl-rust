#[cfg(feature = "MissionSelectionMapViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionSelectionMapViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _mapScrollView: *mut crate::HMUI::ScrollView,
    pub _missionNodeSelectionManager: *mut MissionNodeSelectionManager,
    pub _missionMapAnimationController: *mut MissionMapAnimationController,
    pub _songPreviewPlayer: *mut SongPreviewPlayer,
    pub _perceivedLoudnessPerLevelModel: *mut PerceivedLoudnessPerLevelModel,
    pub _audioClipAsyncLoader: *mut AudioClipAsyncLoader,
    pub _beatmapLevelsModel: *mut BeatmapLevelsModel,
    pub didSelectMissionLevelEvent: *mut crate::System::Action_2<
        *mut MissionSelectionMapViewController,
        *mut MissionNode,
    >,
    pub _selectedMissionNode: *mut MissionNode,
}
#[cfg(feature = "MissionSelectionMapViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionSelectionMapViewController => ""
    ."MissionSelectionMapViewController"
);
#[cfg(feature = "MissionSelectionMapViewController")]
impl std::ops::Deref for MissionSelectionMapViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionSelectionMapViewController")]
impl std::ops::DerefMut for MissionSelectionMapViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionSelectionMapViewController")]
impl MissionSelectionMapViewController {
    #[cfg(feature = "MissionSelectionMapViewController+__c__DisplayClass16_0")]
    pub type __c__DisplayClass16_0 = crate::GlobalNamespace::MissionSelectionMapViewController___c__DisplayClass16_0;
    #[cfg(
        feature = "MissionSelectionMapViewController+_SongPlayerCrossfadeToLevelAsync_d__16"
    )]
    pub type _SongPlayerCrossfadeToLevelAsync_d__16 = crate::GlobalNamespace::MissionSelectionMapViewController__SongPlayerCrossfadeToLevelAsync_d__16;
    pub fn DeselectSelectedNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeselectSelectedNode", ())?;
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
    pub fn HandleMissionNodeSelectionManagerDidSelectMissionNode(
        &mut self,
        missionNodeVisualController: *mut MissionNodeVisualController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionNodeSelectionManagerDidSelectMissionNode",
                (missionNodeVisualController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ShowMissionClearedAnimation(
        &mut self,
        finishCallback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowMissionClearedAnimation", (finishCallback))?;
        Ok(__cordl_ret)
    }
    pub fn SongPlayerCrossfadeToLevelAsync(
        &mut self,
        level: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SongPlayerCrossfadeToLevelAsync", (level))?;
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
    pub fn add_didSelectMissionLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MissionSelectionMapViewController,
            *mut MissionNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectMissionLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_animatedUpdateIsRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_animatedUpdateIsRequired", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectMissionLevelEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MissionSelectionMapViewController,
            *mut MissionNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectMissionLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionSelectionMapViewController")]
impl quest_hook::libil2cpp::ObjectType for MissionSelectionMapViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
