#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataObstaclesAndBombsTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform => ""
    ."BeatmapDataObstaclesAndBombsTransform"
);
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    #[cfg(feature = "BeatmapDataObstaclesAndBombsTransform+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform___c__DisplayClass0_0;
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
