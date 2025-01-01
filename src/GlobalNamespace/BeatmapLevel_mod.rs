#[cfg(feature = "BeatmapLevel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: i32,
    pub hasPrecalculatedData: bool,
    pub levelID: *mut quest_hook::libil2cpp::Il2CppString,
    pub songName: *mut quest_hook::libil2cpp::Il2CppString,
    pub songSubName: *mut quest_hook::libil2cpp::Il2CppString,
    pub songAuthorName: *mut quest_hook::libil2cpp::Il2CppString,
    pub allMappers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub allLighters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub beatsPerMinute: f32,
    pub integratedLufs: f32,
    pub songTimeOffset: f32,
    pub previewStartTime: f32,
    pub previewDuration: f32,
    pub songDuration: f32,
    pub contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub previewMediaData: *mut crate::GlobalNamespace::IPreviewMediaData,
    pub _beatmapBasicDatas: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::System::ValueTuple_2<
            *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
        *mut crate::GlobalNamespace::BeatmapBasicData,
    >,
    pub _characteristicsCache: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
    >,
    pub _beatmapKeysCache: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::BeatmapKey,
    >,
}
#[cfg(feature = "BeatmapLevel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevel => ""
    ."BeatmapLevel"
);
#[cfg(feature = "BeatmapLevel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevel")]
impl crate::GlobalNamespace::BeatmapLevel {
    pub const kInvalidVersion: i32 = -1i32;
    #[cfg(feature = "BeatmapLevel+__c")]
    pub type __c = crate::GlobalNamespace::BeatmapLevel___c;
    #[cfg(feature = "BeatmapLevel+__c__DisplayClass23_0")]
    pub type __c__DisplayClass23_0 = crate::GlobalNamespace::BeatmapLevel___c__DisplayClass23_0;
    pub fn AddBeatmapBasicData(
        &mut self,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
        beatmapBasicData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapBasicData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddBeatmapBasicData",
                (characteristic, difficulty, beatmapBasicData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapKeys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::BeatmapKey,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::BeatmapKey,
            >,
        > = __cordl_object.invoke("GetBeatmapKeys", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacteristics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            >,
        > = __cordl_object.invoke("GetCharacteristics", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColorScheme(
        &mut self,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = __cordl_object.invoke("GetColorScheme", (characteristic, difficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDifficulties(
        &mut self,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        > = __cordl_object.invoke("GetDifficulties", (characteristic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDifficultyBeatmapData(
        &mut self,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapBasicData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapBasicData,
        > = __cordl_object
            .invoke("GetDifficultyBeatmapData", (characteristic, difficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentName(
        &mut self,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::EnvironmentName = __cordl_object
            .invoke("GetEnvironmentName", (characteristic, difficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        version: i32,
        hasPrecalculatedData: bool,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allMappers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        allLighters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        beatsPerMinute: f32,
        integratedLufs: f32,
        songTimeOffset: f32,
        previewStartTime: f32,
        previewDuration: f32,
        songDuration: f32,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
        previewMediaData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPreviewMediaData,
        >,
        beatmapBasicData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::System::ValueTuple_2<
                    *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                *mut crate::GlobalNamespace::BeatmapBasicData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn RemoveBeatmapBasicData(
        &mut self,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveBeatmapBasicData", (characteristic, difficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn _GetBeatmapKeys_b__25_0(
        &mut self,
        entry: crate::System::ValueTuple_2<
            *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = __cordl_object
            .invoke("<GetBeatmapKeys>b__25_0", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        version: i32,
        hasPrecalculatedData: bool,
        levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        allMappers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        allLighters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        beatsPerMinute: f32,
        integratedLufs: f32,
        songTimeOffset: f32,
        previewStartTime: f32,
        previewDuration: f32,
        songDuration: f32,
        contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
        previewMediaData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPreviewMediaData,
        >,
        beatmapBasicData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::System::ValueTuple_2<
                    *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                *mut crate::GlobalNamespace::BeatmapBasicData,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapBasicData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                crate::System::ValueTuple_2<
                    *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                *mut crate::GlobalNamespace::BeatmapBasicData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                crate::System::ValueTuple_2<
                    *mut crate::GlobalNamespace::BeatmapCharacteristicSO,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                *mut crate::GlobalNamespace::BeatmapBasicData,
            >,
        > = __cordl_object.invoke("get_beatmapBasicData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapLevel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
