#[cfg(feature = "MissionSelectionNavigationController")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionSelectionNavigationController {
    __cordl_parent: crate::HMUI::NavigationController,
    pub _missionSelectionMapViewController: *mut crate::GlobalNamespace::MissionSelectionMapViewController,
    pub _missionLevelDetailViewController: *mut crate::GlobalNamespace::MissionLevelDetailViewController,
    pub didPressPlayButtonEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MissionSelectionNavigationController,
    >,
}
#[cfg(feature = "MissionSelectionNavigationController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MissionSelectionNavigationController => ""
    ."MissionSelectionNavigationController"
);
#[cfg(feature = "MissionSelectionNavigationController")]
impl std::ops::Deref for crate::GlobalNamespace::MissionSelectionNavigationController {
    type Target = crate::HMUI::NavigationController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionSelectionNavigationController")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MissionSelectionNavigationController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionSelectionNavigationController")]
impl crate::GlobalNamespace::MissionSelectionNavigationController {
    #[cfg(feature = "MissionSelectionNavigationController+__c__DisplayClass11_0")]
    pub type __c__DisplayClass11_0 = crate::GlobalNamespace::MissionSelectionNavigationController___c__DisplayClass11_0;
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
    pub fn HandleMissionLevelDetailViewControllerDidPressPlayButton(
        &mut self,
        viewController: *mut crate::GlobalNamespace::MissionLevelDetailViewController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionLevelDetailViewControllerDidPressPlayButton",
                (viewController),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleMissionSelectionMapViewControllerDidSelectMissionLevel(
        &mut self,
        viewController: *mut crate::GlobalNamespace::MissionSelectionMapViewController,
        _missionNode: *mut crate::GlobalNamespace::MissionNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleMissionSelectionMapViewControllerDidSelectMissionLevel",
                (viewController, _missionNode),
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
    pub fn PresentMissionClearedIfNeeded(
        &mut self,
        finishedCallback: *mut crate::System::Action_1<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PresentMissionClearedIfNeeded", (finishedCallback))?;
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
            *mut crate::GlobalNamespace::MissionSelectionNavigationController,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressPlayButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedMissionNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::MissionNode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MissionNode = __cordl_object
            .invoke("get_selectedMissionNode", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressPlayButtonEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MissionSelectionNavigationController,
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
#[cfg(feature = "MissionSelectionNavigationController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionSelectionNavigationController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
