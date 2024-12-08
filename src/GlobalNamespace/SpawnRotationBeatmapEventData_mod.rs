#[cfg(feature = "SpawnRotationBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnRotationBeatmapEventData {
    __cordl_parent: BeatmapEventData,
    pub _rotation_k__BackingField: f32,
    pub spawnRotationEventType: crate::GlobalNamespace::SpawnRotationBeatmapEventData_SpawnRotationEventType,
    pub _deltaRotation: f32,
}
#[cfg(feature = "SpawnRotationBeatmapEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SpawnRotationBeatmapEventData => ""
    ."SpawnRotationBeatmapEventData"
);
#[cfg(feature = "SpawnRotationBeatmapEventData")]
impl std::ops::Deref for SpawnRotationBeatmapEventData {
    type Target = BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationBeatmapEventData")]
impl std::ops::DerefMut for SpawnRotationBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationBeatmapEventData")]
impl SpawnRotationBeatmapEventData {
    #[cfg(feature = "SpawnRotationBeatmapEventData+SpawnRotationEventType")]
    pub type SpawnRotationEventType = crate::GlobalNamespace::SpawnRotationBeatmapEventData_SpawnRotationEventType;
    pub fn get_subtypeGroupIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeGroupIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_rotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_rotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_rotation(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        spawnRotationEventType: crate::GlobalNamespace::SpawnRotationBeatmapEventData_SpawnRotationEventType,
        deltaRotation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, spawnRotationEventType, deltaRotation))?;
        Ok(__cordl_ret)
    }
    pub fn Mirror(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mirror", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn RecalculateRotationFromPreviousEvent(
        &mut self,
        previousSpawnRotationBeatmapEventData: *mut SpawnRotationBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecalculateRotationFromPreviousEvent",
                (previousSpawnRotationBeatmapEventData),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetDefault(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapEventData = __cordl_object
            .invoke("GetDefault", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetFirstRotationEventRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFirstRotationEventRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        spawnRotationEventType: crate::GlobalNamespace::SpawnRotationBeatmapEventData_SpawnRotationEventType,
        deltaRotation: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, spawnRotationEventType, deltaRotation))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SpawnRotationBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType for SpawnRotationBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SpawnRotationBeatmapEventData+SpawnRotationEventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpawnRotationBeatmapEventData_SpawnRotationEventType {
    Early = 1i32,
    Late = 2i32,
}
#[cfg(feature = "SpawnRotationBeatmapEventData+SpawnRotationEventType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SpawnRotationBeatmapEventData_SpawnRotationEventType => ""
    ."SpawnRotationBeatmapEventData/SpawnRotationEventType"
);
