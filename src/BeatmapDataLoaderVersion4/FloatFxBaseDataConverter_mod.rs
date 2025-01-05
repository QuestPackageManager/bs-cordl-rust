#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxBaseDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter => "BeatmapDataLoaderVersion4"
    ."FloatFxBaseDataConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    pub fn Convert(
        beat: f32,
        floatFxEvent: crate::BeatmapSaveDataVersion4::FloatFxEvent,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBaseData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBaseData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Convert", (beat, floatFxEvent))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+FloatFxBaseDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::FloatFxBaseDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
