#[cfg(feature = "BpmTimeProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct BpmTimeProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _bpmChangeDataList: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData,
    >,
    pub currentBpmChangesDataIdx: i32,
}
#[cfg(feature = "BpmTimeProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BpmTimeProcessor => ""
    ."BpmTimeProcessor"
);
#[cfg(feature = "BpmTimeProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::BpmTimeProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::BpmTimeProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl crate::GlobalNamespace::BpmTimeProcessor {
    #[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
    pub type BpmChangeData = crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData;
    #[cfg(feature = "BpmTimeProcessor+__c")]
    pub type __c = crate::GlobalNamespace::BpmTimeProcessor___c;
    pub fn ConvertBeatToTime(
        &mut self,
        beat: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ConvertBeatToTime", (beat))?;
        Ok(__cordl_ret)
    }
    pub fn New_AudioSaveData2(
        audioSaveData: *mut crate::BeatmapLevelSaveDataVersion4::AudioSaveData,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (audioSaveData))?;
        Ok(__cordl_object)
    }
    pub fn New_f32_IReadOnlyList_1_0(
        startBpm: f32,
        events: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startBpm, events))?;
        Ok(__cordl_object)
    }
    pub fn New_f32_IReadOnlyList_1_1(
        startBpm: f32,
        bpmEventsSaveData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (startBpm, bpmEventsSaveData))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AudioSaveData2(
        &mut self,
        audioSaveData: *mut crate::BeatmapLevelSaveDataVersion4::AudioSaveData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (audioSaveData))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_IReadOnlyList_1_0(
        &mut self,
        startBpm: f32,
        events: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (startBpm, events))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_IReadOnlyList_1_1(
        &mut self,
        startBpm: f32,
        bpmEventsSaveData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::BeatmapSaveDataVersion3::BpmChangeEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (startBpm, bpmEventsSaveData))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BpmTimeProcessor")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BpmTimeProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BpmTimeProcessor_BpmChangeData {
    pub bpmChangeStartTime: f32,
    pub bpmChangeStartBpmTime: f32,
    pub bpm: f32,
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BpmTimeProcessor_BpmChangeData
    => ""."BpmTimeProcessor/BpmChangeData"
);
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BpmTimeProcessor+BpmChangeData")]
impl crate::GlobalNamespace::BpmTimeProcessor_BpmChangeData {
    pub fn _ctor(
        &mut self,
        bpmChangeStartTime: f32,
        bpmChangeStartBpmTime: f32,
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bpmChangeStartTime, bpmChangeStartBpmTime, bpm),
        )?;
        Ok(__cordl_ret)
    }
}
