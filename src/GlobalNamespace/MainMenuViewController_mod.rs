#[cfg(feature = "MainMenuViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct MainMenuViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _soloButton: *mut crate::UnityEngine::UI::Button,
    pub _partyButton: *mut crate::UnityEngine::UI::Button,
    pub _campaignButton: *mut crate::UnityEngine::UI::Button,
    pub _quitButton: *mut crate::UnityEngine::UI::Button,
    pub _howToPlayButton: *mut crate::UnityEngine::UI::Button,
    pub _beatmapEditorButton: *mut crate::UnityEngine::UI::Button,
    pub _multiplayerButton: *mut crate::UnityEngine::UI::Button,
    pub _optionsButton: *mut crate::UnityEngine::UI::Button,
    pub _musicPackPromoButton: *mut crate::UnityEngine::UI::Button,
    pub _musicPackPromoBanner: *mut MusicPackPromoBanner,
    pub _dlcPromoPanelModel: *mut DlcPromoPanelModel,
    pub _analyticsModel: *mut IAnalyticsModel,
    pub didFinishEvent: *mut crate::System::Action_2<
        *mut MainMenuViewController,
        crate::GlobalNamespace::MainMenuViewController_MenuButton,
    >,
    pub promoButtonWasPressedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
    >,
    pub _isLoadingPackPromoData: bool,
}
#[cfg(feature = "MainMenuViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MainMenuViewController => ""."MainMenuViewController"
);
#[cfg(feature = "MainMenuViewController")]
impl std::ops::Deref for MainMenuViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MainMenuViewController")]
impl std::ops::DerefMut for MainMenuViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MainMenuViewController")]
impl MainMenuViewController {
    #[cfg(feature = "MainMenuViewController+MenuButton")]
    pub type MenuButton = crate::GlobalNamespace::MainMenuViewController_MenuButton;
    #[cfg(feature = "MainMenuViewController+_LoadMusicPackPromoDataAsync_d__21")]
    pub type _LoadMusicPackPromoDataAsync_d__21 = crate::GlobalNamespace::MainMenuViewController__LoadMusicPackPromoDataAsync_d__21;
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
    pub fn HandleMenuButton(
        &mut self,
        menuButton: crate::GlobalNamespace::MainMenuViewController_MenuButton,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMenuButton", (menuButton))?;
        Ok(__cordl_ret)
    }
    pub fn LoadMusicPackPromoDataAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadMusicPackPromoDataAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn PackPromoButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PackPromoButtonWasPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_1", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_2", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_3", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_4", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_5", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_6(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_6", ())?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__20_7(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__20_7", ())?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MainMenuViewController,
            crate::GlobalNamespace::MainMenuViewController_MenuButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_promoButtonWasPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_promoButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MainMenuViewController,
            crate::GlobalNamespace::MainMenuViewController_MenuButton,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_promoButtonWasPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::DlcPromoPanelModel_PromoInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_promoButtonWasPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MainMenuViewController")]
impl quest_hook::libil2cpp::ObjectType for MainMenuViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MainMenuViewController+MenuButton")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuViewController_MenuButton {
    BeatmapEditor = 2i32,
    FloorAdjust = 4i32,
    HowToPlay = 8i32,
    Multiplayer = 6i32,
    Options = 7i32,
    Party = 1i32,
    Quit = 5i32,
    SoloCampaign = 3i32,
    SoloFreePlay = 0i32,
}
#[cfg(feature = "MainMenuViewController+MenuButton")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MainMenuViewController_MenuButton => ""
    ."MainMenuViewController/MenuButton"
);
