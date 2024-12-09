#[cfg(feature = "BeatmapDataZenModeTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataZenModeTransform {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataZenModeTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataZenModeTransform =>
    ""."BeatmapDataZenModeTransform"
);
#[cfg(feature = "BeatmapDataZenModeTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataZenModeTransform {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataZenModeTransform")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataZenModeTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataZenModeTransform")]
impl crate::GlobalNamespace::BeatmapDataZenModeTransform {}
#[cfg(feature = "BeatmapDataZenModeTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataZenModeTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
