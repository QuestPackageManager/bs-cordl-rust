#[cfg(feature = "BeatmapLevelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _version: i32,
    pub _levelID: *mut quest_hook::libil2cpp::Il2CppString,
    pub _songName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _songSubName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _songAuthorName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _levelAuthorName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _previewAudioClip: *mut crate::UnityEngine::AudioClip,
    pub _beatsPerMinute: f32,
    pub _integratedLufs: f32,
    pub _songTimeOffset: f32,
    pub _shuffle: f32,
    pub _shufflePeriod: f32,
    pub _previewStartTime: f32,
    pub _previewDuration: f32,
    pub _songDuration: f32,
    pub _coverImage: *mut crate::UnityEngine::Sprite,
    pub _environmentName: crate::GlobalNamespace::EnvironmentName,
    pub _allDirectionsEnvironmentName: crate::GlobalNamespace::EnvironmentName,
    pub _environmentNames: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::EnvironmentName,
    >,
    pub _previewDifficultyBeatmapSets: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet,
    >,
    pub _contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
}
#[cfg(feature = "BeatmapLevelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelSO => ""
    ."BeatmapLevelSO"
);
#[cfg(feature = "BeatmapLevelSO")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSO")]
impl crate::GlobalNamespace::BeatmapLevelSO {
    #[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmap")]
    pub type PreviewDifficultyBeatmap = crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap;
    #[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet")]
    pub type PreviewDifficultyBeatmapSet = crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet;
    pub fn IncrementVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncrementVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetCoverImage(
        &mut self,
        coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCoverImage", (coverImage))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatsPerMinute: f32,
        songTimeOffset: f32,
        songDuration: f32,
        shuffle: f32,
        shufflePeriod: f32,
        previewStartTime: f32,
        previewDuration: f32,
        coverImage: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        environmentName: crate::GlobalNamespace::EnvironmentName,
        allDirectionEnvironmentName: crate::GlobalNamespace::EnvironmentName,
        environmentNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::EnvironmentName>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    levelID,
                    songName,
                    songSubName,
                    songAuthorName,
                    levelAuthorName,
                    beatsPerMinute,
                    songTimeOffset,
                    songDuration,
                    shuffle,
                    shufflePeriod,
                    previewStartTime,
                    previewDuration,
                    coverImage,
                    environmentName,
                    allDirectionEnvironmentName,
                    environmentNames,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIntegratedLufs(
        &mut self,
        integratedLufs: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIntegratedLufs", (integratedLufs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPreviewAudioClip(
        &mut self,
        audioClip: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPreviewAudioClip", (audioClip))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPreviewDifficultyBeatmaps(
        &mut self,
        previewDifficultyBeatmapSets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPreviewDifficultyBeatmaps", (previewDifficultyBeatmapSets))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSongDuration(
        &mut self,
        songDuration: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSongDuration", (songDuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn __SetEnvironmentName(
        &mut self,
        targetEnvironmentName: crate::GlobalNamespace::EnvironmentName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("__SetEnvironmentName", (targetEnvironmentName))?;
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
    pub fn get_allDirectionsEnvironmentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentName = __cordl_object
            .invoke("get_allDirectionsEnvironmentName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatsPerMinute(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatsPerMinute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_contentRating(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::PlayerSensitivityFlag> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::PlayerSensitivityFlag = __cordl_object
            .invoke("get_contentRating", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_coverImage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = __cordl_object
            .invoke("get_coverImage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentName = __cordl_object
            .invoke("get_environmentName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                crate::GlobalNamespace::EnvironmentName,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                crate::GlobalNamespace::EnvironmentName,
            >,
        > = __cordl_object.invoke("get_environmentNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasPrecalculatedData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPrecalculatedData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_integratedLufs(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_integratedLufs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelAuthorName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelAuthorName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_levelID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_levelID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previewDifficultyBeatmapSets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet,
            >,
        > = __cordl_object.invoke("get_previewDifficultyBeatmapSets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previewDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_previewDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previewStartTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_previewStartTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shuffle(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shuffle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shufflePeriod(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shufflePeriod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songAuthorName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songAuthorName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songDuration(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songDuration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songPreviewAudioClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AudioClip> = __cordl_object
            .invoke("get_songPreviewAudioClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songSubName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songSubName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_songTimeOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_songTimeOffset", ())?;
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
#[cfg(feature = "BeatmapLevelSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelSO")]
impl AsRef<crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider>
for crate::GlobalNamespace::BeatmapLevelSO {
    fn as_ref(&self) -> &crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelSO")]
impl AsMut<crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider>
for crate::GlobalNamespace::BeatmapLevelSO {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IAssetSongPreviewAudioClipProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmap")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSO_PreviewDifficultyBeatmap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    pub _environmentNameIdx: i32,
    pub _beatmapColorSchemeIdx: i32,
    pub _noteJumpMovementSpeed: f32,
    pub _noteJumpStartBeatOffset: f32,
    pub _notesCount: i32,
    pub _obstaclesCount: i32,
    pub _bombsCount: i32,
    pub _cuttableBeatmapObjectsCount: i32,
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap => ""
    ."BeatmapLevelSO/PreviewDifficultyBeatmap"
);
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmap")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmap")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmap")]
impl crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap {
    pub fn New(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        environmentNameIdx: i32,
        beatmapColorSchemeIdx: i32,
        notesCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    difficulty,
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                    environmentNameIdx,
                    beatmapColorSchemeIdx,
                    notesCount,
                    obstaclesCount,
                    bombsCount,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        environmentNameIdx: i32,
        beatmapColorSchemeIdx: i32,
        notesCount: i32,
        obstaclesCount: i32,
        bombsCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    difficulty,
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                    environmentNameIdx,
                    beatmapColorSchemeIdx,
                    notesCount,
                    obstaclesCount,
                    bombsCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapColorSchemeIdx(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_beatmapColorSchemeIdx", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bombsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bombsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cuttableBeatmapObjectsCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_cuttableBeatmapObjectsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapDifficulty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapDifficulty = __cordl_object
            .invoke("get_difficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentNameIdx(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_environmentNameIdx", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpMovementSpeed(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpMovementSpeed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_noteJumpStartBeatOffset(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_noteJumpStartBeatOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_notesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_notesCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_obstaclesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_obstaclesCount", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelSO_PreviewDifficultyBeatmapSet {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapCharacteristic: *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    pub _previewDifficultyBeatmaps: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap,
    >,
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet => ""
    ."BeatmapLevelSO/PreviewDifficultyBeatmapSet"
);
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet")]
impl crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet {
    #[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet+__c")]
    pub type __c = crate::GlobalNamespace::PreviewDifficultyBeatmapSet_BeatmapLevelSO___c;
    pub fn Add(
        &mut self,
        previewDifficultyBeatmap: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (previewDifficultyBeatmap))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_BeatmapCharacteristicSO0(
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapCharacteristic))?;
        Ok(__cordl_object.into())
    }
    pub fn New_List_1_1(
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        previewDifficultyBeatmaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapCharacteristic, previewDifficultyBeatmaps))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_BeatmapCharacteristicSO0(
        &mut self,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapCharacteristic))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_List_1_1(
        &mut self,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        previewDifficultyBeatmaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapCharacteristic, previewDifficultyBeatmaps))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapCharacteristic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        > = __cordl_object.invoke("get_beatmapCharacteristic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficultyBeatmaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmap,
            >,
        > = __cordl_object.invoke("get_difficultyBeatmaps", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelSO+PreviewDifficultyBeatmapSet")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelSO_PreviewDifficultyBeatmapSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
