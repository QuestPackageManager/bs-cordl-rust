#[cfg(feature = "MultiplayerLevelFinishedController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelFinishedController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _levelEndActionsPublisher: *mut IMultiplayerLevelEndActionsPublisher,
    pub _rpcManager: *mut IGameplayRpcManager,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _beatmapBasicData: *mut BeatmapBasicData,
    pub allResultsCollectedEvent: *mut crate::System::Action_2<
        *mut MultiplayerLevelCompletionResults,
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        >,
    >,
    pub _otherPlayersCompletionResults: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut MultiplayerLevelCompletionResults,
    >,
    pub _localPlayerResults: *mut MultiplayerLevelCompletionResults,
    pub _gameFinishReported: bool,
    pub _sceneLoadTime: f32,
}
#[cfg(feature = "MultiplayerLevelFinishedController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLevelFinishedController => ""
    ."MultiplayerLevelFinishedController"
);
#[cfg(feature = "MultiplayerLevelFinishedController")]
impl std::ops::Deref for MultiplayerLevelFinishedController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelFinishedController")]
impl std::ops::DerefMut for MultiplayerLevelFinishedController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelFinishedController")]
impl MultiplayerLevelFinishedController {
    pub const kMinSceneDuration: f32 = 2f32;
    #[cfg(feature = "MultiplayerLevelFinishedController+_StartLevelFinished_d__20")]
    pub type _StartLevelFinished_d__20 = crate::GlobalNamespace::MultiplayerLevelFinishedController__StartLevelFinished_d__20;
    pub fn get_localPlayerResults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerLevelCompletionResults> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerLevelCompletionResults = __cordl_object
            .invoke("get_localPlayerResults", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerNetworkDidFailed(
        &mut self,
        levelCompletionResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerNetworkDidFailed", (levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn remove_allResultsCollectedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelCompletionResults,
            *mut crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::String,
                *mut MultiplayerLevelCompletionResults,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_allResultsCollectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_gameResultsReady(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_gameResultsReady", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_otherPlayersCompletionResults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        > = __cordl_object.invoke("get_otherPlayersCompletionResults", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleRpcLevelFinished(
        &mut self,
        userId: *mut crate::System::String,
        results: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleRpcLevelFinished", (userId, results))?;
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_allResultsCollectedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut MultiplayerLevelCompletionResults,
            *mut crate::System::Collections::Generic::Dictionary_2<
                *mut crate::System::String,
                *mut MultiplayerLevelCompletionResults,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_allResultsCollectedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn StartLevelFinished(
        &mut self,
        localPlayerResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("StartLevelFinished", (localPlayerResults))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerDidFinish(
        &mut self,
        levelCompletionResults: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDidFinish", (levelCompletionResults))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "MultiplayerLevelFinishedController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLevelFinishedController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
