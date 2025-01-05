#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BpmEventConverter {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatToTimeConverter,
    >,
    pub _songFrequency: i32,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::BpmEventConverter =>
    "BeatmapDataLoaderVersion4"."BpmEventConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatToTimeConverter>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
impl crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    pub fn Convert(
        &mut self,
        bpmData: quest_hook::libil2cpp::Gc<crate::BeatmapLevelSaveDataVersion4::BpmData>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        > = __cordl_object.invoke("Convert", (bpmData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        songFrequency: i32,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (songFrequency, bpmTimeProcessor))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        songFrequency: i32,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (songFrequency, bpmTimeProcessor))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BpmEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::BpmEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
