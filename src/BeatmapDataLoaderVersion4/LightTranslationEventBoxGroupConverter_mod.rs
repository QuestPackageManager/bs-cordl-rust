#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationEventBoxGroupConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationEventBoxGroupConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationEventBoxGroupConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::LightTranslationEventBoxGroupConverter =>
    "BeatmapDataLoaderVersion4"."LightTranslationEventBoxGroupConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationEventBoxGroupConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion4::LightTranslationEventBoxGroupConverter {
    type Target = crate::BeatmapDataLoaderVersion4::EventBoxGroupConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationEventBoxGroupConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion4::LightTranslationEventBoxGroupConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationEventBoxGroupConverter")]
impl crate::BeatmapDataLoaderVersion4::LightTranslationEventBoxGroupConverter {
    pub fn _ctor(
        &mut self,
        lightshowSaveData: *mut crate::BeatmapSaveDataVersion4::LightshowSaveData,
        lightGroups: *mut IEnvironmentLightGroups,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightshowSaveData, lightGroups))?;
        Ok(__cordl_ret)
    }
    pub fn ConvertEvents(
        &mut self,
        eventBox: crate::BeatmapSaveDataVersion4::EventBox,
        indexFilter: *mut IndexFilter,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapEventDataBox> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapEventDataBox = __cordl_object
            .invoke("ConvertEvents", (eventBox, indexFilter))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        lightshowSaveData: *mut crate::BeatmapSaveDataVersion4::LightshowSaveData,
        lightGroups: *mut IEnvironmentLightGroups,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightshowSaveData, lightGroups))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+LightTranslationEventBoxGroupConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::LightTranslationEventBoxGroupConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
