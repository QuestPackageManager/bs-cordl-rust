#[cfg(feature = "EventsTestGameplayManager")]
#[repr(C)]
#[derive(Debug)]
pub struct EventsTestGameplayManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _moveTime: bool,
    pub _spawnTestBox: bool,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _beatmapData: *mut BeatmapData,
    pub _audioTimeSource: *mut IAudioTimeSource,
    pub _basicBeatmapEventType: BasicBeatmapEventType,
    pub _floatValue: f32,
    pub _beatmapEventDataBoxGroupLists: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut BeatmapEventDataBoxGroupList,
    >,
    pub groupState: *mut quest_hook::libil2cpp::Il2CppArray<bool>,
    pub _beatmapEventTypeBindings: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::KeyCode,
        BasicBeatmapEventType,
    >,
    pub _intBindings: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::KeyCode,
        i32,
    >,
    pub _beatmapValuesBindings: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::KeyCode,
        i32,
    >,
    pub _floatValuesBindings: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::UnityEngine::KeyCode,
        f32,
    >,
    pub _rotatingLasers: bool,
}
#[cfg(feature = "EventsTestGameplayManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EventsTestGameplayManager => ""
    ."EventsTestGameplayManager"
);
#[cfg(feature = "EventsTestGameplayManager")]
impl std::ops::Deref for EventsTestGameplayManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager")]
impl std::ops::DerefMut for EventsTestGameplayManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager")]
impl EventsTestGameplayManager {
    pub const kNumberOfLightGroups: i32 = 20i32;
    #[cfg(feature = "EventsTestGameplayManager+__c__DisplayClass19_0")]
    pub type __c__DisplayClass19_0 = crate::GlobalNamespace::EventsTestGameplayManager___c__DisplayClass19_0;
    #[cfg(feature = "EventsTestGameplayManager+__c__DisplayClass20_0")]
    pub type __c__DisplayClass20_0 = crate::GlobalNamespace::EventsTestGameplayManager___c__DisplayClass20_0;
    #[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
    pub type MockBeatToTimeConverter = crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter;
    #[cfg(feature = "EventsTestGameplayManager+__c__DisplayClass18_0")]
    pub type __c__DisplayClass18_0 = crate::GlobalNamespace::EventsTestGameplayManager___c__DisplayClass18_0;
    pub fn AddEventsForLightGroup(
        &mut self,
        lightGroupId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddEventsForLightGroup", (lightGroupId))?;
        Ok(__cordl_ret)
    }
    pub fn AddInstantToggleEventsForLightGroup(
        &mut self,
        lightGroupId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInstantToggleEventsForLightGroup", (lightGroupId))?;
        Ok(__cordl_ret)
    }
    pub fn AddTestBox(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTestBox", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddToggleEventsForLightGroup(
        &mut self,
        lightGroupId: i32,
        color: EnvironmentColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToggleEventsForLightGroup", (lightGroupId, color))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
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
#[cfg(feature = "EventsTestGameplayManager")]
impl quest_hook::libil2cpp::ObjectType for EventsTestGameplayManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct EventsTestGameplayManager_MockBeatToTimeConverter {
    __cordl_parent: crate::System::Object,
    pub _bpm: f32,
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter => ""
    ."EventsTestGameplayManager/MockBeatToTimeConverter"
);
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl std::ops::Deref
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl std::ops::DerefMut
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
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
    pub fn New(bpm: f32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bpm))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bpm: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bpm))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "EventsTestGameplayManager+MockBeatToTimeConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EventsTestGameplayManager_MockBeatToTimeConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
