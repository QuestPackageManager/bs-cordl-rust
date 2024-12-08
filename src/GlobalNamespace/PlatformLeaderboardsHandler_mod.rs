#[cfg(feature = "PlatformLeaderboardsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformLeaderboardsHandler {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "PlatformLeaderboardsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PlatformLeaderboardsHandler =>
    ""."PlatformLeaderboardsHandler"
);
#[cfg(feature = "PlatformLeaderboardsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformLeaderboardsHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformLeaderboardsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsHandler")]
impl crate::GlobalNamespace::PlatformLeaderboardsHandler {
    pub fn GetScores(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        count: i32,
        fromRank: i32,
        scope: crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
        referencePlayerId: *mut crate::System::String,
        completionHandler: *mut crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::HMAsyncRequest = __cordl_object
            .invoke(
                "GetScores",
                (
                    beatmapKey,
                    count,
                    fromRank,
                    scope,
                    referencePlayerId,
                    completionHandler,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UploadScore(
        &mut self,
        scoreData: *mut crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
        completionHandler: *mut crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::HMAsyncRequest = __cordl_object
            .invoke("UploadScore", (scoreData, completionHandler))?;
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
#[cfg(feature = "PlatformLeaderboardsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformLeaderboardsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
