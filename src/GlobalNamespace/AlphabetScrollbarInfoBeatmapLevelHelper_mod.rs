#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AlphabetScrollbarInfoBeatmapLevelHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper => ""
    ."AlphabetScrollbarInfoBeatmapLevelHelper"
);
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl std::ops::Deref
for crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    pub const kFirstAlphabet: &'static str = "A";
    pub const kLastAlphabet: &'static str = "Z";
    pub const kMaxCharactersCount: i32 = 28i32;
    pub const kNonAlphabetChar: char = '#';
    pub fn CreateData(
        beatmapLevels: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
            >,
        >,
        sortBeatmapLevels: bool,
        sortedBeatmapLevels: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::IReadOnlyList_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::AlphabetScrollInfo_Data,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::AlphabetScrollInfo_Data,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateData",
                (beatmapLevels, sortBeatmapLevels, sortedBeatmapLevels),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNumericOrSpecial(
        comparedChar: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNumericOrSpecial", (comparedChar))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
