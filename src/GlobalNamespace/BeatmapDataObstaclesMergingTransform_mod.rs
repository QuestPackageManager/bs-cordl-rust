#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataObstaclesMergingTransform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDataObstaclesMergingTransform => ""
    ."BeatmapDataObstaclesMergingTransform"
);
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    #[cfg(feature = "BeatmapDataObstaclesMergingTransform+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform___c__DisplayClass0_0;
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
