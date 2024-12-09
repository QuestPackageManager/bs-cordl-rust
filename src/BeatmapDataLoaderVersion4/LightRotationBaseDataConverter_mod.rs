#[cfg(feature = "BeatmapDataLoaderVersion4+LightRotationBaseDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationBaseDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightRotationBaseDataConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::LightRotationBaseDataConverter =>
    "BeatmapDataLoaderVersion4"."LightRotationBaseDataConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+LightRotationBaseDataConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion4::LightRotationBaseDataConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightRotationBaseDataConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion4::LightRotationBaseDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightRotationBaseDataConverter")]
impl crate::BeatmapDataLoaderVersion4::LightRotationBaseDataConverter {}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightRotationBaseDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::LightRotationBaseDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
