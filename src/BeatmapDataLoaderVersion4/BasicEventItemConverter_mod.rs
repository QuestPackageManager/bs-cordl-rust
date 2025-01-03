#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEventItemConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverter,
    pub _basicEvents: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::BeatmapSaveDataVersion4::BasicEvent>,
    >,
    pub _specialEventsFilter: quest_hook::libil2cpp::Gc<
        crate::BeatmapDataLoaderVersion4::SpecialEventsFilter,
    >,
    pub _lightEventConverter: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IBeatmapLightEventConverter,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion4::BasicEventItemConverter => "BeatmapDataLoaderVersion4"
    ."BasicEventItemConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+BasicEventItemConverter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BasicEventItemConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverter;
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
        index: quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion4::BeatIndex>,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapEventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Convert", (index, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        basicEvents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::BeatmapSaveDataVersion4::BasicEvent,
            >,
        >,
        specialEventsFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapDataLoaderVersion4::SpecialEventsFilter,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (basicEvents, specialEventsFilter, bpmTimeProcessor, lightEventConverter),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        basicEvents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::BeatmapSaveDataVersion4::BasicEvent,
            >,
        >,
        specialEventsFilter: quest_hook::libil2cpp::Gc<
            crate::BeatmapDataLoaderVersion4::SpecialEventsFilter,
        >,
        bpmTimeProcessor: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BpmTimeProcessor,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (basicEvents, specialEventsFilter, bpmTimeProcessor, lightEventConverter),
            )?;
        Ok(__cordl_ret.into())
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
