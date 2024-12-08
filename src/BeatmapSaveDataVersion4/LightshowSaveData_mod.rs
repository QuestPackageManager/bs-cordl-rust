#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightshowSaveData {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::System::String,
    pub waypoints: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatmapSaveDataVersion4::BeatmapBeatIndex,
    >,
    pub waypointsData: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::Waypoint,
    >,
    pub basicEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatmapSaveDataVersion4::BeatIndex,
    >,
    pub basicEventsData: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::BasicEvent,
    >,
    pub colorBoostEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatmapSaveDataVersion4::BeatIndex,
    >,
    pub colorBoostEventsData: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::ColorBoostEvent,
    >,
    pub eventBoxGroups: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::BeatmapSaveDataVersion4::EventBoxGroup,
    >,
    pub indexFilters: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::IndexFilter,
    >,
    pub lightColorEventBoxes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::LightColorEventBox,
    >,
    pub lightColorEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::LightColorEvent,
    >,
    pub lightRotationEventBoxes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::LightRotationEventBox,
    >,
    pub lightRotationEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::LightRotationEvent,
    >,
    pub lightTranslationEventBoxes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::LightTranslationEventBox,
    >,
    pub lightTranslationEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::LightTranslationEvent,
    >,
    pub fxEventBoxes: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::FxEventBox,
    >,
    pub floatFxEvents: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::BeatmapSaveDataVersion4::FloatFxEvent,
    >,
    pub basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
    pub useNormalEventsAsCompatibleEvents: bool,
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion4::LightshowSaveData =>
    "BeatmapSaveDataVersion4"."LightshowSaveData"
);
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion4::LightshowSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion4::LightshowSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl crate::BeatmapSaveDataVersion4::LightshowSaveData {
    pub const kCurrentVersion: &'static str = "4.0.0";
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion4+LightshowSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion4::LightshowSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
