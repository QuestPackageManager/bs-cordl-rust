#[cfg(feature = "BeatSaber+AvatarCore+IAvatarSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct IAvatarSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::IAvatarSystem =>
    "BeatSaber.AvatarCore"."IAvatarSystem"
);
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarSystem")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::IAvatarSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarSystem")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::IAvatarSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarSystem")]
impl crate::BeatSaber::AvatarCore::IAvatarSystem {
    pub fn InstantiateAvatar(
        &mut self,
        avatarDisplayContext: crate::BeatSaber::AvatarCore::AvatarDisplayContext,
        levelOfDetail: i32,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BeatSaber::AvatarCore::Avatar,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BeatSaber::AvatarCore::Avatar,
        > = __cordl_object
            .invoke(
                "InstantiateAvatar",
                (avatarDisplayContext, levelOfDetail, container),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateDefaultAvatarIfUserCreatedAvatarDoesNotExist(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("CreateDefaultAvatarIfUserCreatedAvatarDoesNotExist", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMultiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<MultiplayerAvatarData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            MultiplayerAvatarData,
        > = __cordl_object.invoke("GetMultiplayerAvatarsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeleteUserCreatedAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeleteUserCreatedAvatar", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectableByUser(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_selectableByUser", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAvatarSelectionView(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BeatSaber::AvatarCore::AvatarSelectionView,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BeatSaber::AvatarCore::AvatarSelectionView,
        > = __cordl_object.invoke("InstantiateAvatarSelectionView", (container))?;
        Ok(__cordl_ret)
    }
    pub fn get_isFallbackSystem(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFallbackSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn __GetRandomizedMultiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<MultiplayerAvatarData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            MultiplayerAvatarData,
        > = __cordl_object.invoke("__GetRandomizedMultiplayerAvatarsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionSortOrder(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectionSortOrder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_supportedOptionalAvatarDataTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyCollection_1<u32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            u32,
        > = __cordl_object.invoke("get_supportedOptionalAvatarDataTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn InstantiateAvatarEditorUI(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::BeatSaber::AvatarCore::AvatarEditorFlowCoordinator,
        > = __cordl_object.invoke("InstantiateAvatarEditorUI", (container))?;
        Ok(__cordl_ret)
    }
    pub fn GetMultiplayerAvatarOptionalDataProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider = __cordl_object
            .invoke("GetMultiplayerAvatarOptionalDataProvider", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::AvatarCore::IAvatarSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
