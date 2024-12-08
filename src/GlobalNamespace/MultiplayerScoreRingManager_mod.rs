#[cfg(feature = "MultiplayerScoreRingManager")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerScoreRingManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _delayBetweenScoreUpdates: f32,
    pub _centerDistanceOffset: f32,
    pub _multiplayerController: *mut MultiplayerController,
    pub _multiplayerPlayersManager: *mut MultiplayerPlayersManager,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _spawnCenter: *mut BeatmapObjectSpawnCenter,
    pub _layoutProvider: *mut MultiplayerLayoutProvider,
    pub _scoreProvider: *mut MultiplayerScoreProvider,
    pub _scoreRingItemPool: *mut crate::GlobalNamespace::MultiplayerScoreRingItem_Pool,
    pub _scoreRingItems: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut MultiplayerScoreRingItem,
    >,
    pub _allActivePlayers: *mut crate::System::Collections::Generic::List_1<
        *mut IConnectedPlayer,
    >,
    pub _currentlyScoreUpdateIndex: i32,
    pub _timeSinceLastScoreUpdate: f32,
    pub _firstPlayerItem: *mut MultiplayerScoreRingItem,
    pub _spawnCenterDistanceFound: bool,
    pub _playersSpawned: bool,
}
#[cfg(feature = "MultiplayerScoreRingManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerScoreRingManager => ""
    ."MultiplayerScoreRingManager"
);
#[cfg(feature = "MultiplayerScoreRingManager")]
impl std::ops::Deref for MultiplayerScoreRingManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreRingManager")]
impl std::ops::DerefMut for MultiplayerScoreRingManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreRingManager")]
impl MultiplayerScoreRingManager {
    pub fn TrySpawnTexts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TrySpawnTexts", ())?;
        Ok(__cordl_ret)
    }
    pub fn SpawnTexts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnTexts", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerSpawningDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerSpawningDidFinish", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetScoreRingItem(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerScoreRingItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerScoreRingItem = __cordl_object
            .invoke("GetScoreRingItem", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn HandleStateChanged(
        &mut self,
        state: crate::GlobalNamespace::MultiplayerController_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleStateChanged", (state))?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerDisconnected(
        &mut self,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerDisconnected", (player))?;
        Ok(__cordl_ret)
    }
    pub fn AnimateColorsForAllPlayers(
        &mut self,
        nameColor: crate::UnityEngine::Color,
        scoreColor: crate::UnityEngine::Color,
        duration: f32,
        easeType: EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AnimateColorsForAllPlayers",
                (nameColor, scoreColor, duration, easeType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScore(
        &mut self,
        playerToUpdate: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScore", (playerToUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn GetScoreRingItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::GameObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::GameObject,
        > = __cordl_object.invoke("GetScoreRingItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleSpawnCenterDistanceWasFound(
        &mut self,
        spawnCenterDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSpawnCenterDistanceWasFound", (spawnCenterDistance))?;
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
    pub fn SetPlayerToFailedState(
        &mut self,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPlayerToFailedState", (player))?;
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
    pub fn AnimateColorsForPlayer(
        &mut self,
        userId: *mut crate::System::String,
        nameColor: crate::UnityEngine::Color,
        scoreColor: crate::UnityEngine::Color,
        duration: f32,
        easeType: EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AnimateColorsForPlayer",
                (userId, nameColor, scoreColor, duration, easeType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandlePlayerStateChanged(
        &mut self,
        player: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlayerStateChanged", (player))?;
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
#[cfg(feature = "MultiplayerScoreRingManager")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerScoreRingManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
