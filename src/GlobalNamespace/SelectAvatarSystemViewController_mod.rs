#[cfg(feature = "SelectAvatarSystemViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectAvatarSystemViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _tableView: *mut crate::HMUI::TableView,
    pub _continueButton: *mut crate::UnityEngine::UI::Button,
    pub _hoverHint: *mut crate::HMUI::HoverHint,
    pub _cellPrefab: *mut SelectAvatarSystemCell,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub _playerDataModel: *mut PlayerDataModel,
    pub didSetAvatarSystemPreferredEvent: *mut crate::System::Action_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub didRequestEditOfAvatarEvent: *mut crate::System::Action_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub didRequestCreationOfAvatarEvent: *mut crate::System::Action_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub didPressContinueButtonEvent: *mut crate::System::Action,
    pub _avatarSystems: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub _selectedAvatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
}
#[cfg(feature = "SelectAvatarSystemViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SelectAvatarSystemViewController => ""
    ."SelectAvatarSystemViewController"
);
#[cfg(feature = "SelectAvatarSystemViewController")]
impl std::ops::Deref for SelectAvatarSystemViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl std::ops::DerefMut for SelectAvatarSystemViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl SelectAvatarSystemViewController {
    pub const kCellIdentifier: &'static str = "cellPrefab";
    #[cfg(
        feature = "SelectAvatarSystemViewController+_ReloadContinueButtonInteractability_d__31"
    )]
    pub type _ReloadContinueButtonInteractability_d__31 = crate::GlobalNamespace::SelectAvatarSystemViewController__ReloadContinueButtonInteractability_d__31;
    pub fn CellForIdx(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForIdx", (tableView, idx))?;
        Ok(__cordl_ret)
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
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
    pub fn HandleLoadedSelectionViewDidPressCreateButton(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressCreateButton", (avatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLoadedSelectionViewDidPressEditButton(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressEditButton", (avatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn HandleLoadedSelectionViewDidPressPreferredButton(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressPreferredButton", (avatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn Prewarm(
        &mut self,
        avatarSystems: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
        selectedAvatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Prewarm", (avatarSystems, selectedAvatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn ReloadContinueButtonInteractability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadContinueButtonInteractability", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReloadCreateEditButtonOfAvatarSystem(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadCreateEditButtonOfAvatarSystem", (avatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn SetAllCellsActive(
        &mut self,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAllCellsActive", (active))?;
        Ok(__cordl_ret)
    }
    pub fn SetSelectedAvatarSystem(
        &mut self,
        selectedAvatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedAvatarSystem", (selectedAvatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn _DidActivate_b__26_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__26_0", ())?;
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
    pub fn add_didPressContinueButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didRequestCreationOfAvatarEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didRequestCreationOfAvatarEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didRequestEditOfAvatarEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didRequestEditOfAvatarEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSetAvatarSystemPreferredEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSetAvatarSystemPreferredEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_continueButtonVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_continueButtonVisible", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didPressContinueButtonEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didRequestCreationOfAvatarEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didRequestCreationOfAvatarEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didRequestEditOfAvatarEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didRequestEditOfAvatarEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSetAvatarSystemPreferredEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSetAvatarSystemPreferredEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_continueButtonVisible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_continueButtonVisible", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl quest_hook::libil2cpp::ObjectType for SelectAvatarSystemViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
