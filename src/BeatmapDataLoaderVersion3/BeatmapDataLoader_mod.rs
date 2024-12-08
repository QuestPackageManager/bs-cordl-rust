#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BasicEventConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
    pub _specialEventsFilter: *mut crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/BasicEventConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    pub fn Convert(
        &mut self,
        basicEventSaveData: *mut crate::BeatmapSaveDataVersion3::BasicEventData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapEventData = __cordl_object
            .invoke("Convert", (basicEventSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
        specialEventsFilter: *mut crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor, specialEventsFilter))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
        specialEventsFilter: *mut crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor, specialEventsFilter))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BeatmapDataItemConverter {
    __cordl_parent: crate::System::Object,
    pub _bpmTimeProcessor: *mut BpmTimeProcessor,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/BeatmapDataItemConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    pub fn BeatToTime(&mut self, beat: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("BeatToTime", (beat))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion3::BeatmapDataLoader =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    pub const kDefaultNumberOfLines: i32 = 4i32;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+RotationEventConverter"
    )]
    pub type RotationEventConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_RotationEventConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
    pub type BurstSliderConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
    pub type WaypointConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter"
    )]
    pub type IntVfxEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader___c__DisplayClass4_0;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BeatmapDataItemConverter"
    )]
    pub type BeatmapDataItemConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
    )]
    pub type LightColoBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor"
    )]
    pub type IntVfxBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
    pub type ObstacleConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
    )]
    pub type LightColorEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
    )]
    pub type LightTranslationEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
    pub type ColorNoteConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
    )]
    pub type LightTranslationBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
    pub type BombNoteConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor"
    )]
    pub type FloatVfxBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
    )]
    pub type LightRotationBaseDataConvertor = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
    pub type BpmEventConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter"
    )]
    pub type FloatVfxEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
    pub type SliderConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
    pub type SpecialEventsFilter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader___c__DisplayClass3_0;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
    )]
    pub type LightRotationEventBoxConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter;
    #[cfg(
        feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter"
    )]
    pub type ColorBoostEventConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
    pub type IndexFilterConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter;
    #[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BasicEventConverter")]
    pub type BasicEventConverter = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BasicEventConverter;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BombNoteConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/BombNoteConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    pub fn Convert(
        &mut self,
        bombNoteSaveData: *mut crate::BeatmapSaveDataVersion3::BombNoteData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectData = __cordl_object
            .invoke("Convert", (bombNoteSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BombNoteConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BombNoteConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BpmEventConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/BpmEventConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    pub fn Convert(
        &mut self,
        bpmChangeEventSaveData: *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
    ) -> quest_hook::libil2cpp::Result<*mut BPMChangeBeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BPMChangeBeatmapEventData = __cordl_object
            .invoke("Convert", (bpmChangeEventSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BpmEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BpmEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_BurstSliderConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/BurstSliderConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        sliderSaveData: *mut crate::BeatmapSaveDataVersion3::BurstSliderData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectData = __cordl_object
            .invoke("Convert", (sliderSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+BurstSliderConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BurstSliderConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ColorBoostEventConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/ColorBoostEventConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        colorBoostEventSaveData: *mut crate::BeatmapSaveDataVersion3::ColorBoostEventData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapEventData = __cordl_object
            .invoke("Convert", (colorBoostEventSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorBoostEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorBoostEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ColorNoteConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/ColorNoteConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    pub fn Convert(
        &mut self,
        colorNoteSaveData: *mut crate::BeatmapSaveDataVersion3::ColorNoteData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectData = __cordl_object
            .invoke("Convert", (colorNoteSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ColorNoteConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ColorNoteConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_FloatVfxBaseDataConvertor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/FloatVfxBaseDataConvertor"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxBaseDataConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_FloatVfxEventBoxConverter {
    __cordl_parent: crate::System::Object,
    pub _fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/FloatVfxEventBoxConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    pub fn Convert(
        &mut self,
        saveData: *mut crate::BeatmapSaveDataVersion3::FxEventBox,
        lightGroup: *mut ILightGroup,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapEventDataBox> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapEventDataBox = __cordl_object
            .invoke("Convert", (saveData, lightGroup))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fxEventsCollection))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fxEventsCollection))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+FloatVfxEventBoxConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_FloatVfxEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_IndexFilterConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/IndexFilterConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IndexFilterConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IndexFilterConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_IntVfxBaseDataConvertor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/IntVfxBaseDataConvertor"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxBaseDataConvertor")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_IntVfxEventBoxConverter {
    __cordl_parent: crate::System::Object,
    pub _fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/IntVfxEventBoxConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    pub fn _ctor(
        &mut self,
        fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fxEventsCollection))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        saveData: *mut crate::BeatmapSaveDataVersion3::FxEventBox,
        lightGroup: *mut ILightGroup,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapEventDataBox> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapEventDataBox = __cordl_object
            .invoke("Convert", (saveData, lightGroup))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        fxEventsCollection: *mut crate::BeatmapSaveDataVersion3::FxEventsCollection,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fxEventsCollection))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+IntVfxEventBoxConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_IntVfxEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightColoBaseDataConvertor {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/LightColoBaseDataConvertor"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColoBaseDataConvertor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColoBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightColorEventBoxConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/LightColorEventBoxConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightColorEventBoxConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightColorEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightRotationBaseDataConvertor {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/LightRotationBaseDataConvertor"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationBaseDataConvertor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightRotationEventBoxConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/LightRotationEventBoxConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightRotationEventBoxConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightRotationEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightTranslationBaseDataConvertor {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/LightTranslationBaseDataConvertor"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationBaseDataConvertor"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationBaseDataConvertor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_LightTranslationEventBoxConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/LightTranslationEventBoxConverter"
);
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+LightTranslationEventBoxConverter"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_LightTranslationEventBoxConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_ObstacleConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/ObstacleConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        obstacleSaveData: *mut crate::BeatmapSaveDataVersion3::ObstacleData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectData = __cordl_object
            .invoke("Convert", (obstacleSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+ObstacleConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_ObstacleConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+RotationEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_RotationEventConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+RotationEventConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_RotationEventConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/RotationEventConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+RotationEventConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_RotationEventConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+RotationEventConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_RotationEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+RotationEventConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_RotationEventConverter {
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        rotationEventSaveData: *mut crate::BeatmapSaveDataVersion3::RotationEventData,
    ) -> quest_hook::libil2cpp::Result<*mut SpawnRotationBeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut SpawnRotationBeatmapEventData = __cordl_object
            .invoke("Convert", (rotationEventSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+RotationEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_RotationEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SliderConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/SliderConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    pub fn Convert(
        &mut self,
        sliderSaveData: *mut crate::BeatmapSaveDataVersion3::SliderData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectData = __cordl_object
            .invoke("Convert", (sliderSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SliderConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SliderConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_SpecialEventsFilter {
    __cordl_parent: crate::System::Object,
    pub _eventTypesToFilter: *mut crate::System::Collections::Generic::HashSet_1<
        crate::BeatmapSaveDataCommon::BeatmapEventType,
    >,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/SpecialEventsFilter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    pub fn _ctor(
        &mut self,
        basicEventTypesWithKeywords: *mut crate::BeatmapSaveDataCommon::BasicEventTypesWithKeywords,
        environmentKeywords: *mut EnvironmentKeywords,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (basicEventTypesWithKeywords, environmentKeywords))?;
        Ok(__cordl_ret)
    }
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
        environmentKeywords: *mut EnvironmentKeywords,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (basicEventTypesWithKeywords, environmentKeywords))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+SpecialEventsFilter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_SpecialEventsFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader_WaypointConverter {
    __cordl_parent: crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter,
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter =>
    "BeatmapDataLoaderVersion3"."BeatmapDataLoader/WaypointConverter"
);
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl std::ops::Deref
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    type Target = crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_BeatmapDataItemConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl std::ops::DerefMut
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    pub fn _ctor(
        &mut self,
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_ret)
    }
    pub fn Convert(
        &mut self,
        waypointSaveData: *mut crate::BeatmapSaveDataVersion3::WaypointData,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapObjectData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapObjectData = __cordl_object
            .invoke("Convert", (waypointSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bpmTimeProcessor: *mut BpmTimeProcessor,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpmTimeProcessor))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion3+BeatmapDataLoader+WaypointConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion3::BeatmapDataLoader_WaypointConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
