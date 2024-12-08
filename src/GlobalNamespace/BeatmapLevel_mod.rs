#[cfg(feature = "BeatmapLevel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevel {
    __cordl_parent: crate::System::Object,
    pub version: i32,
    pub hasPrecalculatedData: bool,
    pub levelID: *mut crate::System::String,
    pub songName: *mut crate::System::String,
    pub songSubName: *mut crate::System::String,
    pub songAuthorName: *mut crate::System::String,
    pub allMappers: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub allLighters: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub beatsPerMinute: f32,
    pub integratedLufs: f32,
    pub songTimeOffset: f32,
    pub previewStartTime: f32,
    pub previewDuration: f32,
    pub songDuration: f32,
    pub contentRating: PlayerSensitivityFlag,
    pub previewMediaData: *mut IPreviewMediaData,
    pub beatmapBasicData: *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
        crate::System::ValueTuple_2<*mut BeatmapCharacteristicSO, BeatmapDifficulty>,
        *mut BeatmapBasicData,
    >,
    pub _characteristicsCache: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapCharacteristicSO,
    >,
    pub _beatmapKeysCache: *mut quest_hook::libil2cpp::Il2CppArray<BeatmapKey>,
}
#[cfg(feature = "BeatmapLevel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevel => ""."BeatmapLevel"
);
#[cfg(feature = "BeatmapLevel")]
impl std::ops::Deref for BeatmapLevel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevel")]
impl std::ops::DerefMut for BeatmapLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevel")]
impl BeatmapLevel {
    pub const kInvalidVersion: i32 = -1i32;
    #[cfg(feature = "BeatmapLevel+__c__DisplayClass21_0")]
    pub type __c__DisplayClass21_0 = crate::GlobalNamespace::BeatmapLevel___c__DisplayClass21_0;
    #[cfg(feature = "BeatmapLevel+__c")]
    pub type __c = crate::GlobalNamespace::BeatmapLevel___c;
    pub fn GetBeatmapKeys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<BeatmapKey>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            BeatmapKey,
        > = __cordl_object.invoke("GetBeatmapKeys", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCharacteristics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut BeatmapCharacteristicSO,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut BeatmapCharacteristicSO,
        > = __cordl_object.invoke("GetCharacteristics", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetColorScheme(
        &mut self,
        characteristic: *mut BeatmapCharacteristicSO,
        difficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<*mut ColorScheme> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ColorScheme = __cordl_object
            .invoke("GetColorScheme", (characteristic, difficulty))?;
        Ok(__cordl_ret)
    }
    pub fn GetDifficulties(
        &mut self,
        characteristic: *mut BeatmapCharacteristicSO,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<BeatmapDifficulty>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            BeatmapDifficulty,
        > = __cordl_object.invoke("GetDifficulties", (characteristic))?;
        Ok(__cordl_ret)
    }
    pub fn GetDifficultyBeatmapData(
        &mut self,
        characteristic: *mut BeatmapCharacteristicSO,
        difficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapBasicData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapBasicData = __cordl_object
            .invoke("GetDifficultyBeatmapData", (characteristic, difficulty))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnvironmentName(
        &mut self,
        characteristic: *mut BeatmapCharacteristicSO,
        difficulty: BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<EnvironmentName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: EnvironmentName = __cordl_object
            .invoke("GetEnvironmentName", (characteristic, difficulty))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        version: i32,
        hasPrecalculatedData: bool,
        levelID: *mut crate::System::String,
        songName: *mut crate::System::String,
        songSubName: *mut crate::System::String,
        songAuthorName: *mut crate::System::String,
        allMappers: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        allLighters: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        beatsPerMinute: f32,
        integratedLufs: f32,
        songTimeOffset: f32,
        previewStartTime: f32,
        previewDuration: f32,
        songDuration: f32,
        contentRating: PlayerSensitivityFlag,
        previewMediaData: *mut IPreviewMediaData,
        beatmapBasicData: *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
            crate::System::ValueTuple_2<*mut BeatmapCharacteristicSO, BeatmapDifficulty>,
            *mut BeatmapBasicData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    version,
                    hasPrecalculatedData,
                    levelID,
                    songName,
                    songSubName,
                    songAuthorName,
                    allMappers,
                    allLighters,
                    beatsPerMinute,
                    integratedLufs,
                    songTimeOffset,
                    previewStartTime,
                    previewDuration,
                    songDuration,
                    contentRating,
                    previewMediaData,
                    beatmapBasicData,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _GetBeatmapKeys_b__23_0(
        &mut self,
        entry: crate::System::ValueTuple_2<
            *mut BeatmapCharacteristicSO,
            BeatmapDifficulty,
        >,
    ) -> quest_hook::libil2cpp::Result<BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: BeatmapKey = __cordl_object
            .invoke("<GetBeatmapKeys>b__23_0", (entry))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        version: i32,
        hasPrecalculatedData: bool,
        levelID: *mut crate::System::String,
        songName: *mut crate::System::String,
        songSubName: *mut crate::System::String,
        songAuthorName: *mut crate::System::String,
        allMappers: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        allLighters: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        beatsPerMinute: f32,
        integratedLufs: f32,
        songTimeOffset: f32,
        previewStartTime: f32,
        previewDuration: f32,
        songDuration: f32,
        contentRating: PlayerSensitivityFlag,
        previewMediaData: *mut IPreviewMediaData,
        beatmapBasicData: *mut crate::System::Collections::Generic::IReadOnlyDictionary_2<
            crate::System::ValueTuple_2<*mut BeatmapCharacteristicSO, BeatmapDifficulty>,
            *mut BeatmapBasicData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    version,
                    hasPrecalculatedData,
                    levelID,
                    songName,
                    songSubName,
                    songAuthorName,
                    allMappers,
                    allLighters,
                    beatsPerMinute,
                    integratedLufs,
                    songTimeOffset,
                    previewStartTime,
                    previewDuration,
                    songDuration,
                    contentRating,
                    previewMediaData,
                    beatmapBasicData,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevel")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
