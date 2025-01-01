#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActivePlayerGameplayManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameSongController: *mut crate::GlobalNamespace::GameSongController,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    pub _multiplayerLevelEndActions: *mut crate::GlobalNamespace::IMultiplayerLevelEndActionsListener,
    pub _prepareLevelCompletionResults: *mut crate::GlobalNamespace::PrepareLevelCompletionResults,
    pub _gameEnergyCounter: *mut crate::GlobalNamespace::GameEnergyCounter,
    pub _inGameMenuController: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerInGameMenuController,
    pub _platformHelper: *mut crate::GlobalNamespace::IVRPlatformHelper,
    pub _disconnectHelper: *mut crate::GlobalNamespace::MultiplayerLocalPlayerDisconnectHelper,
    pub _saberManager: *mut crate::GlobalNamespace::SaberManager,
    pub _initData: *mut crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager_InitData,
    pub _levelFinishedOrQuit: bool,
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager => ""
    ."MultiplayerLocalActivePlayerGameplayManager"
);
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager")]
impl crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager {
    #[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager+InitData")]
    pub type InitData = crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager_InitData;
    pub fn HandleGameEnergyDidReach0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameEnergyDidReach0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleHmdUnmounted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleHmdUnmounted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInGameMenuControllerDidGiveUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInGameMenuControllerDidGiveUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInGameMenuControllerRequestsDisconnect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInGameMenuControllerRequestsDisconnect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusCaptured(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusCaptured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleInputFocusReleased(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleInputFocusReleased", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleSongDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongDidFinish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleVrFocusWasCapturedEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVrFocusWasCapturedEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnApplicationFocus(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (hasFocus))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformPlayerFail(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformPlayerFail", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PerformPlayerGivenUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PerformPlayerGivenUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActivePlayerGameplayManager_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub continueGameplayWith0Energy: bool,
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager_InitData => ""
    ."MultiplayerLocalActivePlayerGameplayManager/InitData"
);
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager+InitData")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager+InitData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager+InitData")]
impl crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager_InitData {
    pub fn New(
        continueGameplayWith0Energy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (continueGameplayWith0Energy))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        continueGameplayWith0Energy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (continueGameplayWith0Energy))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerGameplayManager+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalActivePlayerGameplayManager_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
