#[cfg(feature = "BeatmapLevelChecksums")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelChecksums {
    pub beatmapLevelChecksum: *mut crate::System::String,
    pub coverImageChecksum: *mut crate::System::String,
    pub songAudioClipChecksum: *mut crate::System::String,
    pub audioDataAssetChecksum: *mut crate::System::String,
    pub difficultyBeatmapsChecksums: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums,
    >,
}
#[cfg(feature = "BeatmapLevelChecksums")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for BeatmapLevelChecksums => ""."BeatmapLevelChecksums"
);
#[cfg(feature = "BeatmapLevelChecksums")]
unsafe impl quest_hook::libil2cpp::ThisArgument for BeatmapLevelChecksums {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BeatmapLevelChecksums")]
impl BeatmapLevelChecksums {
    #[cfg(feature = "BeatmapLevelChecksums+DifficultyBeatmapChecksums")]
    pub type DifficultyBeatmapChecksums = crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums;
    #[cfg(feature = "BeatmapLevelChecksums+__c")]
    pub type __c = crate::GlobalNamespace::BeatmapLevelChecksums___c;
    #[cfg(feature = "BeatmapLevelChecksums+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::GlobalNamespace::BeatmapLevelChecksums___c__DisplayClass7_0;
    #[cfg(feature = "BeatmapLevelChecksums+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::GlobalNamespace::BeatmapLevelChecksums___c__DisplayClass8_0;
    pub fn GetOrAddDifficultyBeatmapChecksums(
        &mut self,
        beatmapDifficulty: *mut crate::System::String,
        beatmapCharacteristic: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums,
    > {
        let __cordl_ret: crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetOrAddDifficultyBeatmapChecksums",
            (beatmapDifficulty, beatmapCharacteristic),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        other: BeatmapLevelChecksums,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetDifficultyBeatmapChecksums(
        &mut self,
        beatmapDifficulty: *mut crate::System::String,
        beatmapCharacteristic: *mut crate::System::String,
        beatmapAssetChecksum: *mut crate::System::String,
        lightshowAssetChecksum: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapLevelChecksums+DifficultyBeatmapChecksums")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BeatmapLevelChecksums_DifficultyBeatmapChecksums {
    pub beatmapDifficulty: *mut crate::System::String,
    pub beatmapCharacteristic: *mut crate::System::String,
    pub beatmapAssetChecksum: *mut crate::System::String,
    pub lightshowAssetChecksum: *mut crate::System::String,
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
    pub fn _ctor_String_String_String_String0(
        &mut self,
        beatmapDifficulty: *mut crate::System::String,
        beatmapCharacteristic: *mut crate::System::String,
        beatmapAssetChecksum: *mut crate::System::String,
        lightshowAssetChecksum: *mut crate::System::String,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor_BeatmapLevelChecksums_DifficultyBeatmapChecksums1(
        &mut self,
        other: crate::GlobalNamespace::BeatmapLevelChecksums_DifficultyBeatmapChecksums,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (other),
        )?;
        Ok(__cordl_ret)
    }
}
