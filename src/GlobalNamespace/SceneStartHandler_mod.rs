#[cfg(feature = "SceneStartHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct SceneStartHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _gameplayRpcManager: *mut crate::GlobalNamespace::IGameplayRpcManager,
    pub _playersAtGameStartModel: *mut crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
    pub _readyPlayers: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _playersSpecificSettings: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut quest_hook::libil2cpp::Il2CppString,
        *mut crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
    >,
    pub _started: bool,
    pub _sessionGameId: *mut quest_hook::libil2cpp::Il2CppString,
    pub sceneSetupDidFinishEvent: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub sceneSetupDidReceiveTooLateEvent: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "SceneStartHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SceneStartHandler => ""
    ."SceneStartHandler"
);
#[cfg(feature = "SceneStartHandler")]
impl std::ops::Deref for crate::GlobalNamespace::SceneStartHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SceneStartHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::SceneStartHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SceneStartHandler")]
impl crate::GlobalNamespace::SceneStartHandler {
    pub fn AddPlayerSpecificSettingsToDictionary(
        &mut self,
        playerSpecificSettingsNetSerializable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddPlayerSpecificSettingsToDictionary",
                (playerSpecificSettingsNetSerializable),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayersSpecificSettingsAtGameStartData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        > = __cordl_object.invoke("CreatePlayersSpecificSettingsAtGameStartData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ForceStart", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSceneLoadStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSceneLoadStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleGetGameplaySceneReady(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGetGameplaySceneReady", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSetGameplaySceneReady(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerSpecificSettings: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSetGameplaySceneReady", (userId, playerSpecificSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSetGameplaySceneSyncFinished(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playersAtGameStart: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        >,
        sessionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSetGameplaySceneSyncFinished",
                (userId, playersAtGameStart, sessionId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSetPlayerDidConnectLate(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        failedUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playersAtGameStart: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerSpecificSettingsAtStartNetSerializable,
        >,
        sessionId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleSetPlayerDidConnectLate",
                (userId, failedUserId, playersAtGameStart, sessionId),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        playersAtGameStartModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, playersAtGameStartModel),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
        gameplayRpcManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IGameplayRpcManager,
        >,
        playersAtGameStartModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayersSpecificSettingsAtGameStartModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (multiplayerSessionManager, gameplayRpcManager, playersAtGameStartModel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sceneSetupDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sceneSetupDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_sceneSetupDidReceiveTooLateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_sceneSetupDidReceiveTooLateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sceneSetupDidFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sceneSetupDidFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_sceneSetupDidReceiveTooLateEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_sceneSetupDidReceiveTooLateEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SceneStartHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SceneStartHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
