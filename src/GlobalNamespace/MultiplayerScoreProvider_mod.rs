#[cfg(feature = "MultiplayerScoreProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerScoreProvider {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scoreSyncStateManager: *mut IScoreSyncStateManager,
    pub _firstPlayer_k__BackingField: *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    pub firstPlayerDidChangeEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    >,
    pub _sharedOffsetSyncTime: i64,
    pub _rankedPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    >,
    pub _players: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    >,
}
#[cfg(feature = "MultiplayerScoreProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerScoreProvider => ""
    ."MultiplayerScoreProvider"
);
#[cfg(feature = "MultiplayerScoreProvider")]
impl std::ops::Deref for MultiplayerScoreProvider {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreProvider")]
impl std::ops::DerefMut for MultiplayerScoreProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreProvider")]
impl MultiplayerScoreProvider {
    #[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
    pub type RankedPlayer = crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer;
    #[cfg(feature = "MultiplayerScoreProvider+__c")]
    pub type __c = crate::GlobalNamespace::MultiplayerScoreProvider___c;
    #[cfg(feature = "MultiplayerScoreProvider+__c__DisplayClass18_0")]
    pub type __c__DisplayClass18_0 = crate::GlobalNamespace::MultiplayerScoreProvider___c__DisplayClass18_0;
    pub fn GetPositionOfPlayer(
        &mut self,
        userId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPositionOfPlayer", (userId))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TryGetScore(
        &mut self,
        userId: *mut crate::System::String,
        data: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetScore", (userId, data))?;
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
    pub fn add_firstPlayerDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_firstPlayerDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_firstPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer = __cordl_object
            .invoke("get_firstPlayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rankedPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        > = __cordl_object.invoke("get_rankedPlayers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scoresAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_scoresAvailable", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_firstPlayerDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_firstPlayerDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_firstPlayer(
        &mut self,
        value: *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_firstPlayer", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerScoreProvider")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerScoreProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerScoreProvider_RankedPlayer {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSyncState: *mut MultiplayerSyncState_3<
        StandardScoreSyncState,
        crate::GlobalNamespace::StandardScoreSyncState_Score,
        i32,
    >,
    pub _scoreSyncManager: *mut MultiplayerScoreProvider,
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer => ""
    ."MultiplayerScoreProvider/RankedPlayer"
);
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
impl crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    pub fn CompareTo(
        &mut self,
        other: *mut crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSyncState: *mut MultiplayerSyncState_3<
            StandardScoreSyncState,
            crate::GlobalNamespace::StandardScoreSyncState_Score,
            i32,
        >,
        scoreSyncManager: *mut MultiplayerScoreProvider,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSyncState, scoreSyncManager))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        multiplayerSyncState: *mut MultiplayerSyncState_3<
            StandardScoreSyncState,
            crate::GlobalNamespace::StandardScoreSyncState_Score,
            i32,
        >,
        scoreSyncManager: *mut MultiplayerScoreProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSyncState, scoreSyncManager))?;
        Ok(__cordl_ret)
    }
    pub fn get_isActiveOrFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActiveOrFinished", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFailed", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isMe(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMe", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lastScoreTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_lastScoreTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_offsetSyncTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_score(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_score", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_wasActiveAtLevelStart(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasActiveAtLevelStart", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}