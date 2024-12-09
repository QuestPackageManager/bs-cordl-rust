#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformLeaderboardsHandler {
    __cordl_parent: crate::GlobalNamespace::PlatformLeaderboardsHandler,
    pub _leaderboardIdsModel: *mut crate::GlobalNamespace::LeaderboardIdsModel,
    pub _oculusRequestIds: *mut crate::System::Collections::Generic::HashSet_1<u64>,
    pub _gameplayModifiersModel: *mut crate::GlobalNamespace::GameplayModifiersModelSO,
}
#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusPlatformLeaderboardsHandler => ""
    ."OculusPlatformLeaderboardsHandler"
);
#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OculusPlatformLeaderboardsHandler {
    type Target = crate::GlobalNamespace::PlatformLeaderboardsHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusPlatformLeaderboardsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
impl crate::GlobalNamespace::OculusPlatformLeaderboardsHandler {
    #[cfg(feature = "OculusPlatformLeaderboardsHandler+__c")]
    pub type __c = crate::GlobalNamespace::OculusPlatformLeaderboardsHandler___c;
    #[cfg(feature = "OculusPlatformLeaderboardsHandler+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::GlobalNamespace::OculusPlatformLeaderboardsHandler___c__DisplayClass4_0;
    #[cfg(feature = "OculusPlatformLeaderboardsHandler+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::GlobalNamespace::OculusPlatformLeaderboardsHandler___c__DisplayClass6_0;
    #[cfg(feature = "OculusPlatformLeaderboardsHandler+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::GlobalNamespace::OculusPlatformLeaderboardsHandler___c__DisplayClass7_0;
    pub fn AddOculusRequest(
        &mut self,
        oculusRequest: *mut crate::Oculus::Platform::Request,
        asyncRequest: *mut crate::GlobalNamespace::HMAsyncRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOculusRequest", (oculusRequest, asyncRequest))?;
        Ok(__cordl_ret)
    }
    pub fn CheckMessageForValidRequest(
        &mut self,
        message: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckMessageForValidRequest", (message))?;
        Ok(__cordl_ret)
    }
    pub fn GetScores(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        count: i32,
        fromRank: i32,
        scope: crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
        referencePlayerId: *mut quest_hook::libil2cpp::Il2CppString,
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
#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusPlatformLeaderboardsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
