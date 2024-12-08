#[cfg(feature = "BeatmapTypeConverters")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapTypeConverters {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapTypeConverters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapTypeConverters => ""."BeatmapTypeConverters"
);
#[cfg(feature = "BeatmapTypeConverters")]
impl std::ops::Deref for BeatmapTypeConverters {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl std::ops::DerefMut for BeatmapTypeConverters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl BeatmapTypeConverters {}
#[cfg(feature = "BeatmapTypeConverters")]
impl quest_hook::libil2cpp::ObjectType for BeatmapTypeConverters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
