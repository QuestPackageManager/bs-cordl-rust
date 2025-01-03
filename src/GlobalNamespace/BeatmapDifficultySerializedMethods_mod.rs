#[cfg(feature = "BeatmapDifficultySerializedMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultySerializedMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDifficultySerializedMethods => ""
    ."BeatmapDifficultySerializedMethods"
);
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    pub const kDifficultyEasySerializedName: &'static str = "Easy";
    pub const kDifficultyExpertPlusNameSerializedLegacy: &'static str = "Expert+";
    pub const kDifficultyExpertPlusSerializedName: &'static str = "ExpertPlus";
    pub const kDifficultyExpertSerializedName: &'static str = "Expert";
    pub const kDifficultyHardSerializedName: &'static str = "Hard";
    pub const kDifficultyNormalSerializedName: &'static str = "Normal";
    pub const kDifficultyUnknownSerializedName: &'static str = "Unknown";
    pub fn BeatmapDifficultyFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        difficulty: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::BeatmapDifficulty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BeatmapDifficultyFromSerializedName", (name, difficulty))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName(
        difficulty: crate::GlobalNamespace::BeatmapDifficulty,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializedName", (difficulty))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultySerializedMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
