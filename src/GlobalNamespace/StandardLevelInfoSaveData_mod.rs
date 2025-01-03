#[cfg(feature = "StandardLevelInfoSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelInfoSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _version: *mut quest_hook::libil2cpp::Il2CppString,
    pub _songName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _songSubName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _songAuthorName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _levelAuthorName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _beatsPerMinute: f32,
    pub _songTimeOffset: f32,
    pub _shuffle: f32,
    pub _shufflePeriod: f32,
    pub _previewStartTime: f32,
    pub _previewDuration: f32,
    pub _songFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub _coverImageFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub _environmentName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _allDirectionsEnvironmentName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _environmentNames: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _colorSchemes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapLevelColorSchemeSaveData,
    >,
    pub _difficultyBeatmapSets: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet,
    >,
}
#[cfg(feature = "StandardLevelInfoSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::StandardLevelInfoSaveData => ""
    ."StandardLevelInfoSaveData"
);
#[cfg(feature = "StandardLevelInfoSaveData")]
impl std::ops::Deref for crate::GlobalNamespace::StandardLevelInfoSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData")]
impl std::ops::DerefMut for crate::GlobalNamespace::StandardLevelInfoSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData")]
impl crate::GlobalNamespace::StandardLevelInfoSaveData {
    pub const kCurrentVersion: &'static str = "2.1.0";
    pub const kDefaultBeatmapCharacteristicName: &'static str = "Standard";
    #[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmap")]
    pub type DifficultyBeatmap = crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap;
    #[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmapSet")]
    pub type DifficultyBeatmapSet = crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet;
    pub fn DeserializeFromJSONString(
        stringData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::StandardLevelInfoSaveData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::StandardLevelInfoSaveData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeserializeFromJSONString", (stringData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatsPerMinute: f32,
        songTimeOffset: f32,
        shuffle: f32,
        shufflePeriod: f32,
        previewStartTime: f32,
        previewDuration: f32,
        songFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverImageFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allDirectionsEnvironmentName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        environmentNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        colorSchemes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapLevelColorSchemeSaveData,
            >,
        >,
        difficultyBeatmapSets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    songName,
                    songSubName,
                    songAuthorName,
                    levelAuthorName,
                    beatsPerMinute,
                    songTimeOffset,
                    shuffle,
                    shufflePeriod,
                    previewStartTime,
                    previewDuration,
                    songFilename,
                    coverImageFilename,
                    environmentName,
                    allDirectionsEnvironmentName,
                    environmentNames,
                    colorSchemes,
                    difficultyBeatmapSets,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        levelAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beatsPerMinute: f32,
        songTimeOffset: f32,
        shuffle: f32,
        shufflePeriod: f32,
        previewStartTime: f32,
        previewDuration: f32,
        songFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        coverImageFilename: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        environmentName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allDirectionsEnvironmentName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        environmentNames: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        colorSchemes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapLevelColorSchemeSaveData,
            >,
        >,
        difficultyBeatmapSets: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    songName,
                    songSubName,
                    songAuthorName,
                    levelAuthorName,
                    beatsPerMinute,
                    songTimeOffset,
                    shuffle,
                    shufflePeriod,
                    previewStartTime,
                    previewDuration,
                    songFilename,
                    coverImageFilename,
                    environmentName,
                    allDirectionsEnvironmentName,
                    environmentNames,
                    colorSchemes,
                    difficultyBeatmapSets,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allDirectionsEnvironmentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_allDirectionsEnvironmentName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatsPerMinute(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatsPerMinute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorSchemes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapLevelColorSchemeSaveData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapLevelColorSchemeSaveData,
            >,
        > = __cordl_object.invoke("get_colorSchemes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_coverImageFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_coverImageFilename", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficultyBeatmapSets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet,
            >,
        > = __cordl_object.invoke("get_difficultyBeatmapSets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_environmentName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_environmentNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        > = __cordl_object.invoke("get_environmentNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasAllData(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasAllData", ())?;
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
    pub fn get_songFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_songFilename", ())?;
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
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_version", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelInfoSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelInfoSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmap")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelInfoSaveData_DifficultyBeatmap {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _difficulty: *mut quest_hook::libil2cpp::Il2CppString,
    pub _difficultyRank: i32,
    pub _beatmapFilename: *mut quest_hook::libil2cpp::Il2CppString,
    pub _noteJumpMovementSpeed: f32,
    pub _noteJumpStartBeatOffset: f32,
    pub _beatmapColorSchemeIdx: i32,
    pub _environmentNameIdx: i32,
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmap")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap => ""
    ."StandardLevelInfoSaveData/DifficultyBeatmap"
);
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmap")]
impl std::ops::Deref
for crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmap")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmap")]
impl crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap {
    pub fn New(
        difficultyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficultyRank: i32,
        beatmapFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        beatmapColorSchemeIdx: i32,
        environmentNameIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    difficultyName,
                    difficultyRank,
                    beatmapFilename,
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                    beatmapColorSchemeIdx,
                    environmentNameIdx,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        difficultyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficultyRank: i32,
        beatmapFilename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        noteJumpMovementSpeed: f32,
        noteJumpStartBeatOffset: f32,
        beatmapColorSchemeIdx: i32,
        environmentNameIdx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    difficultyName,
                    difficultyRank,
                    beatmapFilename,
                    noteJumpMovementSpeed,
                    noteJumpStartBeatOffset,
                    beatmapColorSchemeIdx,
                    environmentNameIdx,
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
    pub fn get_beatmapFilename(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_beatmapFilename", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficulty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_difficulty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficultyRank(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_difficultyRank", ())?;
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
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmap")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmapSet")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardLevelInfoSaveData_DifficultyBeatmapSet {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _beatmapCharacteristicName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _difficultyBeatmaps: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap,
    >,
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmapSet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet => ""
    ."StandardLevelInfoSaveData/DifficultyBeatmapSet"
);
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmapSet")]
impl std::ops::Deref
for crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmapSet")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmapSet")]
impl crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet {
    pub fn New(
        beatmapCharacteristicName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        difficultyBeatmaps: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapCharacteristicName, difficultyBeatmaps))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapCharacteristicName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        difficultyBeatmaps: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapCharacteristicName, difficultyBeatmaps))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapCharacteristicName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_beatmapCharacteristicName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_difficultyBeatmaps(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmap,
            >,
        > = __cordl_object.invoke("get_difficultyBeatmaps", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardLevelInfoSaveData+DifficultyBeatmapSet")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardLevelInfoSaveData_DifficultyBeatmapSet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
