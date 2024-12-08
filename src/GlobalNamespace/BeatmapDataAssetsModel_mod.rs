#[cfg(feature = "BeatmapDataAssetsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataAssetsModel {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataAssetsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataAssetsModel => ""
    ."BeatmapDataAssetsModel"
);
#[cfg(feature = "BeatmapDataAssetsModel")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataAssetsModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataAssetsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl crate::GlobalNamespace::BeatmapDataAssetsModel {}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataAssetsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
