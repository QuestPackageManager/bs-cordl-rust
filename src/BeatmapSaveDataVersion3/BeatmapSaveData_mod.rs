#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveData {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::System::String,
    pub bpmEvents: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
    >,
    pub rotationEvents: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::RotationEventData,
    >,
    pub colorNotes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::ColorNoteData,
    >,
    pub bombNotes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::BombNoteData,
    >,
    pub obstacles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::ObstacleData,
    >,
    pub sliders: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::SliderData,
    >,
    pub burstSliders: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::BurstSliderData,
    >,
    pub waypoints: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::WaypointData,
    >,
    pub basicBeatmapEvents: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::BasicEventData,
    >,
    pub colorBoostBeatmapEvents: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::ColorBoostEventData,
    >,
    pub lightColorEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
    >,
    pub lightRotationEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::LightRotationEventBoxGroup,
    >,
    pub lightTranslationEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
    >,
    pub vfxEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion3::FxEventBoxGroup,
    >,
    pub _fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
    pub basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
    pub useNormalEventsAsCompatibleEvents: bool,
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::BeatmapSaveData =>
    "BeatmapSaveDataVersion3"."BeatmapSaveData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::BeatmapSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::BeatmapSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
impl crate::BeatmapSaveDataVersion3::BeatmapSaveData {
    pub const kCurrentVersion: &'static str = "3.3.0";
    pub fn New(
        bpmEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
        >,
        rotationEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::RotationEventData,
        >,
        colorNotes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ColorNoteData,
        >,
        bombNotes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BombNoteData,
        >,
        obstacles: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ObstacleData,
        >,
        sliders: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::SliderData,
        >,
        burstSliders: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BurstSliderData,
        >,
        waypoints: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::WaypointData,
        >,
        basicBeatmapEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BasicEventData,
        >,
        colorBoostBeatmapEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ColorBoostEventData,
        >,
        lightColorEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
        >,
        lightRotationEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightRotationEventBoxGroup,
        >,
        lightTranslationEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
        >,
        vfxEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::FxEventBoxGroup,
        >,
        fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
        basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        useNormalEventsAsCompatibleEvents: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    bpmEvents,
                    rotationEvents,
                    colorNotes,
                    bombNotes,
                    obstacles,
                    sliders,
                    burstSliders,
                    waypoints,
                    basicBeatmapEvents,
                    colorBoostBeatmapEvents,
                    lightColorEventBoxGroups,
                    lightRotationEventBoxGroups,
                    lightTranslationEventBoxGroups,
                    vfxEventBoxGroups,
                    fxEventsCollection,
                    basicEventTypesWithKeywords,
                    useNormalEventsAsCompatibleEvents,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpmEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
        >,
        rotationEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::RotationEventData,
        >,
        colorNotes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ColorNoteData,
        >,
        bombNotes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BombNoteData,
        >,
        obstacles: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ObstacleData,
        >,
        sliders: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::SliderData,
        >,
        burstSliders: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BurstSliderData,
        >,
        waypoints: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::WaypointData,
        >,
        basicBeatmapEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BasicEventData,
        >,
        colorBoostBeatmapEvents: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ColorBoostEventData,
        >,
        lightColorEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
        >,
        lightRotationEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightRotationEventBoxGroup,
        >,
        lightTranslationEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
        >,
        vfxEventBoxGroups: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::FxEventBoxGroup,
        >,
        fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
        basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        useNormalEventsAsCompatibleEvents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    bpmEvents,
                    rotationEvents,
                    colorNotes,
                    bombNotes,
                    obstacles,
                    sliders,
                    burstSliders,
                    waypoints,
                    basicBeatmapEvents,
                    colorBoostBeatmapEvents,
                    lightColorEventBoxGroups,
                    lightRotationEventBoxGroups,
                    lightTranslationEventBoxGroups,
                    vfxEventBoxGroups,
                    fxEventsCollection,
                    basicEventTypesWithKeywords,
                    useNormalEventsAsCompatibleEvents,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion3::BeatmapSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
