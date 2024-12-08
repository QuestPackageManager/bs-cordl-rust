#[cfg(feature = "FileSystemBeatmapLevelData")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemBeatmapLevelData {
    __cordl_parent: crate::System::Object,
    pub _audioClipPath: *mut crate::System::String,
    pub _audioDataPath: *mut crate::System::String,
    pub _name: *mut crate::System::String,
    pub _difficultyBeatmaps: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::ValueTuple_2<
            *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
        *mut crate::GlobalNamespace::FileDifficultyBeatmap,
    >,
    pub _audioClip: *mut crate::UnityEngine::AudioClip,
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileSystemBeatmapLevelData =>
    ""."FileSystemBeatmapLevelData"
);
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl crate::GlobalNamespace::FileSystemBeatmapLevelData {
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
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::FileDifficultyBeatmap,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::FileDifficultyBeatmap = __cordl_object
            .invoke("GetDifficultyBeatmap", (beatmapKey))?;
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
    pub fn New(
        name: *mut crate::System::String,
        audioClipPath: *mut crate::System::String,
        audioDataPath: *mut crate::System::String,
        difficultyBeatmaps: *mut crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
            *mut crate::GlobalNamespace::FileDifficultyBeatmap,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, audioClipPath, audioDataPath, difficultyBeatmaps),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        audioClipPath: *mut crate::System::String,
        audioDataPath: *mut crate::System::String,
        difficultyBeatmaps: *mut crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
            *mut crate::GlobalNamespace::FileDifficultyBeatmap,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, audioClipPath, audioDataPath, difficultyBeatmaps))?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_songAudioClipPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_songAudioClipPath", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
