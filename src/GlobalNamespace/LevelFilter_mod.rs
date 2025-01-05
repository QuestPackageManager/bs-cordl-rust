#[cfg(feature = "LevelFilter")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LevelFilter {
    pub songOwned: bool,
    pub songNotOwned: bool,
    pub songUnplayed: bool,
    pub difficulties: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub songPacks: crate::GlobalNamespace::SongPackMask,
    pub characteristicSerializedName: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub minBpm: f32,
    pub maxBpm: f32,
    pub sensitivity: crate::GlobalNamespace::PlayerSensitivityFlag,
    pub searchText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub limitIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
    >,
}
#[cfg(feature = "LevelFilter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelFilter => ""."LevelFilter"
);
#[cfg(feature = "LevelFilter")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::GlobalNamespace::LevelFilter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LevelFilter")]
impl crate::GlobalNamespace::LevelFilter {
    pub fn CleanText(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CleanText", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterLevelByText(
        levels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
        searchTerms: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterLevelByText", (levels, searchTerms))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterLevelsAsync_IEnumerable_1_1(
        beatmapLevels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
        filter: crate::GlobalNamespace::LevelFilter,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        levelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEntitlementModel,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        *mut crate::GlobalNamespace::BeatmapLevel,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        *mut crate::GlobalNamespace::BeatmapLevel,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FilterLevelsAsync",
                (beatmapLevels, filter, playerDataModel, levelsModel, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterLevelsAsync_Il2CppArray0(
        packs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::BeatmapLevelPack,
            >,
        >,
        filter: crate::GlobalNamespace::LevelFilter,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
        levelsModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEntitlementModel,
        >,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        *mut crate::GlobalNamespace::BeatmapLevel,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        *mut crate::GlobalNamespace::BeatmapLevel,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FilterLevelsAsync",
                (packs, filter, playerDataModel, levelsModel, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWithoutFilter(
        filter: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::LevelFilter>,
        ignoreFilterBySongs: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsWithoutFilter", (filter, ignoreFilterBySongs))?;
        Ok(__cordl_ret.into())
    }
    pub fn _FilterLevelByText_g__CalculateMatchScore_15_1(
        levelString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        searchTerms: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "<FilterLevelByText>g__CalculateMatchScore|15_1",
                (levelString, searchTerms),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _FilterLevelsAsync_g__HasPlayedAnyDifficulty_14_0(
        level: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "<FilterLevelsAsync>g__HasPlayedAnyDifficulty|14_0",
                (level, playerDataModel),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _FilterLevelsAsync_g__MatchesCharacteristic_14_1(
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        characteristicFilter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "<FilterLevelsAsync>g__MatchesCharacteristic|14_1",
                (beatmapLevel, characteristicFilter),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _FilterLevelsAsync_g__MatchesDifficulty_14_2(
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
        difficultyFilter: crate::GlobalNamespace::BeatmapDifficultyMask,
        characteristicFilter: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "<FilterLevelsAsync>g__MatchesDifficulty|14_2",
                (beatmapLevel, difficultyFilter, characteristicFilter),
            )?;
        Ok(__cordl_ret.into())
    }
}
