#[cfg(feature = "BeatmapDataLoaderVersion4+SpecialEventsFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct SpecialEventsFilter {
    __cordl_parent: crate::System::Object,
    pub _eventTypesToFilter: *mut crate::System::Collections::Generic::HashSet_1<
        crate::BeatmapSaveDataCommon::BeatmapEventType,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+SpecialEventsFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::SpecialEventsFilter
    => "BeatmapDataLoaderVersion4"."SpecialEventsFilter"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+SpecialEventsFilter")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::SpecialEventsFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+SpecialEventsFilter")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::SpecialEventsFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+SpecialEventsFilter")]
impl crate::BeatmapDataLoaderVersion4::SpecialEventsFilter {
    pub fn IsEventValid(
        &mut self,
        basicBeatmapEventType: crate::BeatmapSaveDataCommon::BeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsEventValid", (basicBeatmapEventType))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        environmentKeywords: *mut crate::GlobalNamespace::EnvironmentKeywords,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (basicEventTypesWithKeywords, environmentKeywords))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        environmentKeywords: *mut crate::GlobalNamespace::EnvironmentKeywords,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (basicEventTypesWithKeywords, environmentKeywords))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+SpecialEventsFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::SpecialEventsFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
