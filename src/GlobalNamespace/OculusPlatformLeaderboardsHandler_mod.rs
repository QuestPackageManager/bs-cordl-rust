#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformLeaderboardsHandler {
    __cordl_parent: crate::GlobalNamespace::PlatformLeaderboardsHandler,
    pub _leaderboardIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LeaderboardIdsModel,
    >,
    pub _oculusRequestIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<u64>,
    >,
    pub _gameplayModifiersModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiersModelSO,
    >,
}
#[cfg(feature = "OculusPlatformLeaderboardsHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusPlatformLeaderboardsHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusPlatformLeaderboardsHandler";
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
    pub fn AddOculusRequest(
        &mut self,
        oculusRequest: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
        asyncRequest: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::HMAsyncRequest,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddOculusRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddOculusRequest", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (oculusRequest, asyncRequest))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckMessageForValidRequest(
        &mut self,
        message: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>),
                        bool,
                        1usize,
                    >("CheckMessageForValidRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CheckMessageForValidRequest", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (message))? };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                            i32,
                            i32,
                            crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::HMAsyncRequest,
                        >,
                        6usize,
                    >("GetScores")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetScores", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        beatmapKey,
                        count,
                        fromRank,
                        scope,
                        referencePlayerId,
                        completionHandler,
                    ),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::HMAsyncRequest,
                        >,
                        2usize,
                    >("UploadScore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UploadScore", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = unsafe { method.invoke_unchecked(self, (scoreData, completionHandler))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
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
