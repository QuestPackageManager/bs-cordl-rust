#[cfg(feature = "MissionLevelDetailViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelDetailViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _gameplayModifiersModel: *mut GameplayModifiersModelSO,
    pub _playButton: *mut crate::UnityEngine::UI::Button,
    pub _levelBar: *mut LevelBar,
    pub _objectiveListItems: *mut ObjectiveListItemsList,
    pub _gameplayModifierInfoListItemsList: *mut GameplayModifierInfoListItemsList,
    pub _modifiersPanelGO: *mut crate::UnityEngine::GameObject,
    pub didPressPlayButtonEvent: *mut crate::System::Action_1<
        *mut MissionLevelDetailViewController,
    >,
    pub _missionNode: *mut MissionNode,
}
#[cfg(feature = "MissionLevelDetailViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MissionLevelDetailViewController => ""
    ."MissionLevelDetailViewController"
);
#[cfg(feature = "MissionLevelDetailViewController")]
impl std::ops::Deref for MissionLevelDetailViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelDetailViewController")]
impl std::ops::DerefMut for MissionLevelDetailViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelDetailViewController")]
impl MissionLevelDetailViewController {
    #[cfg(feature = "MissionLevelDetailViewController+__c__DisplayClass14_0")]
    pub type __c__DisplayClass14_0 = crate::GlobalNamespace::MissionLevelDetailViewController___c__DisplayClass14_0;
    pub fn remove_didPressPlayButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MissionLevelDetailViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressPlayButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn PlayButtonPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayButtonPressed", ())?;
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
    pub fn Setup(
        &mut self,
        missionNode: *mut MissionNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (missionNode))?;
        Ok(__cordl_ret)
    }
    pub fn get_missionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MissionNode = __cordl_object
            .invoke("get_missionNode", ())?;
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
    pub fn add_didPressPlayButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MissionLevelDetailViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressPlayButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshContent", ())?;
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
#[cfg(feature = "MissionLevelDetailViewController")]
impl quest_hook::libil2cpp::ObjectType for MissionLevelDetailViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
