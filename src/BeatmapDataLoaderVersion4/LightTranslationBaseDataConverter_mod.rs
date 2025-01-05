#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationBaseDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationBaseDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationBaseDataConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::LightTranslationBaseDataConverter =>
    "BeatmapDataLoaderVersion4"."LightTranslationBaseDataConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationBaseDataConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion4::LightTranslationBaseDataConverter {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationBaseDataConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion4::LightTranslationBaseDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationBaseDataConverter")]
impl crate::BeatmapDataLoaderVersion4::LightTranslationBaseDataConverter {
    pub fn Convert(
        beat: f32,
        lightTranslationEvent: crate::BeatmapSaveDataVersion4::LightTranslationEvent,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightTranslationBaseData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightTranslationBaseData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (beat, lightTranslationEvent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationBaseDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::LightTranslationBaseDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
