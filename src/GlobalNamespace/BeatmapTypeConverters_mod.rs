#[cfg(feature = "BeatmapTypeConverters")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapTypeConverters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapTypeConverters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapTypeConverters => ""
    ."BeatmapTypeConverters"
);
#[cfg(feature = "BeatmapTypeConverters")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapTypeConverters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapTypeConverters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl crate::GlobalNamespace::BeatmapTypeConverters {}
#[cfg(feature = "BeatmapTypeConverters")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapTypeConverters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
