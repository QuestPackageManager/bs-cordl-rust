#[cfg(feature = "BeatmapLevel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: i32,
    pub hasPrecalculatedData: bool,
    pub levelID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songSubName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub songAuthorName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub allMappers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub allLighters: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub beatsPerMinute: f32,
    pub integratedLufs: f32,
    pub songTimeOffset: f32,
    pub previewStartTime: f32,
    pub previewDuration: f32,
    pub songDuration: f32,
    pub contentRating: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub previewMediaData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPreviewMediaData,
    >,
    pub _beatmapBasicDatas: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::System::ValueTuple_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapBasicData>,
        >,
    >,
    pub _characteristicsCache: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
        >,
    >,
    pub _beatmapKeysCache: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::GlobalNamespace::BeatmapKey>,
    >,
}
#[cfg(feature = "BeatmapLevel")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatmapLevel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLevel";
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapBasicData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("AddBeatmapBasicData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddBeatmapBasicData", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (characteristic, difficulty, beatmapBasicData))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::BeatmapKey,
                            >,
                        >,
                        0usize,
                    >("GetBeatmapKeys")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetBeatmapKeys", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::BeatmapKey,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCharacteristics(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                                >,
                            >,
                        >,
                        0usize,
                    >("GetCharacteristics")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetCharacteristics", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapCharacteristicSO,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                        ),
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorScheme>,
                        2usize,
                    >("GetColorScheme")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetColorScheme", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ColorScheme,
        > = unsafe { method.invoke_unchecked(self, (characteristic, difficulty))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapCharacteristicSO,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                crate::GlobalNamespace::BeatmapDifficulty,
                            >,
                        >,
                        1usize,
                    >("GetDifficulties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetDifficulties", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::GlobalNamespace::BeatmapDifficulty,
            >,
        > = unsafe { method.invoke_unchecked(self, (characteristic))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapBasicData,
                        >,
                        2usize,
                    >("GetDifficultyBeatmapData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetDifficultyBeatmapData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapBasicData,
        > = unsafe { method.invoke_unchecked(self, (characteristic, difficulty))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnvironmentName(
        &mut self,
        characteristic: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapCharacteristicSO,
        >,
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentName> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                        ),
                        crate::GlobalNamespace::EnvironmentName,
                        2usize,
                    >("GetEnvironmentName")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetEnvironmentName", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::EnvironmentName = unsafe {
            method.invoke_unchecked(self, (characteristic, difficulty))?
        };
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
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        allLighters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
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
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                    >,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapBasicData>,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("RemoveBeatmapBasicData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "RemoveBeatmapBasicData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (characteristic, difficulty))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _GetBeatmapKeys_b__25_0(
        &mut self,
        entry: crate::System::ValueTuple_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapCharacteristicSO>,
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BeatmapKey> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::ValueTuple_2<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapCharacteristicSO,
                            >,
                            crate::GlobalNamespace::BeatmapDifficulty,
                        >),
                        crate::GlobalNamespace::BeatmapKey,
                        1usize,
                    >("<GetBeatmapKeys>b__25_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "<GetBeatmapKeys>b__25_0", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatmapKey = unsafe {
            method.invoke_unchecked(self, (entry))?
        };
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
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        allLighters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
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
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                    >,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapBasicData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        quest_hook::libil2cpp::Il2CppString,
                                    >,
                                >,
                            >,
                            f32,
                            f32,
                            f32,
                            f32,
                            f32,
                            f32,
                            crate::GlobalNamespace::PlayerSensitivityFlag,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IPreviewMediaData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::Dictionary_2<
                                    crate::System::ValueTuple_2<
                                        quest_hook::libil2cpp::Gc<
                                            crate::GlobalNamespace::BeatmapCharacteristicSO,
                                        >,
                                        crate::GlobalNamespace::BeatmapDifficulty,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapBasicData,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        17usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 17usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatmapBasicData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                    >,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapBasicData>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                                crate::System::ValueTuple_2<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                                    >,
                                    crate::GlobalNamespace::BeatmapDifficulty,
                                >,
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapBasicData,
                                >,
                            >,
                        >,
                        0usize,
                    >("get_beatmapBasicData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_beatmapBasicData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<
                crate::System::ValueTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::BeatmapCharacteristicSO,
                    >,
                    crate::GlobalNamespace::BeatmapDifficulty,
                >,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapBasicData>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
