#[cfg(feature = "BeatmapDifficultyMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultyMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDifficultyMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDifficultyMethods => ""
    ."BeatmapDifficultyMethods"
);
#[cfg(feature = "BeatmapDifficultyMethods")]
impl std::ops::Deref for BeatmapDifficultyMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl std::ops::DerefMut for BeatmapDifficultyMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl BeatmapDifficultyMethods {}
#[cfg(feature = "BeatmapDifficultyMethods")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDifficultyMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
