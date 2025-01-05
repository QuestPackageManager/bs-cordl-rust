#[cfg(feature = "BeatmapDataNoArrowsTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataNoArrowsTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataNoArrowsTransform =>
    ""."BeatmapDataNoArrowsTransform"
);
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
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
    pub fn _CreateTransformedData_g__ProcessData_0_0(
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<CreateTransformedData>g__ProcessData|0_0", (beatmapDataItem))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataNoArrowsTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
