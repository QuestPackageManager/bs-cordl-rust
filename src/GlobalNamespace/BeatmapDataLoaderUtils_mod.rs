#[cfg(feature = "BeatmapDataLoaderUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoaderUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataLoaderUtils => ""
    ."BeatmapDataLoaderUtils"
);
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataLoaderUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataLoaderUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl crate::GlobalNamespace::BeatmapDataLoaderUtils {}
#[cfg(feature = "BeatmapDataLoaderUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataLoaderUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
