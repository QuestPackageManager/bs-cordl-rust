#[cfg(feature = "AveragingValueRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct AveragingValueRecorder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _averageWindowDuration: f32,
    pub _historyValuesPerSecond: f32,
    pub _historyValuesCount: i32,
    pub _averageWindowValues: *mut crate::System::Collections::Generic::Queue_1<
        crate::GlobalNamespace::AveragingValueRecorder_AverageValueData,
    >,
    pub _historyValues: *mut crate::System::Collections::Generic::Queue_1<f32>,
    pub _time: f32,
    pub _historyTime: f32,
    pub _averageValue: f32,
    pub _averageWindowValuesDuration: f32,
    pub _lastValue: f32,
}
#[cfg(feature = "AveragingValueRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AveragingValueRecorder => ""
    ."AveragingValueRecorder"
);
#[cfg(feature = "AveragingValueRecorder")]
impl std::ops::Deref for crate::GlobalNamespace::AveragingValueRecorder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AveragingValueRecorder")]
impl std::ops::DerefMut for crate::GlobalNamespace::AveragingValueRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AveragingValueRecorder")]
impl crate::GlobalNamespace::AveragingValueRecorder {
    #[cfg(feature = "AveragingValueRecorder+AverageValueData")]
    pub type AverageValueData = crate::GlobalNamespace::AveragingValueRecorder_AverageValueData;
    pub fn GetAverageValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetAverageValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHistoryValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Queue_1<f32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Queue_1<f32> = __cordl_object
            .invoke("GetHistoryValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLastValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetLastValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        averageWindowDuration: f32,
        historyWindowDuration: f32,
        historyValuesPerSecond: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (averageWindowDuration, historyWindowDuration, historyValuesPerSecond),
            )?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
        value: f32,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (value, deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        averageWindowDuration: f32,
        historyWindowDuration: f32,
        historyValuesPerSecond: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (averageWindowDuration, historyWindowDuration, historyValuesPerSecond),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "AveragingValueRecorder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AveragingValueRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AveragingValueRecorder_AverageValueData {
    pub _value_k__BackingField: f32,
    pub _time_k__BackingField: f32,
}
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AveragingValueRecorder_AverageValueData => ""
    ."AveragingValueRecorder/AverageValueData"
);
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "AveragingValueRecorder+AverageValueData")]
impl crate::GlobalNamespace::AveragingValueRecorder_AverageValueData {
    pub fn _ctor(
        &mut self,
        value: f32,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, _cordl_time),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_time",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_time(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_time",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_value(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_value",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
