#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BasicEventConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverterProvider,
    pub _specialEventsFilter: *mut crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter,
    pub _canUseEnvironmentEventsAndShouldLoadDynamicEvents: bool,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter =>
    "BeatmapDataLoaderVersion2_6_0AndEarlier"."BeatmapDataLoader/BasicEventConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    pub fn Convert(
        &mut self,
        e: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapEventData = __cordl_object
            .invoke("Convert", (e))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
        specialEventsFilter: *mut crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter,
        canUseEnvironmentEventsAndShouldLoadDynamicEvents: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    bpmTimeProcessor,
                    specialEventsFilter,
                    canUseEnvironmentEventsAndShouldLoadDynamicEvents,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
        specialEventsFilter: *mut crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter,
        canUseEnvironmentEventsAndShouldLoadDynamicEvents: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    bpmTimeProcessor,
                    specialEventsFilter,
                    canUseEnvironmentEventsAndShouldLoadDynamicEvents,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader =>
    "BeatmapDataLoaderVersion2_6_0AndEarlier"."BeatmapDataLoader"
);
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    pub const kDefaultNumberOfLines: i32 = 4i32;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+BasicEventConverter"
    )]
    pub type BasicEventConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_BasicEventConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
    )]
    pub type ColorNoteConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
    )]
    pub type ObstacleConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
    )]
    pub type SliderConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
    )]
    pub type SpecialEventsFilter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
    )]
    pub type WaypointConverter = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+__c")]
    pub type __c = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader___c;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+__c__DisplayClass1_0"
    )]
    pub type __c__DisplayClass1_0 = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader___c__DisplayClass1_0;
    #[cfg(
        feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+__c__DisplayClass2_0"
    )]
    pub type __c__DisplayClass2_0 = crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader___c__DisplayClass2_0;
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
#[cfg(feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ColorNoteConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverterProvider,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter =>
    "BeatmapDataLoaderVersion2_6_0AndEarlier"."BeatmapDataLoader/ColorNoteConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    pub fn Convert(
        &mut self,
        n: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::NoteData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapObjectData = __cordl_object
            .invoke("Convert", (n))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ColorNoteConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ColorNoteConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ObstacleConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverterProvider,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter =>
    "BeatmapDataLoaderVersion2_6_0AndEarlier"."BeatmapDataLoader/ObstacleConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    pub fn Convert(
        &mut self,
        o: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::ObstacleData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::ObstacleData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::ObstacleData = __cordl_object
            .invoke("Convert", (o))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+ObstacleConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_ObstacleConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SliderConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverterProvider,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter =>
    "BeatmapDataLoaderVersion2_6_0AndEarlier"."BeatmapDataLoader/SliderConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    pub fn Convert(
        &mut self,
        s: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::SliderData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::SliderData = __cordl_object
            .invoke("Convert", (s))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SliderConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SliderConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SpecialEventsFilter {
    __cordl_parent: crate::System::Object,
    pub _eventTypesToFilter: *mut crate::System::Collections::Generic::HashSet_1<
        crate::BeatmapSaveDataCommon::BeatmapEventType,
    >,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter =>
    "BeatmapDataLoaderVersion2_6_0AndEarlier"."BeatmapDataLoader/SpecialEventsFilter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
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
        basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
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
        basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::SpecialEventKeywordFiltersData,
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
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+SpecialEventsFilter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_SpecialEventsFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_WaypointConverter {
    __cordl_parent: crate::GlobalNamespace::BeatToTimeConverterProvider,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter =>
    "BeatmapDataLoaderVersion2_6_0AndEarlier"."BeatmapDataLoader/WaypointConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    type Target = crate::GlobalNamespace::BeatToTimeConverterProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    pub fn Convert(
        &mut self,
        waypointSaveData: *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::WaypointData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::WaypointData = __cordl_object
            .invoke("Convert", (waypointSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut crate::GlobalNamespace::IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion2_6_0AndEarlier+BeatmapDataLoader+WaypointConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion2_6_0AndEarlier::BeatmapDataLoader_WaypointConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
