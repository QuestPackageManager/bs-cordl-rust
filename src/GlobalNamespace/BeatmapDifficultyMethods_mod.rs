#[cfg(feature = "BeatmapDifficultyMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultyMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDifficultyMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDifficultyMethods => ""
    ."BeatmapDifficultyMethods"
);
#[cfg(feature = "BeatmapDifficultyMethods")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultyMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultyMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl crate::GlobalNamespace::BeatmapDifficultyMethods {
    pub const kDefaultDifficultyNjs: f32 = 10f32;
    pub const kExpertDifficultyNjs: f32 = 12f32;
    pub const kExpertPlusDifficultyNjs: f32 = 16f32;
    pub const kFastNotesNjs: f32 = 20f32;
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultyMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
