#[cfg(feature = "MultiplayerOtherPlayersScoreDiffTextManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerOtherPlayersScoreDiffTextManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _multiplayerController: *mut MultiplayerController,
    pub _playersManager: *mut MultiplayerPlayersManager,
    pub _scoreProvider: *mut MultiplayerScoreProvider,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _layoutProvider: *mut MultiplayerLayoutProvider,
    pub _initData: *mut crate::GlobalNamespace::CoreGameHUDController_InitData,
    pub _timeToNextUpdate: f32,
}
#[cfg(feature = "MultiplayerOtherPlayersScoreDiffTextManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerOtherPlayersScoreDiffTextManager => ""
    ."MultiplayerOtherPlayersScoreDiffTextManager"
);
#[cfg(feature = "MultiplayerOtherPlayersScoreDiffTextManager")]
impl std::ops::Deref for MultiplayerOtherPlayersScoreDiffTextManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerOtherPlayersScoreDiffTextManager")]
impl std::ops::DerefMut for MultiplayerOtherPlayersScoreDiffTextManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerOtherPlayersScoreDiffTextManager")]
impl MultiplayerOtherPlayersScoreDiffTextManager {
    pub const kUpdateInterval: f32 = 0.5f32;
    #[cfg(feature = "MultiplayerOtherPlayersScoreDiffTextManager+__c")]
    pub type __c = crate::GlobalNamespace::MultiplayerOtherPlayersScoreDiffTextManager___c;
    pub fn HandleStateChanged(
        &mut self,
        newState: crate::GlobalNamespace::MultiplayerController_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStateChanged", (newState))?;
        Ok(__cordl_ret)
    }
    pub fn HideAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitLeftRightPositions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitLeftRightPositions", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "MultiplayerOtherPlayersScoreDiffTextManager")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerOtherPlayersScoreDiffTextManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}