#[cfg(feature = "MultiplayerScoreProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerScoreProvider {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scoreSyncStateManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IScoreSyncStateManager,
    >,
    pub _firstPlayer_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
    >,
    pub firstPlayerDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
            >,
        >,
    >,
    pub _sharedOffsetSyncTime: i64,
    pub _rankedPlayers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
            >,
        >,
    >,
    pub _players: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
            >,
        >,
    >,
}
#[cfg(feature = "MultiplayerScoreProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerScoreProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerScoreProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "MultiplayerScoreProvider")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerScoreProvider {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerScoreProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerScoreProvider")]
impl crate::GlobalNamespace::MultiplayerScoreProvider {
    #[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
    pub type RankedPlayer = crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer;
    pub fn GetPositionOfPlayer(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPositionOfPlayer", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetScore(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetScore", (userId, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
    pub fn add_firstPlayerDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_firstPlayerDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_firstPlayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        > = __cordl_object.invoke("get_firstPlayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rankedPlayers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
                >,
            >,
        > = __cordl_object.invoke("get_rankedPlayers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scoresAvailable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_scoresAvailable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_firstPlayerDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_firstPlayerDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_firstPlayer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_firstPlayer", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerScoreProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerScoreProvider {
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _multiplayerSyncState: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerSyncState_3<
            crate::GlobalNamespace::StandardScoreSyncState,
            crate::GlobalNamespace::StandardScoreSyncState_Score,
            i32,
        >,
    >,
    pub _scoreSyncManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerScoreProvider,
    >,
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "RankedPlayer";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        multiplayerSyncState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerSyncState_3<
                crate::GlobalNamespace::StandardScoreSyncState,
                crate::GlobalNamespace::StandardScoreSyncState_Score,
                i32,
            >,
        >,
        scoreSyncManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSyncState, scoreSyncManager))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        multiplayerSyncState: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerSyncState_3<
                crate::GlobalNamespace::StandardScoreSyncState,
                crate::GlobalNamespace::StandardScoreSyncState_Score,
                i32,
            >,
        >,
        scoreSyncManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSyncState, scoreSyncManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isActiveOrFinished(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isActiveOrFinished", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isMe(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isMe", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lastScoreTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_lastScoreTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_offsetSyncTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_score(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_score", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wasActiveAtLevelStart(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_wasActiveAtLevelStart", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    >,
> for crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerScoreProvider+RankedPlayer")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    >,
> for crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerScoreProvider_RankedPlayer,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
