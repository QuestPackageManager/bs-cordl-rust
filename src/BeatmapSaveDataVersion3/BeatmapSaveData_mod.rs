#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub bpmEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
        >,
    >,
    pub rotationEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::RotationEventData,
        >,
    >,
    pub colorNotes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ColorNoteData,
        >,
    >,
    pub bombNotes: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BombNoteData,
        >,
    >,
    pub obstacles: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ObstacleData,
        >,
    >,
    pub sliders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::SliderData,
        >,
    >,
    pub burstSliders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BurstSliderData,
        >,
    >,
    pub waypoints: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::WaypointData,
        >,
    >,
    pub basicBeatmapEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::BasicEventData,
        >,
    >,
    pub colorBoostBeatmapEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::ColorBoostEventData,
        >,
    >,
    pub lightColorEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
        >,
    >,
    pub lightRotationEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightRotationEventBoxGroup,
        >,
    >,
    pub lightTranslationEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
        >,
    >,
    pub vfxEventBoxGroups: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion3::FxEventBoxGroup,
        >,
    >,
    pub _fxEventsCollection: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion3::FxEventsCollection,
    >,
    pub basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
    >,
    pub useNormalEventsAsCompatibleEvents: bool,
}
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::BeatmapSaveData =>
    "BeatmapSaveDataVersion3"."BeatmapSaveData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+BeatmapSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::BeatmapSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        bpmEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
            >,
        >,
        rotationEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::RotationEventData,
            >,
        >,
        colorNotes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::ColorNoteData,
            >,
        >,
        bombNotes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BombNoteData,
            >,
        >,
        obstacles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::ObstacleData,
            >,
        >,
        sliders: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::SliderData,
            >,
        >,
        burstSliders: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BurstSliderData,
            >,
        >,
        waypoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::WaypointData,
            >,
        >,
        basicBeatmapEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BasicEventData,
            >,
        >,
        colorBoostBeatmapEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::ColorBoostEventData,
            >,
        >,
        lightColorEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
            >,
        >,
        lightRotationEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::LightRotationEventBoxGroup,
            >,
        >,
        lightTranslationEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
            >,
        >,
        vfxEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::FxEventBoxGroup,
            >,
        >,
        fxEventsCollection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
        basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        >,
        useNormalEventsAsCompatibleEvents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        bpmEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
            >,
        >,
        rotationEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::RotationEventData,
            >,
        >,
        colorNotes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::ColorNoteData,
            >,
        >,
        bombNotes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BombNoteData,
            >,
        >,
        obstacles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::ObstacleData,
            >,
        >,
        sliders: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::SliderData,
            >,
        >,
        burstSliders: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BurstSliderData,
            >,
        >,
        waypoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::WaypointData,
            >,
        >,
        basicBeatmapEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::BasicEventData,
            >,
        >,
        colorBoostBeatmapEvents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::ColorBoostEventData,
            >,
        >,
        lightColorEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::LightColorEventBoxGroup,
            >,
        >,
        lightRotationEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::LightRotationEventBoxGroup,
            >,
        >,
        lightTranslationEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::LightTranslationEventBoxGroup,
            >,
        >,
        vfxEventBoxGroups: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::BeatmapSaveDataVersion3::FxEventBoxGroup,
            >,
        >,
        fxEventsCollection: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion3::FxEventsCollection,
        >,
        basicEventTypesWithKeywords: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        >,
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
        Ok(__cordl_ret.into())
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
