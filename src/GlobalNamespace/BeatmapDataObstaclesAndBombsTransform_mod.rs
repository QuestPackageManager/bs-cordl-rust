#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataObstaclesAndBombsTransform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDataObstaclesAndBombsTransform => ""
    ."BeatmapDataObstaclesAndBombsTransform"
);
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl std::ops::Deref for BeatmapDataObstaclesAndBombsTransform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl std::ops::DerefMut for BeatmapDataObstaclesAndBombsTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl BeatmapDataObstaclesAndBombsTransform {
    #[cfg(feature = "BeatmapDataObstaclesAndBombsTransform+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::GlobalNamespace::BeatmapDataObstaclesAndBombsTransform___c__DisplayClass0_0;
}
#[cfg(feature = "BeatmapDataObstaclesAndBombsTransform")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDataObstaclesAndBombsTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
