#[cfg(feature = "LeaderboardUploadData")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardUploadData {
    __cordl_parent: crate::System::Object,
    pub playerName: *mut crate::System::String,
    pub playerId: *mut crate::System::String,
    pub score: *mut crate::System::String,
    pub leaderboardId: *mut crate::System::String,
    pub songName: *mut crate::System::String,
    pub songSubName: *mut crate::System::String,
    pub authorName: *mut crate::System::String,
    pub bpm: *mut crate::System::String,
    pub difficulty: *mut crate::System::String,
    pub infoHash: *mut crate::System::String,
    pub modifiers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "LeaderboardUploadData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LeaderboardUploadData => ""."LeaderboardUploadData"
);
#[cfg(feature = "LeaderboardUploadData")]
impl std::ops::Deref for LeaderboardUploadData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl std::ops::DerefMut for LeaderboardUploadData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl LeaderboardUploadData {
    pub fn _ctor(
        &mut self,
        playerName: *mut crate::System::String,
        playerId: *mut crate::System::String,
        score: *mut crate::System::String,
        leaderboardId: *mut crate::System::String,
        songName: *mut crate::System::String,
        songSubName: *mut crate::System::String,
        authorName: *mut crate::System::String,
        bpm: *mut crate::System::String,
        difficulty: *mut crate::System::String,
        infoHash: *mut crate::System::String,
        modifiers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn New(
        playerName: *mut crate::System::String,
        playerId: *mut crate::System::String,
        score: *mut crate::System::String,
        leaderboardId: *mut crate::System::String,
        songName: *mut crate::System::String,
        songSubName: *mut crate::System::String,
        authorName: *mut crate::System::String,
        bpm: *mut crate::System::String,
        difficulty: *mut crate::System::String,
        infoHash: *mut crate::System::String,
        modifiers: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LeaderboardUploadData")]
impl quest_hook::libil2cpp::ObjectType for LeaderboardUploadData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
