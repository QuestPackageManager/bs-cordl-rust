#[cfg(feature = "MultiplayerResultsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _levelClearedGO: *mut crate::UnityEngine::GameObject,
    pub _levelFailedGO: *mut crate::UnityEngine::GameObject,
    pub _levelResultsGO: *mut crate::UnityEngine::GameObject,
    pub _levelBar: *mut LevelBar,
    pub _resultsTableView: *mut ResultsTableView,
    pub _backToLobbyButton: *mut crate::UnityEngine::UI::Button,
    pub _backToMenuButton: *mut crate::UnityEngine::UI::Button,
    pub backToLobbyPressedEvent: *mut crate::System::Action_1<
        *mut MultiplayerResultsViewController,
    >,
    pub backToMenuPressedEvent: *mut crate::System::Action_1<
        *mut MultiplayerResultsViewController,
    >,
}
#[cfg(feature = "MultiplayerResultsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerResultsViewController => ""
    ."MultiplayerResultsViewController"
);
#[cfg(feature = "MultiplayerResultsViewController")]
impl std::ops::Deref for MultiplayerResultsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsViewController")]
impl std::ops::DerefMut for MultiplayerResultsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsViewController")]
impl MultiplayerResultsViewController {
    pub fn BackToLobbyPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackToLobbyPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn BackToMenuPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BackToMenuPressed", ())?;
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
    pub fn Init(
        &mut self,
        multiplayerResultsData: *mut MultiplayerResultsData,
        beatmapKey: BeatmapKey,
        showBackToLobbyButton: bool,
        showBackToMenuButton: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Init",
                (
                    multiplayerResultsData,
                    beatmapKey,
                    showBackToLobbyButton,
                    showBackToMenuButton,
                ),
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
    pub fn add_backToLobbyPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_backToLobbyPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_backToMenuPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_backToMenuPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_backToLobbyPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_backToLobbyPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_backToMenuPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerResultsViewController>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_backToMenuPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerResultsViewController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerResultsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
