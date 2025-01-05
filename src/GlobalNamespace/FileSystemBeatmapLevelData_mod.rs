#[cfg(feature = "FileSystemBeatmapLevelData")]
#[repr(C)]
#[derive(Debug)]
pub struct FileSystemBeatmapLevelData {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _audioClipPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _audioDataPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _difficultyBeatmaps: quest_hook::libil2cpp::Gc<
        crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FileDifficultyBeatmap>,
    >,
    pub _audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileSystemBeatmapLevelData =>
    ""."FileSystemBeatmapLevelData"
);
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl std::ops::Deref for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn ContainsBeatmapData(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ContainsBeatmapData", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAudioDataString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetAudioDataString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAudioDataStringAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetAudioDataStringAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetBeatmapString", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetBeatmapStringAsync", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDifficultyBeatmap(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FileDifficultyBeatmap>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FileDifficultyBeatmap,
        > = __cordl_object.invoke("GetDifficultyBeatmap", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLightshowString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLightshowString", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLightshowStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("GetLightshowStringAsync", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn IBeatmapLevelData_ContainsBeatmapData(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IBeatmapLevelData.ContainsBeatmapData", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn IBeatmapLevelData_GetBeatmapString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("IBeatmapLevelData.GetBeatmapString", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn IBeatmapLevelData_GetBeatmapStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object
            .invoke("IBeatmapLevelData.GetBeatmapStringAsync", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn IBeatmapLevelData_GetLightshowString(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("IBeatmapLevelData.GetLightshowString", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn IBeatmapLevelData_GetLightshowStringAsync(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object
            .invoke("IBeatmapLevelData.GetLightshowStringAsync", (beatmapKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        audioClipPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        audioDataPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficultyBeatmaps: quest_hook::libil2cpp::Gc<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FileDifficultyBeatmap>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (name, audioClipPath, audioDataPath, difficultyBeatmaps),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        audioClipPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        audioDataPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficultyBeatmaps: quest_hook::libil2cpp::Gc<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FileDifficultyBeatmap>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, audioClipPath, audioDataPath, difficultyBeatmaps))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_name", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songAudioClipPath(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songAudioClipPath", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>>
for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData>>
for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatmapLevelData> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFilePathSongAudioClipProvider>,
> for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IFilePathSongAudioClipProvider,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FileSystemBeatmapLevelData")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IFilePathSongAudioClipProvider>,
> for crate::GlobalNamespace::FileSystemBeatmapLevelData {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IFilePathSongAudioClipProvider,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
