#[cfg(feature = "SelectAvatarSystemCell")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectAvatarSystemCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _avatarSystemCollection: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    >,
    pub didSetAvatarSystemPreferredEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    >,
    pub didRequestEditOfAvatarEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    >,
    pub didRequestCreationOfAvatarEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
            >,
        >,
    >,
    pub _loadedAvatarSystemMetadata: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub _selectedAvatarSystemMetadata: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
    >,
    pub _loadedSelectionView: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarSelectionView,
    >,
    pub _activateViewOnEnable: bool,
}
#[cfg(feature = "SelectAvatarSystemCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SelectAvatarSystemCell => ""
    ."SelectAvatarSystemCell"
);
#[cfg(feature = "SelectAvatarSystemCell")]
impl std::ops::Deref for crate::GlobalNamespace::SelectAvatarSystemCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectAvatarSystemCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectAvatarSystemCell")]
impl crate::GlobalNamespace::SelectAvatarSystemCell {
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLoadedSelectionViewDidPressCreateButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressCreateButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLoadedSelectionViewDidPressEditButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressEditButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleLoadedSelectionViewDidPressPreferredButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleLoadedSelectionViewDidPressPreferredButton", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Load(
        &mut self,
        avatarSystemMetadata: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Load", (avatarSystemMetadata))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReloadIsCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadIsCreated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSelectedAvatarSystem(
        &mut self,
        avatarSystem: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSelectedAvatarSystem", (avatarSystem))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryActivateOnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryActivateOnEnable", ())?;
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
    pub fn add_didRequestCreationOfAvatarEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                >,
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
    pub fn remove_didRequestCreationOfAvatarEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::BeatSaber::AvatarCore::IAvatarSystemMetadata,
                >,
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
}
#[cfg(feature = "SelectAvatarSystemCell")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectAvatarSystemCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
