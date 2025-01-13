#[cfg(feature = "PlatformLeaderboardsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformLeaderboardsModel {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gameplayModifiersModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayModifiersModelSO,
    >,
    pub _leaderboardScoreUploader: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LeaderboardScoreUploader,
    >,
    pub _platformUserModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPlatformUserModel,
    >,
    pub _platformLeaderboardsHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlatformLeaderboardsHandler,
    >,
    pub _beatmapLevelsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsModel,
    >,
    pub allScoresDidUploadEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _currentGetScoreRequest: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::HMAsyncRequest,
    >,
    pub _state: crate::GlobalNamespace::PlatformLeaderboardsModel_State,
    pub _playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "PlatformLeaderboardsModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel";
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
#[cfg(feature = "PlatformLeaderboardsModel")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformLeaderboardsModel {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformLeaderboardsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel")]
impl crate::GlobalNamespace::PlatformLeaderboardsModel {
    #[cfg(feature = "PlatformLeaderboardsModel+GetScoresCompletionHandler")]
    pub type GetScoresCompletionHandler = crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler;
    #[cfg(feature = "PlatformLeaderboardsModel+GetScoresResult")]
    pub type GetScoresResult = crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult;
    #[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
    pub type LeaderboardScore = crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore;
    #[cfg(feature = "PlatformLeaderboardsModel+ScoresScope")]
    pub type ScoresScope = crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope;
    #[cfg(feature = "PlatformLeaderboardsModel+State")]
    pub type State = crate::GlobalNamespace::PlatformLeaderboardsModel_State;
    #[cfg(feature = "PlatformLeaderboardsModel+UploadScoreCompletionHandler")]
    pub type UploadScoreCompletionHandler = crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler;
    #[cfg(feature = "PlatformLeaderboardsModel+UploadScoreResult")]
    pub type UploadScoreResult = crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult;
    pub fn GetFriendsScores(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        count: i32,
        fromRank: i32,
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
                "GetFriendsScores",
                (beatmapKey, count, fromRank, completionHandler),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScoresAroundPlayer(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        count: i32,
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
            .invoke("GetScoresAroundPlayer", (beatmapKey, count, completionHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScores_BeatmapKey_PlatformLeaderboardsModel_ScoresScope_PlatformLeaderboardsModel_GetScoresCompletionHandler0(
        &mut self,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
        count: i32,
        fromRank: i32,
        scope: crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope,
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
                (beatmapKey, count, fromRank, scope, completionHandler),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScores_ByRefMut_PlatformLeaderboardsModel_GetScoresCompletionHandler1(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        count: i32,
        fromRank: i32,
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
            .invoke("GetScores", (beatmapKey, count, fromRank, completionHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleAllScoresDidUpload(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAllScoresDidUpload", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlatformUserInfoDidChange(
        &mut self,
        newInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandlePlatformUserInfoDidChange", (newInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeForUserInfo(
        &mut self,
        newInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeForUserInfo", (newInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UploadScore_ByRefMut_i32_i32_i32__cordl_bool_i32_i32_i32_i32_f32_GameplayModifiers1(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        multipliedScore: i32,
        modifiedScore: i32,
        maxPossibleMultipliedScore: i32,
        fullCombo: bool,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCount: i32,
        maxCombo: i32,
        energy: f32,
        gameplayModifiers: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayModifiers,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UploadScore",
                (
                    beatmapKey,
                    multipliedScore,
                    modifiedScore,
                    maxPossibleMultipliedScore,
                    fullCombo,
                    goodCutsCount,
                    badCutsCount,
                    missedCount,
                    maxCombo,
                    energy,
                    gameplayModifiers,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UploadScore_LeaderboardScoreUploader_ScoreData_PlatformLeaderboardsModel_UploadScoreCompletionHandler0(
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
    pub fn add_allScoresDidUploadEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_allScoresDidUploadEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_initialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_initialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_playerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_playerId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_allScoresDidUploadEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_allScoresDidUploadEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformLeaderboardsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresCompletionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformLeaderboardsModel_GetScoresCompletionHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresCompletionHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel/GetScoresCompletionHandler";
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
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresCompletionHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresCompletionHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresCompletionHandler")]
impl crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler {
    pub fn BeginInvoke(
        &mut self,
        result: crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult,
        scores: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
                >,
            >,
        >,
        referencePlayerScoreIndex: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke(
                "BeginInvoke",
                (result, scores, referencePlayerScoreIndex, callback, object),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        result: crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult,
        scores: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
                >,
            >,
        >,
        referencePlayerScoreIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (result, scores, referencePlayerScoreIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresCompletionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresCompletionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformLeaderboardsModel_GetScoresResult {
    #[default]
    Failed = 1i32,
    FailedTooManyRequests = 2i32,
    _cordl_Ok = 0i32,
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel/GetScoresResult";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+GetScoresResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlatformLeaderboardsModel_GetScoresResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformLeaderboardsModel_LeaderboardScore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub score: i32,
    pub rank: i32,
    pub playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel/LeaderboardScore";
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
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
impl std::ops::Deref
for crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
impl crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore {
    pub fn CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        score: i32,
        rank: i32,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (score, rank, playerName, playerId))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        score: i32,
        rank: i32,
        playerName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        playerId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (score, rank, playerName, playerId))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    >,
> for crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+LeaderboardScore")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    >,
> for crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+ScoresScope")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformLeaderboardsModel_ScoresScope {
    #[default]
    AroundPlayer = 1i32,
    Friends = 2i32,
    Global = 0i32,
}
#[cfg(feature = "PlatformLeaderboardsModel+ScoresScope")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel/ScoresScope";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+ScoresScope")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+ScoresScope")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+ScoresScope")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+ScoresScope")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlatformLeaderboardsModel_ScoresScope {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformLeaderboardsModel_State {
    #[default]
    Initialized = 2i32,
    Initializing = 1i32,
    NotInitialized = 0i32,
}
#[cfg(feature = "PlatformLeaderboardsModel+State")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel_State {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel/State";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+State")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlatformLeaderboardsModel_State {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+State")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlatformLeaderboardsModel_State {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+State")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlatformLeaderboardsModel_State {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+State")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlatformLeaderboardsModel_State {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreCompletionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformLeaderboardsModel_UploadScoreCompletionHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreCompletionHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel/UploadScoreCompletionHandler";
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
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreCompletionHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreCompletionHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreCompletionHandler")]
impl crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler {
    pub fn BeginInvoke(
        &mut self,
        result: crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (result, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        result: crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreCompletionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlatformLeaderboardsModel_UploadScoreResult {
    #[default]
    Failed = 1i32,
    FailedTooManyRequests = 2i32,
    _cordl_Ok = 0i32,
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreResult")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformLeaderboardsModel/UploadScoreResult";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreResult")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreResult")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreResult")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "PlatformLeaderboardsModel+UploadScoreResult")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreResult {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
