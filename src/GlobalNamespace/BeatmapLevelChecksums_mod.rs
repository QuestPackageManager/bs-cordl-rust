#[cfg(feature = "BeatmapLevelChecksums")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BeatmapLevelChecksums {
    pub beatmapLevelChecksum: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub coverImageChecksum: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub songAudioClipChecksum: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub audioDataAssetChecksum: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub difficultyBeatmapsChecksums: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums,
    >,
}
#[cfg(feature = "BeatmapLevelChecksums")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelChecksums => ""
    ."BeatmapLevelChecksums"
);
#[cfg(feature = "BeatmapLevelChecksums")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapLevelChecksums {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelChecksums")]
impl crate::GlobalNamespace::BeatmapLevelChecksums {
    #[cfg(feature = "BeatmapLevelChecksums+DifficultyBeatmapChecksums")]
    pub type DifficultyBeatmapChecksums = crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums;
    pub fn GetOrAddDifficultyBeatmapChecksums(
        &mut self,
        beatmapDifficulty: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums,
    > {
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetOrAddDifficultyBeatmapChecksums",
            (beatmapDifficulty, beatmapCharacteristic),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDifficultyBeatmapChecksums(
        &mut self,
        beatmapDifficulty: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapAssetChecksum: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        lightshowAssetChecksum: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDifficultyBeatmapChecksums",
            (
                beatmapDifficulty,
                beatmapCharacteristic,
                beatmapAssetChecksum,
                lightshowAssetChecksum,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        other: crate::GlobalNamespace::BeatmapLevelChecksums,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLevelChecksums+DifficultyBeatmapChecksums")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BeatmapLevelChecksums_DifficultyBeatmapChecksums {
    pub beatmapDifficulty: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub beatmapCharacteristic: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub beatmapAssetChecksum: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub lightshowAssetChecksum: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "BeatmapLevelChecksums+DifficultyBeatmapChecksums")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums => ""
    ."BeatmapLevelChecksums/DifficultyBeatmapChecksums"
);
#[cfg(feature = "BeatmapLevelChecksums+DifficultyBeatmapChecksums")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelChecksums+DifficultyBeatmapChecksums")]
impl crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums {
    pub fn _ctor_BeatmapLevelChecksums_DifficultyBeatmapChecksums1(
        &mut self,
        other: crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_Gc_Gc_Gc0(
        &mut self,
        beatmapDifficulty: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapCharacteristic: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        beatmapAssetChecksum: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        lightshowAssetChecksum: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                beatmapDifficulty,
                beatmapCharacteristic,
                beatmapAssetChecksum,
                lightshowAssetChecksum,
            ),
        )?;
        Ok(__cordl_ret.into())
    }
}
