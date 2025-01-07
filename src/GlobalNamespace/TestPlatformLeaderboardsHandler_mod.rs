#[cfg(feature = "TestPlatformLeaderboardsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct TestPlatformLeaderboardsHandler {
    __cordl_parent: crate::GlobalNamespace::PlatformLeaderboardsHandler,
}
#[cfg(feature = "TestPlatformLeaderboardsHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::TestPlatformLeaderboardsHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TestPlatformLeaderboardsHandler";
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
#[cfg(feature = "TestPlatformLeaderboardsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::TestPlatformLeaderboardsHandler {
    type Target = crate::GlobalNamespace::PlatformLeaderboardsHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TestPlatformLeaderboardsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::TestPlatformLeaderboardsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TestPlatformLeaderboardsHandler")]
impl crate::GlobalNamespace::TestPlatformLeaderboardsHandler {
    pub fn GetScores(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        count: i32,
        fromRank: i32,
        scope: crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
        referencePlayerId: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        completionHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = __cordl_object
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
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UploadScore(
        &mut self,
        scoreData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
        >,
        completionHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = __cordl_object.invoke("UploadScore", (scoreData, completionHandler))?;
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
}
#[cfg(feature = "TestPlatformLeaderboardsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TestPlatformLeaderboardsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
