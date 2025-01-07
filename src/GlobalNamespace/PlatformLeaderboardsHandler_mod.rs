#[cfg(feature = "PlatformLeaderboardsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformLeaderboardsHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlatformLeaderboardsHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsHandler";
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
#[cfg(feature = "PlatformLeaderboardsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformLeaderboardsHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
