#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexFilterConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::IndexFilterConverter
    => "BeatmapDataLoaderVersion4"."IndexFilterConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl crate::BeatmapDataLoaderVersion4::IndexFilterConverter {}
#[cfg(feature = "BeatmapDataLoaderVersion4+IndexFilterConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::IndexFilterConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
