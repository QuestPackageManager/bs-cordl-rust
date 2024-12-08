#[cfg(feature = "BeatmapDataTransformHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataTransformHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataTransformHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataTransformHelper =>
    ""."BeatmapDataTransformHelper"
);
#[cfg(feature = "BeatmapDataTransformHelper")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataTransformHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataTransformHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataTransformHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataTransformHelper")]
impl crate::GlobalNamespace::BeatmapDataTransformHelper {}
#[cfg(feature = "BeatmapDataTransformHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataTransformHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
