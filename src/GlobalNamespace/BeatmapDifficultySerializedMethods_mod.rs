#[cfg(feature = "BeatmapDifficultySerializedMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultySerializedMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDifficultySerializedMethods => ""
    ."BeatmapDifficultySerializedMethods"
);
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl std::ops::Deref for BeatmapDifficultySerializedMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl std::ops::DerefMut for BeatmapDifficultySerializedMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl BeatmapDifficultySerializedMethods {
    pub const kDifficultyEasySerializedName: &'static str = "Easy";
    pub const kDifficultyExpertPlusNameSerializedLegacy: &'static str = "Expert+";
    pub const kDifficultyExpertPlusSerializedName: &'static str = "ExpertPlus";
    pub const kDifficultyExpertSerializedName: &'static str = "Expert";
    pub const kDifficultyHardSerializedName: &'static str = "Hard";
    pub const kDifficultyNormalSerializedName: &'static str = "Normal";
    pub const kDifficultyUnknownSerializedName: &'static str = "Unknown";
}
#[cfg(feature = "BeatmapDifficultySerializedMethods")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDifficultySerializedMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}