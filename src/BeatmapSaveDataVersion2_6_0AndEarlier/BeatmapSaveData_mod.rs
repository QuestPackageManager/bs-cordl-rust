#[cfg(feature = "cordl_class_BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapSaveData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _events: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        >,
    >,
    pub _notes: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData>,
    >,
    pub _sliders: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
        >,
    >,
    pub _waypoints: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
        >,
    >,
    pub _obstacles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
        >,
    >,
    pub _specialEventsKeywordFilters: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
    >,
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatmapSaveDataVersion2_6_0AndEarlier";
    const CLASS_NAME: &'static str = "BeatmapSaveData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl std::ops::DerefMut
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    pub const kCurrentVersion: &'static str = "2.6.0";
    pub fn ConvertBeatmapSaveDataPreV2_5_0Inline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ConvertBeatmapSaveDataPreV2_5_0Inline")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertBeatmapSaveDataPreV2_5_0Inline", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        events: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
            >,
        >,
        notes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
            >,
        >,
        sliders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
            >,
        >,
        waypoints: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
            >,
        >,
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
            >,
        >,
        specialEventsKeywordFilters: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        events: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
            >,
        >,
        notes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
            >,
        >,
        sliders: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
            >,
        >,
        waypoints: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
            >,
        >,
        obstacles: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
            >,
        >,
        specialEventsKeywordFilters: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        events,
                        notes,
                        sliders,
                        waypoints,
                        obstacles,
                        specialEventsKeywordFilters,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_events(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
                            >,
                        >,
                        0usize,
                    >("get_events")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_events", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_notes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
                            >,
                        >,
                        0usize,
                    >("get_notes")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_notes", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_obstacles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
                            >,
                        >,
                        0usize,
                    >("get_obstacles")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_obstacles", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_sliders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
                            >,
                        >,
                        0usize,
                    >("get_sliders")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_sliders", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_specialEventsKeywordFilters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
                        >,
                        0usize,
                    >("get_specialEventsKeywordFilters")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_specialEventsKeywordFilters", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_version")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_version", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_waypoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Gc<
                                crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
                            >,
                        >,
                        0usize,
                    >("get_waypoints")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_waypoints", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatmapSaveDataVersion2_6_0AndEarlier+BeatmapSaveData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
