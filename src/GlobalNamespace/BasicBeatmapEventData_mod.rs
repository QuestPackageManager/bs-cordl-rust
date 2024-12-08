#[cfg(feature = "BasicBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicBeatmapEventData {
    __cordl_parent: BeatmapEventData,
    pub _sameTypeIndex_k__BackingField: i32,
    pub basicBeatmapEventType: BasicBeatmapEventType,
    pub value: i32,
    pub floatValue: f32,
}
#[cfg(feature = "BasicBeatmapEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BasicBeatmapEventData => ""."BasicBeatmapEventData"
);
#[cfg(feature = "BasicBeatmapEventData")]
impl std::ops::Deref for BasicBeatmapEventData {
    type Target = BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapEventData")]
impl std::ops::DerefMut for BasicBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BasicBeatmapEventData")]
impl BasicBeatmapEventData {
    pub fn RecalculateSameTypeIndexFromPreviousEvent(
        &mut self,
        basicBeatmapEventData: *mut BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecalculateSameTypeIndexFromPreviousEvent",
                (basicBeatmapEventData),
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
    pub fn get_nextSameTypeEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BasicBeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BasicBeatmapEventData = __cordl_object
            .invoke("get_nextSameTypeEventData", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        basicBeatmapEventType: BasicBeatmapEventType,
        value: i32,
        floatValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, basicBeatmapEventType, value, floatValue))?;
        Ok(__cordl_ret)
    }
    pub fn set_sameTypeIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sameTypeIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_sameTypeIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sameTypeIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetFirstSameTypeIndex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFirstSameTypeIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_previousSameTypeEventData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BasicBeatmapEventData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BasicBeatmapEventData = __cordl_object
            .invoke("get_previousSameTypeEventData", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        basicBeatmapEventType: BasicBeatmapEventType,
        value: i32,
        floatValue: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, basicBeatmapEventType, value, floatValue),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BasicBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType for BasicBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
