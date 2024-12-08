#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEventItemConverter {
    __cordl_parent: BeatToTimeConverterProvider,
    pub _basicEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::BasicEvent,
    >,
    pub _specialEventsFilter: *mut crate::BeatmapDataLoaderVersion4::SpecialEventsFilter,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::BasicEventItemConverter => "BeatmapDataLoaderVersion4"
    ."BasicEventItemConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BasicEventItemConverter {
    type Target = BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::BasicEventItemConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
impl crate::BeatmapDataLoaderVersion4::BasicEventItemConverter {
    pub fn Convert(
        &mut self,
        index: *mut crate::BeatmapSaveDataVersion4::BeatIndex,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapEventData = __cordl_object
            .invoke("Convert", (index))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        basicEvents: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::BasicEvent,
        >,
        specialEventsFilter: *mut crate::BeatmapDataLoaderVersion4::SpecialEventsFilter,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (basicEvents, specialEventsFilter, bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        basicEvents: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::BeatmapSaveDataVersion4::BasicEvent,
        >,
        specialEventsFilter: *mut crate::BeatmapDataLoaderVersion4::SpecialEventsFilter,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (basicEvents, specialEventsFilter, bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::BasicEventItemConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}