#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataObstaclesMergingTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapDataObstaclesMergingTransform => ""
    ."BeatmapDataObstaclesMergingTransform"
);
#[cfg(feature = "BeatmapDataObstaclesMergingTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataObstaclesMergingTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CanBeMerged(
        firstObstacle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
        secondObstacle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObstacleData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanBeMerged", (firstObstacle, secondObstacle))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTransformedData(
        beatmapData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IReadonlyBeatmapData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyBeatmapData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTransformedData", (beatmapData))?;
        Ok(__cordl_ret.into())
    }
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
