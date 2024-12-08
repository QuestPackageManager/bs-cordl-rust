#[cfg(feature = "SelectAvatarSystemCell")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectAvatarSystemCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub didSetAvatarSystemPreferredEvent: *mut crate::System::Action_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub didRequestEditOfAvatarEvent: *mut crate::System::Action_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub didRequestCreationOfAvatarEvent: *mut crate::System::Action_1<
        *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub _loadedAvatarSystemMetadata: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    pub _selectedAvatarSystemMetadata: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    pub _loadedSelectionView: *mut crate::BeatSaber::AvatarCore::AvatarSelectionView,
    pub _activateViewOnEnable: bool,
}
#[cfg(feature = "SelectAvatarSystemCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SelectAvatarSystemCell => ""."SelectAvatarSystemCell"
);
#[cfg(feature = "SelectAvatarSystemCell")]
impl std::ops::Deref for SelectAvatarSystemCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemCell")]
impl std::ops::DerefMut for SelectAvatarSystemCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemCell")]
impl SelectAvatarSystemCell {
    #[cfg(feature = "SelectAvatarSystemCell+_ReloadIsCreated_d__17")]
    pub type _ReloadIsCreated_d__17 = crate::GlobalNamespace::SelectAvatarSystemCell__ReloadIsCreated_d__17;
    #[cfg(feature = "SelectAvatarSystemCell+_Load_d__14")]
    pub type _Load_d__14 = crate::GlobalNamespace::SelectAvatarSystemCell__Load_d__14;
    pub fn TryActivateOnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryActivateOnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
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
    pub fn SetSelectedAvatarSystem(
        &mut self,
        avatarSystem: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedAvatarSystem", (avatarSystem))?;
        Ok(__cordl_ret)
    }
    pub fn ReloadIsCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadIsCreated", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLoadedSelectionViewDidPressCreateButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressCreateButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLoadedSelectionViewDidPressEditButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressEditButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn Load(
        &mut self,
        avatarSystemMetadata: *mut crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Load", (avatarSystemMetadata))?;
        Ok(__cordl_ret)
    }
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleLoadedSelectionViewDidPressPreferredButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressPreferredButton", ())?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SelectAvatarSystemCell")]
impl quest_hook::libil2cpp::ObjectType for SelectAvatarSystemCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
