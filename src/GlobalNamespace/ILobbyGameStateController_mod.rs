#[cfg(feature = "ILobbyGameStateController")]
#[repr(C)]
#[derive(Debug)]
pub struct ILobbyGameStateController {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ILobbyGameStateController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ILobbyGameStateController => ""
    ."ILobbyGameStateController"
);
#[cfg(feature = "ILobbyGameStateController")]
impl std::ops::Deref for ILobbyGameStateController {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ILobbyGameStateController")]
impl std::ops::DerefMut for ILobbyGameStateController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ILobbyGameStateController")]
impl ILobbyGameStateController {
    pub fn ClearDisconnectedState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearDisconnectedState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentLevelIfGameStarted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetCurrentLevelIfGameStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGameStateAndConfigurationAsync(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("GetGameStateAndConfigurationAsync", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn IsCloseToStartGame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsCloseToStartGame", ())?;
        Ok(__cordl_ret)
    }
    pub fn PredictCountdownEndTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PredictCountdownEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartListeningToGameStart(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListeningToGameStart", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_beforeSceneSwitchCallbackEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beforeSceneSwitchCallbackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_countdownCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_countdownCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_countdownStartedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_countdownStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_gameStartCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStartCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_gameStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_gameStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelDidGetDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelDidGetDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_levelFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            *mut MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_levelFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_lobbyDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_lobbyDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_lobbyStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerLobbyState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_lobbyStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_playerMissingEntitlementsChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerMissingEntitlementsChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectedLevelGameplaySetupDataChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectedLevelGameplaySetupDataChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_songStillDownloadingEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_songStillDownloadingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_startButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_1<CannotStartGameReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_startTimeChangedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_startTimeChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_cannotStartGameReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<CannotStartGameReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: CannotStartGameReason = __cordl_object
            .invoke("get_cannotStartGameReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_countdownEndTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_countdownEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_countdownStarted(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_countdownStarted", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_disconnectedReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<DisconnectedReason> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: DisconnectedReason = __cordl_object
            .invoke("get_disconnectedReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDisconnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDisconnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelStartInitiated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_levelStartInitiated", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_predictedCountdownEndTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("get_predictedCountdownEndTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedLevelGameplaySetupData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut ILevelGameplaySetupData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILevelGameplaySetupData = __cordl_object
            .invoke("get_selectedLevelGameplaySetupData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_startTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_startTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_state(&mut self) -> quest_hook::libil2cpp::Result<MultiplayerLobbyState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: MultiplayerLobbyState = __cordl_object.invoke("get_state", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_beforeSceneSwitchCallbackEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beforeSceneSwitchCallbackEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_countdownCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_countdownCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_countdownStartedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_countdownStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_gameStartCancelledEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStartCancelledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_gameStartedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_gameStartedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelDidGetDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action_1<DisconnectedReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelDidGetDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_levelFinishedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelScenesTransitionSetupDataSO,
            *mut MultiplayerResultsData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_levelFinishedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_lobbyDisconnectedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_lobbyDisconnectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_lobbyStateChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<MultiplayerLobbyState>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_lobbyStateChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerMissingEntitlementsChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut PlayersMissingEntitlementsNetSerializable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerMissingEntitlementsChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectedLevelGameplaySetupDataChangedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut ILevelGameplaySetupData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectedLevelGameplaySetupDataChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_songStillDownloadingEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_songStillDownloadingEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_startButtonEnabledEvent(
        &mut self,
        value: *mut crate::System::Action_1<CannotStartGameReason>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startButtonEnabledEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_startTimeChangedEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_startTimeChangedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_state(
        &mut self,
        value: MultiplayerLobbyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_state", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ILobbyGameStateController")]
impl quest_hook::libil2cpp::ObjectType for ILobbyGameStateController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
