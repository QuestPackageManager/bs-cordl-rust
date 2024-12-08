#[cfg(feature = "MissionLevelDetailViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionLevelDetailViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _gameplayModifiersModel: *mut crate::GlobalNamespace::GameplayModifiersModelSO,
    pub _playButton: *mut crate::UnityEngine::UI::Button,
    pub _levelBar: *mut crate::GlobalNamespace::LevelBar,
    pub _objectiveListItems: *mut crate::GlobalNamespace::ObjectiveListItemsList,
    pub _gameplayModifierInfoListItemsList: *mut crate::GlobalNamespace::GameplayModifierInfoListItemsList,
    pub _modifiersPanelGO: *mut crate::UnityEngine::GameObject,
    pub didPressPlayButtonEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MissionLevelDetailViewController,
    >,
    pub _missionNode: *mut crate::GlobalNamespace::MissionNode,
}
#[cfg(feature = "MissionLevelDetailViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionLevelDetailViewController => ""
    ."MissionLevelDetailViewController"
);
#[cfg(feature = "MissionLevelDetailViewController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionLevelDetailViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelDetailViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionLevelDetailViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionLevelDetailViewController")]
impl crate::GlobalNamespace::MissionLevelDetailViewController {
    #[cfg(feature = "MissionLevelDetailViewController+__c__DisplayClass14_0")]
    pub type __c__DisplayClass14_0 = crate::GlobalNamespace::MissionLevelDetailViewController___c__DisplayClass14_0;
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn Setup(
        &mut self,
        missionNode: *mut crate::GlobalNamespace::MissionNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (missionNode))?;
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
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MissionLevelDetailViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressPlayButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_missionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MissionNode = __cordl_object
            .invoke("get_missionNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressPlayButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MissionLevelDetailViewController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressPlayButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MissionLevelDetailViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionLevelDetailViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
