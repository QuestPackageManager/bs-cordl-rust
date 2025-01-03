#[cfg(feature = "SelectAvatarSystemViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectAvatarSystemViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
    pub _continueButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub _hoverHint: quest_hook::libil2cpp::Gc<crate::HMUI::HoverHint>,
    pub _cellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SelectAvatarSystemCell,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _avatarSystemCollection: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub didSetAvatarSystemPreferredEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata>,
    >,
    pub didRequestEditOfAvatarEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata>,
    >,
    pub didRequestCreationOfAvatarEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata>,
    >,
    pub didPressContinueButtonEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _avatarSystems: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    >,
    pub _selectedAvatarSystem: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
}
#[cfg(feature = "SelectAvatarSystemViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SelectAvatarSystemViewController => ""
    ."SelectAvatarSystemViewController"
);
#[cfg(feature = "SelectAvatarSystemViewController")]
impl std::ops::Deref for crate::GlobalNamespace::SelectAvatarSystemViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectAvatarSystemViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl crate::GlobalNamespace::SelectAvatarSystemViewController {
    pub const kCellIdentifier: &'static str = "cellPrefab";
    pub fn CellForIdx(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = __cordl_object
            .invoke("CellForIdx", (tableView, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellSize(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", (idx))?;
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
    pub fn HandleLoadedSelectionViewDidPressCreateButton(
        &mut self,
        avatarSystem: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressCreateButton", (avatarSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLoadedSelectionViewDidPressEditButton(
        &mut self,
        avatarSystem: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressEditButton", (avatarSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLoadedSelectionViewDidPressPreferredButton(
        &mut self,
        avatarSystem: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressPreferredButton", (avatarSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Prewarm(
        &mut self,
        avatarSystems: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
        selectedAvatarSystem: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Prewarm", (avatarSystems, selectedAvatarSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReloadContinueButtonInteractability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadContinueButtonInteractability", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReloadCreateEditButtonOfAvatarSystem(
        &mut self,
        avatarSystem: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadCreateEditButtonOfAvatarSystem", (avatarSystem))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetSelectedAvatarSystem(
        &mut self,
        selectedAvatarSystem: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedAvatarSystem", (selectedAvatarSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn _DidActivate_b__26_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<DidActivate>b__26_0", ())?;
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
    pub fn add_didPressContinueButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didRequestCreationOfAvatarEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didRequestCreationOfAvatarEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didRequestEditOfAvatarEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didRequestEditOfAvatarEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSetAvatarSystemPreferredEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSetAvatarSystemPreferredEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_continueButtonVisible(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_continueButtonVisible", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didPressContinueButtonEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didPressContinueButtonEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didRequestCreationOfAvatarEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didRequestCreationOfAvatarEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didRequestEditOfAvatarEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didRequestEditOfAvatarEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSetAvatarSystemPreferredEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSetAvatarSystemPreferredEvent", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectAvatarSystemViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl AsRef<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::SelectAvatarSystemViewController {
    fn as_ref(&self) -> &crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SelectAvatarSystemViewController")]
impl AsMut<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::SelectAvatarSystemViewController {
    fn as_mut(&mut self) -> &mut crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
