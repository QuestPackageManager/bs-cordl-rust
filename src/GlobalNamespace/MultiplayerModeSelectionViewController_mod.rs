#[cfg(feature = "MultiplayerModeSelectionViewController+MenuButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerModeSelectionViewController_MenuButton {
    CreateServer = 1i32,
    GameBrowser = 3i32,
    JoinWithCode = 2i32,
    QuickPlay = 0i32,
}
#[cfg(feature = "MultiplayerModeSelectionViewController+MenuButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerModeSelectionViewController_MenuButton => ""
    ."MultiplayerModeSelectionViewController/MenuButton"
);
#[cfg(feature = "MultiplayerModeSelectionViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerModeSelectionViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _quickPlayButton: *mut crate::UnityEngine::UI::Button,
    pub _gameBrowserButton: *mut crate::UnityEngine::UI::Button,
    pub _joinWithCodeButton: *mut crate::UnityEngine::UI::Button,
    pub _createServerButton: *mut crate::UnityEngine::UI::Button,
    pub _maintenanceMessageText: *mut crate::TMPro::TextMeshProUGUI,
    pub _customServerEndPointText: *mut crate::TMPro::TextMeshProUGUI,
    pub _networkConfig: *mut INetworkConfig,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut MultiplayerModeSelectionViewController,
        crate::GlobalNamespace::MultiplayerModeSelectionViewController_MenuButton,
    >,
}
#[cfg(feature = "MultiplayerModeSelectionViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerModeSelectionViewController => ""
    ."MultiplayerModeSelectionViewController"
);
#[cfg(feature = "MultiplayerModeSelectionViewController")]
impl std::ops::Deref for MultiplayerModeSelectionViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerModeSelectionViewController")]
impl std::ops::DerefMut for MultiplayerModeSelectionViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerModeSelectionViewController")]
impl MultiplayerModeSelectionViewController {
    #[cfg(feature = "MultiplayerModeSelectionViewController+MenuButton")]
    pub type MenuButton = crate::GlobalNamespace::MultiplayerModeSelectionViewController_MenuButton;
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
    pub fn _DidActivate_b__11_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__11_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerModeSelectionViewController,
            crate::GlobalNamespace::MultiplayerModeSelectionViewController_MenuButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__11_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__11_1", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__11_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__11_3", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__11_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__11_2", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerModeSelectionViewController,
            crate::GlobalNamespace::MultiplayerModeSelectionViewController_MenuButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
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
    pub fn HandleMenuButton(
        &mut self,
        menuButton: crate::GlobalNamespace::MultiplayerModeSelectionViewController_MenuButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuButton", (menuButton))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        multiplayerStatusData: *mut MultiplayerStatusData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (multiplayerStatusData))?;
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
#[cfg(feature = "MultiplayerModeSelectionViewController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerModeSelectionViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
