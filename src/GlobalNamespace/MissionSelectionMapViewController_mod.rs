#[cfg(feature = "MissionSelectionMapViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionSelectionMapViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _mapScrollView: quest_hook::libil2cpp::Gc<crate::HMUI::ScrollView>,
    pub _missionNodeSelectionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionNodeSelectionManager,
    >,
    pub _missionMapAnimationController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionMapAnimationController,
    >,
    pub _songPreviewPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SongPreviewPlayer,
    >,
    pub _perceivedLoudnessPerLevelModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PerceivedLoudnessPerLevelModel,
    >,
    pub _audioClipAsyncLoader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioClipAsyncLoader,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub didSelectMissionLevelEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::MissionSelectionMapViewController,
            *mut crate::GlobalNamespace::MissionNode,
        >,
    >,
    pub _selectedMissionNode: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MissionNode,
    >,
}
#[cfg(feature = "MissionSelectionMapViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionSelectionMapViewController => ""
    ."MissionSelectionMapViewController"
);
#[cfg(feature = "MissionSelectionMapViewController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionSelectionMapViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionSelectionMapViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionSelectionMapViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionSelectionMapViewController")]
impl crate::GlobalNamespace::MissionSelectionMapViewController {
    pub fn DeselectSelectedNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeselectSelectedNode", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleMissionNodeSelectionManagerDidSelectMissionNode(
        &mut self,
        missionNodeVisualController: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MissionNodeVisualController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionNodeSelectionManagerDidSelectMissionNode",
                (missionNodeVisualController),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ShowMissionClearedAnimation(
        &mut self,
        finishCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowMissionClearedAnimation", (finishCallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SongPlayerCrossfadeToLevelAsync(
        &mut self,
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SongPlayerCrossfadeToLevelAsync", (level))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectMissionLevelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::MissionSelectionMapViewController,
                *mut crate::GlobalNamespace::MissionNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectMissionLevelEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_animatedUpdateIsRequired(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_animatedUpdateIsRequired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectMissionLevelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::MissionSelectionMapViewController,
                *mut crate::GlobalNamespace::MissionNode,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectMissionLevelEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionSelectionMapViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionSelectionMapViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
