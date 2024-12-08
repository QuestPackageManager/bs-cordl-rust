#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarSystem {
    __cordl_parent: crate::BeatSaber::AvatarCore::AvatarSystem,
    pub _avatarDataModel: *mut crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _settings: *mut crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings,
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarAdapter::BeatAvatarSystem
    => "BeatSaber.BeatAvatarAdapter"."BeatAvatarSystem"
);
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystem")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystem {
    type Target = crate::BeatSaber::AvatarCore::AvatarSystem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystem")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystem")]
impl crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystem {
    #[cfg(
        feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystem+_CreateDefaultAvatarIfUserCreatedAvatarDoesNotExist_d__12"
    )]
    pub type _CreateDefaultAvatarIfUserCreatedAvatarDoesNotExist_d__12 = crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystem__CreateDefaultAvatarIfUserCreatedAvatarDoesNotExist_d__12;
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
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        model: *mut crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
        settings: *mut crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, model, settings))?;
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
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        model: *mut crate::BeatSaber::BeatAvatarSDK::AvatarDataModel,
        settings: *mut crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystemSettings,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, model, settings))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarAdapter+BeatAvatarSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarAdapter::BeatAvatarSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
