#[cfg(feature = "BeatmapLevelDataSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelDataSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub _version: i32,
    pub _audioClip: *mut crate::UnityEngine::AudioClip,
    pub _audioDataAsset: *mut crate::UnityEngine::TextAsset,
    pub _difficultyBeatmapSets: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet,
    >,
}
#[cfg(feature = "BeatmapLevelDataSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelDataSO => ""
    ."BeatmapLevelDataSO"
);
#[cfg(feature = "BeatmapLevelDataSO")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelDataSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelDataSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataSO")]
impl crate::GlobalNamespace::BeatmapLevelDataSO {
    #[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmap")]
    pub type DifficultyBeatmap = crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap;
    #[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet")]
    pub type DifficultyBeatmapSet = crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet;
    #[cfg(feature = "BeatmapLevelDataSO+__c__DisplayClass21_0")]
    pub type __c__DisplayClass21_0 = crate::GlobalNamespace::BeatmapLevelDataSO___c__DisplayClass21_0;
    #[cfg(feature = "BeatmapLevelDataSO+__c__DisplayClass22_0")]
    pub type __c__DisplayClass22_0 = crate::GlobalNamespace::BeatmapLevelDataSO___c__DisplayClass22_0;
    pub fn GetAudioDataString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetAudioDataString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAudioDataStringAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetAudioDataStringAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetBeatmapString", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetBeatmapStringAsync", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetDifficultyBeatmap(
        &mut self,
        beatmapKey: crate::GlobalNamespace::BeatmapKey,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap = __cordl_object
            .invoke("GetDifficultyBeatmap", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetDifficultyBeatmapSet(
        &mut self,
        beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet = __cordl_object
            .invoke("GetDifficultyBeatmapSet", (beatmapCharacteristic))?;
        Ok(__cordl_ret)
    }
    pub fn GetLightshowString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetLightshowString", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn GetLightshowStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetLightshowStringAsync", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn IBeatmapLevelData_GetBeatmapString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("IBeatmapLevelData.GetBeatmapString", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn IBeatmapLevelData_GetBeatmapStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object
            .invoke("IBeatmapLevelData.GetBeatmapStringAsync", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn IBeatmapLevelData_GetLightshowString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("IBeatmapLevelData.GetLightshowString", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn IBeatmapLevelData_GetLightshowStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object
            .invoke("IBeatmapLevelData.GetLightshowStringAsync", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn IBeatmapLevelData_get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("IBeatmapLevelData.get_name", ())?;
        Ok(__cordl_ret)
    }
    pub fn IncrementVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetAudioClip(
        &mut self,
        audioClip: *mut crate::UnityEngine::AudioClip,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAudioClip", (audioClip))?;
        Ok(__cordl_ret)
    }
    pub fn SetDifficultyBeatmapSets(
        &mut self,
        difficultyBeatmapSets: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDifficultyBeatmapSets", (difficultyBeatmapSets))?;
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
    pub fn get_audioDataAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextAsset = __cordl_object
            .invoke("get_audioDataAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_difficultyBeatmapSets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet,
        > = __cordl_object.invoke("get_difficultyBeatmapSets", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::AudioClip> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::AudioClip = __cordl_object
            .invoke("get_songAudioClip", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_audioDataAsset(
        &mut self,
        value: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_audioDataAsset", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelDataSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevelDataSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmap")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelDataSO_DifficultyBeatmap {
    __cordl_parent: crate::System::Object,
    pub _difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _beatmapAsset: *mut crate::UnityEngine::TextAsset,
    pub _lightshowAsset: *mut crate::UnityEngine::TextAsset,
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap => ""
    ."BeatmapLevelDataSO/DifficultyBeatmap"
);
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmap")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmap")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmap")]
impl crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap {
    pub fn GetBeatmapString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetBeatmapString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapStringAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetBeatmapStringAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLightshowString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetLightshowString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLightshowStringAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetLightshowStringAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_BeatmapDifficulty_TextAsset0(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapAsset: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (difficulty, beatmapAsset))?;
        Ok(__cordl_object)
    }
    pub fn New_TextAsset1(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapAsset: *mut crate::UnityEngine::TextAsset,
        lightshowAsset: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (difficulty, beatmapAsset, lightshowAsset))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_BeatmapDifficulty_TextAsset0(
        &mut self,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapAsset: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (difficulty, beatmapAsset))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_TextAsset1(
        &mut self,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapAsset: *mut crate::UnityEngine::TextAsset,
        lightshowAsset: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (difficulty, beatmapAsset, lightshowAsset))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextAsset = __cordl_object
            .invoke("get_beatmapAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_difficulty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasLightshowAsset(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasLightshowAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lightshowAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextAsset = __cordl_object
            .invoke("get_lightshowAsset", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_beatmapAsset(
        &mut self,
        value: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_beatmapAsset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lightshowAsset(
        &mut self,
        value: *mut crate::UnityEngine::TextAsset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lightshowAsset", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelDataSO_DifficultyBeatmapSet {
    __cordl_parent: crate::System::Object,
    pub _beatmapCharacteristicSerializedName: *mut crate::System::String,
    pub _difficultyBeatmaps: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap,
    >,
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet => ""
    ."BeatmapLevelDataSO/DifficultyBeatmapSet"
);
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet")]
impl crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet {
    #[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet+__c")]
    pub type __c = crate::GlobalNamespace::DifficultyBeatmapSet_BeatmapLevelDataSO___c;
    pub fn Add(
        &mut self,
        difficultyBeatmap: *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (difficultyBeatmap))?;
        Ok(__cordl_ret)
    }
    pub fn New_IEnumerable_1_0(
        beatmapCharacteristicSerializedName: *mut crate::System::String,
        difficultyBeatmaps: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (beatmapCharacteristicSerializedName, difficultyBeatmaps),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String1(
        beatmapCharacteristicSerializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapCharacteristicSerializedName))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_IEnumerable_1_0(
        &mut self,
        beatmapCharacteristicSerializedName: *mut crate::System::String,
        difficultyBeatmaps: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapCharacteristicSerializedName, difficultyBeatmaps))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String1(
        &mut self,
        beatmapCharacteristicSerializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapCharacteristicSerializedName))?;
        Ok(__cordl_ret)
    }
    pub fn get_beatmapCharacteristicSerializedName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_beatmapCharacteristicSerializedName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_difficultyBeatmaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmap,
        > = __cordl_object.invoke("get_difficultyBeatmaps", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelDataSO+DifficultyBeatmapSet")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelDataSO_DifficultyBeatmapSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
