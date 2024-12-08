#[cfg(feature = "FloatFxBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxBeatmapEventData {
    __cordl_parent: FxBeatmapEventData,
    pub _value_k__BackingField: f32,
    pub _easeType_k__BackingField: EaseType,
}
#[cfg(feature = "FloatFxBeatmapEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FloatFxBeatmapEventData => ""."FloatFxBeatmapEventData"
);
#[cfg(feature = "FloatFxBeatmapEventData")]
impl std::ops::Deref for FloatFxBeatmapEventData {
    type Target = FxBeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBeatmapEventData")]
impl std::ops::DerefMut for FloatFxBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBeatmapEventData")]
impl FloatFxBeatmapEventData {
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
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
    pub fn New(
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
        value: f32,
        easeType: EaseType,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, groupId, elementId, usePreviousEventValue, value, easeType),
            )?;
        Ok(__cordl_object)
    }
    pub fn UpdateBy(
        &mut self,
        other: *mut FloatFxBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBy", (other))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
        value: f32,
        easeType: EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_time, groupId, elementId, usePreviousEventValue, value, easeType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_easeType(&mut self) -> quest_hook::libil2cpp::Result<EaseType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: EaseType = __cordl_object.invoke("get_easeType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_easeType(
        &mut self,
        value: EaseType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_easeType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_value(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_value", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FloatFxBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType for FloatFxBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}