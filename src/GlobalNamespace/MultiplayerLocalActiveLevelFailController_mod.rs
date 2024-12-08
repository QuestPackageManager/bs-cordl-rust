#[cfg(feature = "MultiplayerLocalActiveLevelFailController")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActiveLevelFailController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _levelFailedTextEffect: *mut LevelFailedTextEffect,
    pub _levelEndActionsPublisher: *mut IMultiplayerLevelEndActionsPublisher,
    pub _beatmapObjectSpawnController: *mut BeatmapObjectSpawnController,
    pub _gameSongController: *mut GameSongController,
    pub _beatmapObjectManager: *mut BeatmapObjectManager,
    pub _multiplayerPlayersManager: *mut MultiplayerPlayersManager,
}
#[cfg(feature = "MultiplayerLocalActiveLevelFailController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLocalActiveLevelFailController => ""
    ."MultiplayerLocalActiveLevelFailController"
);
#[cfg(feature = "MultiplayerLocalActiveLevelFailController")]
impl std::ops::Deref for MultiplayerLocalActiveLevelFailController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActiveLevelFailController")]
impl std::ops::DerefMut for MultiplayerLocalActiveLevelFailController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActiveLevelFailController")]
impl MultiplayerLocalActiveLevelFailController {
    #[cfg(
        feature = "MultiplayerLocalActiveLevelFailController+_PlayerFailedCoroutine_d__8"
    )]
    pub type _PlayerFailedCoroutine_d__8 = crate::GlobalNamespace::MultiplayerLocalActiveLevelFailController__PlayerFailedCoroutine_d__8;
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
    pub fn PlayerFailedCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("PlayerFailedCoroutine", ())?;
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
#[cfg(feature = "MultiplayerLocalActiveLevelFailController")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLocalActiveLevelFailController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
