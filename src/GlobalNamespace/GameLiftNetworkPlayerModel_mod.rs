#[cfg(feature = "GameLiftNetworkPlayerModel")]
#[repr(C)]
#[derive(Debug)]
pub struct GameLiftNetworkPlayerModel {
    __cordl_parent: NetworkPlayerModel_1<*mut GameLiftConnectionManager>,
    pub _gameLiftPlayerSessionProvider: *mut crate::BGNet::Core::GameLift::IGameLiftPlayerSessionProvider,
    pub _cachedConnectToServerParams: *mut crate::GlobalNamespace::GameLiftConnectionManager_ConnectToServerParams,
    pub _cachedStartClientParams: *mut crate::GlobalNamespace::GameLiftConnectionManager_StartClientParams,
}
#[cfg(feature = "GameLiftNetworkPlayerModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameLiftNetworkPlayerModel => ""
    ."GameLiftNetworkPlayerModel"
);
#[cfg(feature = "GameLiftNetworkPlayerModel")]
impl std::ops::Deref for GameLiftNetworkPlayerModel {
    type Target = NetworkPlayerModel_1<*mut GameLiftConnectionManager>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftNetworkPlayerModel")]
impl std::ops::DerefMut for GameLiftNetworkPlayerModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameLiftNetworkPlayerModel")]
impl GameLiftNetworkPlayerModel {
    pub fn GetConnectToServerParams(
        &mut self,
        selectionMask: BeatmapLevelSelectionMask,
        configuration: GameplayServerConfiguration,
        secret: *mut crate::System::String,
        code: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut IConnectionInitParams_1<*mut GameLiftConnectionManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IConnectionInitParams_1<*mut GameLiftConnectionManager> = __cordl_object
            .invoke(
                "GetConnectToServerParams",
                (selectionMask, configuration, secret, code),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetStartClientParams(
        &mut self,
        selectionMask: BeatmapLevelSelectionMask,
        configuration: GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<
        *mut IConnectionInitParams_1<*mut GameLiftConnectionManager>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IConnectionInitParams_1<*mut GameLiftConnectionManager> = __cordl_object
            .invoke("GetStartClientParams", (selectionMask, configuration))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshPublicServers(
        &mut self,
        localSelectionMask: BeatmapLevelSelectionMask,
        localConfiguration: GameplayServerConfiguration,
        onSuccess: *mut crate::System::Action_1<
            *mut crate::System::Collections::Generic::IReadOnlyList_1<PublicServerInfo>,
        >,
        onFailure: *mut crate::System::Action_1<ConnectionFailedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RefreshPublicServers",
                (localSelectionMask, localConfiguration, onSuccess, onFailure),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn get_code(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_code", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_configuration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<GameplayServerConfiguration> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: GameplayServerConfiguration = __cordl_object
            .invoke("get_configuration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_partyOwnerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_partyOwnerId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_secret(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_secret", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectionMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<BeatmapLevelSelectionMask> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapLevelSelectionMask = __cordl_object
            .invoke("get_selectionMask", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameLiftNetworkPlayerModel")]
impl quest_hook::libil2cpp::ObjectType for GameLiftNetworkPlayerModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
