#[cfg(feature = "LeaderboardUploadData")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardUploadData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub score: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub authorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub bpm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub infoHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub modifiers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
}
#[cfg(feature = "LeaderboardUploadData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LeaderboardUploadData => ""
    ."LeaderboardUploadData"
);
#[cfg(feature = "LeaderboardUploadData")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardUploadData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardUploadData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl crate::GlobalNamespace::LeaderboardUploadData {
    pub fn New(
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        authorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bpm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        infoHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    playerName,
                    playerId,
                    score,
                    leaderboardId,
                    songName,
                    songSubName,
                    authorName,
                    bpm,
                    difficulty,
                    infoHash,
                    modifiers,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        score: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        leaderboardId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        authorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bpm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        infoHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    playerName,
                    playerId,
                    score,
                    leaderboardId,
                    songName,
                    songSubName,
                    authorName,
                    bpm,
                    difficulty,
                    infoHash,
                    modifiers,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LeaderboardUploadData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
