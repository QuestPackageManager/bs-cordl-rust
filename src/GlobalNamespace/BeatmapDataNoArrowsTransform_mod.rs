#[cfg(feature = "BeatmapDataNoArrowsTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataNoArrowsTransform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDataNoArrowsTransform => ""
    ."BeatmapDataNoArrowsTransform"
);
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl std::ops::Deref for BeatmapDataNoArrowsTransform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl std::ops::DerefMut for BeatmapDataNoArrowsTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl BeatmapDataNoArrowsTransform {}
#[cfg(feature = "BeatmapDataNoArrowsTransform")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDataNoArrowsTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
