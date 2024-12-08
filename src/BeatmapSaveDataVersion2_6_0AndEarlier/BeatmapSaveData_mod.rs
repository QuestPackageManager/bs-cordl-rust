#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveData {
    __cordl_parent: crate::System::Object,
    pub _version: *mut crate::System::String,
    pub _events: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
    >,
    pub _notes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
    >,
    pub _sliders: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
    >,
    pub _waypoints: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
    >,
    pub _obstacles: *mut crate::System::Collections::Generic::List_1<
        *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
    >,
    pub _specialEventsKeywordFilters: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."BeatmapSaveData"
);
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl std::ops::DerefMut
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    pub const kCurrentVersion: &'static str = "2.6.0";
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_version", ())?;
        Ok(__cordl_ret)
    }
    pub fn ConvertBeatmapSaveDataPreV2_5_0Inline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConvertBeatmapSaveDataPreV2_5_0Inline", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_events(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        > = __cordl_object.invoke("get_events", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_notes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
        > = __cordl_object.invoke("get_notes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_specialEventsKeywordFilters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData = __cordl_object
            .invoke("get_specialEventsKeywordFilters", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        events: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        >,
        notes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
        >,
        sliders: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
        >,
        waypoints: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
        >,
        obstacles: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
        >,
        specialEventsKeywordFilters: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    events,
                    notes,
                    sliders,
                    waypoints,
                    obstacles,
                    specialEventsKeywordFilters,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_waypoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
        > = __cordl_object.invoke("get_waypoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_obstacles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
        > = __cordl_object.invoke("get_obstacles", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
        > = __cordl_object.invoke("get_sliders", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        events: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        >,
        notes: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
        >,
        sliders: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
        >,
        waypoints: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
        >,
        obstacles: *mut crate::System::Collections::Generic::List_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
        >,
        specialEventsKeywordFilters: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    events,
                    notes,
                    sliders,
                    waypoints,
                    obstacles,
                    specialEventsKeywordFilters,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
