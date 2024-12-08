#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarSystem {
    __cordl_parent: crate::System::Object,
    pub _typeIdentifier: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    pub _supportedOptionalAvatarDataTypes: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
        u32,
    >,
    pub _isFallbackSystem: bool,
    pub _selectionSortOrder: i32,
    pub _selectableByUser: bool,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarSystem =>
    "BeatSaber.AvatarCore"."AvatarSystem"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystem")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::AvatarSystem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystem")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::AvatarSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystem")]
impl crate::BeatSaber::AvatarCore::AvatarSystem {
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
    pub fn GetMultiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::MultiplayerAvatarData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::MultiplayerAvatarData,
        > = __cordl_object.invoke("GetMultiplayerAvatarsData", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn New(
        identifier: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
        isFallbackSystem: bool,
        selectableByUser: bool,
        selectionSortOrder: i32,
        supportedOptionalAvatarDataTypes: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            u32,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    identifier,
                    isFallbackSystem,
                    selectableByUser,
                    selectionSortOrder,
                    supportedOptionalAvatarDataTypes,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn __GetRandomizedMultiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::MultiplayerAvatarData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            crate::GlobalNamespace::MultiplayerAvatarData,
        > = __cordl_object.invoke("__GetRandomizedMultiplayerAvatarsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        identifier: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
        isFallbackSystem: bool,
        selectableByUser: bool,
        selectionSortOrder: i32,
        supportedOptionalAvatarDataTypes: *mut crate::System::Collections::Generic::IReadOnlyCollection_1<
            u32,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    identifier,
                    isFallbackSystem,
                    selectableByUser,
                    selectionSortOrder,
                    supportedOptionalAvatarDataTypes,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_avatarCreated(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<bool>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<bool> = __cordl_object
            .invoke("get_avatarCreated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isFallbackSystem(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFallbackSystem", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectableByUser(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_selectableByUser", ())?;
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
    pub fn get_typeIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatSaber::AvatarCore::AvatarSystemIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::AvatarCore::AvatarSystemIdentifier = __cordl_object
            .invoke("get_typeIdentifier", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarSystem")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::AvatarCore::AvatarSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
