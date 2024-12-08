#[cfg(feature = "LeaderboardScoreUploader")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardScoreUploader {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub allScoresDidUploadEvent: *mut crate::System::Action,
    pub _scoresToUpload: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
    >,
    pub _scoresToUploadForCurrentPlayer: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
    >,
    pub _uploadScoreCallback: *mut crate::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback,
    pub _playerId: *mut crate::System::String,
    pub _uploading: bool,
    pub _fileStorage: *mut IFileStorage,
}
#[cfg(feature = "LeaderboardScoreUploader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LeaderboardScoreUploader => ""
    ."LeaderboardScoreUploader"
);
#[cfg(feature = "LeaderboardScoreUploader")]
impl std::ops::Deref for LeaderboardScoreUploader {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader")]
impl std::ops::DerefMut for LeaderboardScoreUploader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader")]
impl LeaderboardScoreUploader {
    pub const kScoresToUploadFileName: &'static str = "ScoresToUpload.dat";
    #[cfg(feature = "LeaderboardScoreUploader+ScoresToUploadData")]
    pub type ScoresToUploadData = crate::GlobalNamespace::LeaderboardScoreUploader_ScoresToUploadData;
    #[cfg(feature = "LeaderboardScoreUploader+ScoreData")]
    pub type ScoreData = crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData;
    #[cfg(feature = "LeaderboardScoreUploader+UploadScoreCallback")]
    pub type UploadScoreCallback = crate::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback;
    #[cfg(feature = "LeaderboardScoreUploader+__c__DisplayClass16_0")]
    pub type __c__DisplayClass16_0 = crate::GlobalNamespace::LeaderboardScoreUploader___c__DisplayClass16_0;
    #[cfg(feature = "LeaderboardScoreUploader+_LoadScoresToUploadFromFile_d__17")]
    pub type _LoadScoresToUploadFromFile_d__17 = crate::GlobalNamespace::LeaderboardScoreUploader__LoadScoresToUploadFromFile_d__17;
    #[cfg(feature = "LeaderboardScoreUploader+_SaveScoresToUploadToFile_d__18")]
    pub type _SaveScoresToUploadToFile_d__18 = crate::GlobalNamespace::LeaderboardScoreUploader__SaveScoresToUploadToFile_d__18;
    #[cfg(feature = "LeaderboardScoreUploader+_UploadScoresCoroutine_d__16")]
    pub type _UploadScoresCoroutine_d__16 = crate::GlobalNamespace::LeaderboardScoreUploader__UploadScoresCoroutine_d__16;
    pub fn AddScore(
        &mut self,
        scoreData: *mut crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddScore", (scoreData))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        uploadScoreCallback: *mut crate::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback,
        playerId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (uploadScoreCallback, playerId))?;
        Ok(__cordl_ret)
    }
    pub fn LoadScoresToUploadFromFile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("LoadScoresToUploadFromFile", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnApplicationQuit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationQuit", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveScoresToUploadToFile(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveScoresToUploadToFile", ())?;
        Ok(__cordl_ret)
    }
    pub fn Uninitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Uninitialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn UploadScoresCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("UploadScoresCoroutine", ())?;
        Ok(__cordl_ret)
    }
    pub fn _UploadScoresCoroutine_b__16_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("<UploadScoresCoroutine>b__16_1", ())?;
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
    pub fn add_allScoresDidUploadEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_allScoresDidUploadEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_allScoresDidUploadEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_allScoresDidUploadEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LeaderboardScoreUploader")]
impl quest_hook::libil2cpp::ObjectType for LeaderboardScoreUploader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LeaderboardScoreUploader+ScoreData")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardScoreUploader_ScoreData {
    __cordl_parent: crate::System::Object,
    pub _playerId_k__BackingField: *mut crate::System::String,
    pub _beatmapKey_k__BackingField: BeatmapKey,
    pub _gameplayModifiers_k__BackingField: *mut GameplayModifiers,
    pub _multipliedScore_k__BackingField: i32,
    pub _modifiedScore_k__BackingField: i32,
    pub _fullCombo_k__BackingField: bool,
    pub _goodCutsCount_k__BackingField: i32,
    pub _badCutsCount_k__BackingField: i32,
    pub _missedCount_k__BackingField: i32,
    pub _maxCombo_k__BackingField: i32,
    pub uploadAttemptCount: i32,
    pub currentUploadAttemptCount: i32,
}
#[cfg(feature = "LeaderboardScoreUploader+ScoreData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LeaderboardScoreUploader_ScoreData => ""
    ."LeaderboardScoreUploader/ScoreData"
);
#[cfg(feature = "LeaderboardScoreUploader+ScoreData")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader+ScoreData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader+ScoreData")]
impl crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData {
    pub fn New(
        playerId: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        multipliedScore: i32,
        modifiedScore: i32,
        fullCombo: bool,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCount: i32,
        maxCombo: i32,
        gameplayModifiers: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    playerId,
                    beatmapKey,
                    multipliedScore,
                    modifiedScore,
                    fullCombo,
                    goodCutsCount,
                    badCutsCount,
                    missedCount,
                    maxCombo,
                    gameplayModifiers,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        playerId: *mut crate::System::String,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        multipliedScore: i32,
        modifiedScore: i32,
        fullCombo: bool,
        goodCutsCount: i32,
        badCutsCount: i32,
        missedCount: i32,
        maxCombo: i32,
        gameplayModifiers: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    playerId,
                    beatmapKey,
                    multipliedScore,
                    modifiedScore,
                    fullCombo,
                    goodCutsCount,
                    badCutsCount,
                    missedCount,
                    maxCombo,
                    gameplayModifiers,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_badCutsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_badCutsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapKey(&mut self) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object.invoke("get_beatmapKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fullCombo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_fullCombo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameplayModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameplayModifiers> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameplayModifiers = __cordl_object
            .invoke("get_gameplayModifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_goodCutsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_goodCutsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_maxCombo(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxCombo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missedCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_missedCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_modifiedScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_modifiedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_multipliedScore(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_multipliedScore", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_playerId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_badCutsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_badCutsCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapKey(
        &mut self,
        value: BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fullCombo(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fullCombo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_gameplayModifiers(
        &mut self,
        value: *mut GameplayModifiers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_gameplayModifiers", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_goodCutsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_goodCutsCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_maxCombo(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxCombo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_missedCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_missedCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_modifiedScore(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_modifiedScore", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_multipliedScore(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_multipliedScore", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_playerId(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_playerId", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LeaderboardScoreUploader+ScoreData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LeaderboardScoreUploader+ScoresToUploadData")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardScoreUploader_ScoresToUploadData {
    __cordl_parent: crate::System::Object,
    pub scores: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
    >,
}
#[cfg(feature = "LeaderboardScoreUploader+ScoresToUploadData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LeaderboardScoreUploader_ScoresToUploadData => ""
    ."LeaderboardScoreUploader/ScoresToUploadData"
);
#[cfg(feature = "LeaderboardScoreUploader+ScoresToUploadData")]
impl std::ops::Deref
for crate::GlobalNamespace::LeaderboardScoreUploader_ScoresToUploadData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader+ScoresToUploadData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LeaderboardScoreUploader_ScoresToUploadData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader+ScoresToUploadData")]
impl crate::GlobalNamespace::LeaderboardScoreUploader_ScoresToUploadData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "LeaderboardScoreUploader+ScoresToUploadData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LeaderboardScoreUploader_ScoresToUploadData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LeaderboardScoreUploader+UploadScoreCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardScoreUploader_UploadScoreCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "LeaderboardScoreUploader+UploadScoreCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback => ""
    ."LeaderboardScoreUploader/UploadScoreCallback"
);
#[cfg(feature = "LeaderboardScoreUploader+UploadScoreCallback")]
impl std::ops::Deref
for crate::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader+UploadScoreCallback")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardScoreUploader+UploadScoreCallback")]
impl crate::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback {
    pub fn BeginInvoke(
        &mut self,
        scoreData: *mut crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
        completionHandler: *mut crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (scoreData, completionHandler, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut HMAsyncRequest = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        scoreData: *mut crate::GlobalNamespace::LeaderboardScoreUploader_ScoreData,
        completionHandler: *mut crate::GlobalNamespace::PlatformLeaderboardsModel_UploadScoreCompletionHandler,
    ) -> quest_hook::libil2cpp::Result<*mut HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut HMAsyncRequest = __cordl_object
            .invoke("Invoke", (scoreData, completionHandler))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LeaderboardScoreUploader+UploadScoreCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LeaderboardScoreUploader_UploadScoreCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
