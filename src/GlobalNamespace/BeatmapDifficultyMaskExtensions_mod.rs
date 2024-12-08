#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDifficultyMaskExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDifficultyMaskExtensions
    => ""."BeatmapDifficultyMaskExtensions"
);
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {}
#[cfg(feature = "BeatmapDifficultyMaskExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDifficultyMaskExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
