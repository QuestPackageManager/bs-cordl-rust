#[cfg(feature = "MultiplayerLocalActivePlayerFacade+Factory")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActivePlayerFacade_Factory {
    __cordl_parent: crate::Zenject::PlaceholderFactory_2<
        MultiplayerPlayerStartState,
        *mut MultiplayerLocalActivePlayerFacade,
    >,
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade+Factory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory => ""
    ."MultiplayerLocalActivePlayerFacade/Factory"
);
#[cfg(feature = "MultiplayerLocalActivePlayerFacade+Factory")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory {
    type Target = crate::Zenject::PlaceholderFactory_2<
        MultiplayerPlayerStartState,
        *mut MultiplayerLocalActivePlayerFacade,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade+Factory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade+Factory")]
impl crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade+Factory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActivePlayerFacade {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _activeOnlyGameObjects: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _outroAnimator: *mut crate::UnityEngine::GameObject,
    pub _songController: *mut IStartSeekSongController,
    pub _introAnimator: *mut MultiplayerLocalActivePlayerIntroAnimator,
    pub _gameSongController: *mut GameSongController,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
    pub _prepareLevelCompletionResults: *mut PrepareLevelCompletionResults,
    pub _beatmapCallbacksUpdater: *mut BeatmapCallbacksUpdater,
    pub playerDidFinishEvent: *mut crate::System::Action_1<
        *mut MultiplayerLevelCompletionResults,
    >,
    pub playerNetworkDidFailedEvent: *mut crate::System::Action_1<
        *mut MultiplayerLevelCompletionResults,
    >,
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLocalActivePlayerFacade => ""
    ."MultiplayerLocalActivePlayerFacade"
);
#[cfg(feature = "MultiplayerLocalActivePlayerFacade")]
impl std::ops::Deref for MultiplayerLocalActivePlayerFacade {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade")]
impl std::ops::DerefMut for MultiplayerLocalActivePlayerFacade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade")]
impl MultiplayerLocalActivePlayerFacade {
    #[cfg(feature = "MultiplayerLocalActivePlayerFacade+Factory")]
    pub type Factory = crate::GlobalNamespace::MultiplayerLocalActivePlayerFacade_Factory;
    pub fn remove_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ReportPlayerDidFinish(
        &mut self,
        results: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportPlayerDidFinish", (results))?;
        Ok(__cordl_ret)
    }
    pub fn get_songController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IStartSeekSongController> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IStartSeekSongController = __cordl_object
            .invoke("get_songController", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerDidFinishEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerDidFinishEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn ResumeSpawning(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResumeSpawning", ())?;
        Ok(__cordl_ret)
    }
    pub fn __GetActiveOnlyGameObjects(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object.invoke("__GetActiveOnlyGameObjects", ())?;
        Ok(__cordl_ret)
    }
    pub fn PauseSpawning(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PauseSpawning", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_introAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerLocalActivePlayerIntroAnimator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerLocalActivePlayerIntroAnimator = __cordl_object
            .invoke("get_introAnimator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_outroAnimator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("get_outroAnimator", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_currentLocalPlayerLevelCompletionResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LevelCompletionResults> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LevelCompletionResults = __cordl_object
            .invoke("get_currentLocalPlayerLevelCompletionResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn __ForceStopSong(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__ForceStopSong", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_playerNetworkDidFailedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut MultiplayerLevelCompletionResults>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_playerNetworkDidFailedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DisablePlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisablePlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReportPlayerNetworkDidFailed(
        &mut self,
        results: *mut MultiplayerLevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReportPlayerNetworkDidFailed", (results))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerFacade")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLocalActivePlayerFacade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
