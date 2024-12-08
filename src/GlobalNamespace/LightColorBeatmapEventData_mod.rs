#[cfg(feature = "LightColorBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBeatmapEventData {
    __cordl_parent: BeatmapEventData,
    pub groupId: i32,
    pub elementId: i32,
    pub usePreviousValue: bool,
    pub easeType: EaseType,
    pub _colorType_k__BackingField: EnvironmentColorType,
    pub _brightness_k__BackingField: f32,
    pub _strobeBeatFrequency_k__BackingField: i32,
    pub _strobeBrightness_k__BackingField: f32,
    pub _strobeFade_k__BackingField: bool,
}
#[cfg(feature = "LightColorBeatmapEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightColorBeatmapEventData => ""
    ."LightColorBeatmapEventData"
);
#[cfg(feature = "LightColorBeatmapEventData")]
impl std::ops::Deref for LightColorBeatmapEventData {
    type Target = BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventData")]
impl std::ops::DerefMut for LightColorBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventData")]
impl LightColorBeatmapEventData {
    pub fn get_colorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<EnvironmentColorType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: EnvironmentColorType = __cordl_object
            .invoke("get_colorType", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_strobeBrightness(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_strobeBrightness", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorType(
        &mut self,
        value: EnvironmentColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_brightness(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_brightness", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_strobeFade(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_strobeFade", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_strobeBeatFrequency(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_strobeBeatFrequency", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_strobeFade(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_strobeFade", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousValue: bool,
        easeType: EaseType,
        colorType: EnvironmentColorType,
        brightness: f32,
        strobeBeatFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    _cordl_time,
                    groupId,
                    elementId,
                    usePreviousValue,
                    easeType,
                    colorType,
                    brightness,
                    strobeBeatFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
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
    pub fn get_strobeBrightness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_strobeBrightness", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_strobeBeatFrequency(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_strobeBeatFrequency", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CopyColorDataFrom(
        &mut self,
        lightColorBeatmapEventData: *mut LightColorBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyColorDataFrom", (lightColorBeatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn DisableStrobe(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableStrobe", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_brightness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_brightness", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousValue: bool,
        easeType: EaseType,
        colorType: EnvironmentColorType,
        brightness: f32,
        strobeBeatFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_time,
                    groupId,
                    elementId,
                    usePreviousValue,
                    easeType,
                    colorType,
                    brightness,
                    strobeBeatFrequency,
                    strobeBrightness,
                    strobeFade,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightColorBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType for LightColorBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
