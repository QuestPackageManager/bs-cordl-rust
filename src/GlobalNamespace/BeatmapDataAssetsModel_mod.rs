#[cfg(feature = "BeatmapDataAssetsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataAssetsModel {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataAssetsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDataAssetsModel => ""."BeatmapDataAssetsModel"
);
#[cfg(feature = "BeatmapDataAssetsModel")]
impl std::ops::Deref for BeatmapDataAssetsModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl std::ops::DerefMut for BeatmapDataAssetsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl BeatmapDataAssetsModel {}
#[cfg(feature = "BeatmapDataAssetsModel")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDataAssetsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
