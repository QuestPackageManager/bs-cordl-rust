#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorBaseDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBaseDataConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorBaseDataConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::LightColorBaseDataConverter =>
    "BeatmapDataLoaderVersion4"."LightColorBaseDataConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorBaseDataConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::LightColorBaseDataConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorBaseDataConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion4::LightColorBaseDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorBaseDataConverter")]
impl crate::BeatmapDataLoaderVersion4::LightColorBaseDataConverter {}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightColorBaseDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::LightColorBaseDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
