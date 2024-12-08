#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputMetrics")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputMetrics {
    pub _maxNumDevices_k__BackingField: i32,
    pub _currentNumDevices_k__BackingField: i32,
    pub _maxStateSizeInBytes_k__BackingField: i32,
    pub _currentStateSizeInBytes_k__BackingField: i32,
    pub _currentControlCount_k__BackingField: i32,
    pub _currentLayoutCount_k__BackingField: i32,
    pub _totalEventBytes_k__BackingField: i32,
    pub _totalEventCount_k__BackingField: i32,
    pub _totalUpdateCount_k__BackingField: i32,
    pub _totalEventProcessingTime_k__BackingField: f64,
    pub _totalEventLagTime_k__BackingField: f64,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputMetrics")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::LowLevel::InputMetrics
    => "UnityEngine.InputSystem.LowLevel"."InputMetrics"
);
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputMetrics")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputMetrics {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputMetrics")]
impl crate::UnityEngine::InputSystem::LowLevel::InputMetrics {
    pub fn get_totalEventProcessingTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalEventProcessingTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_currentNumDevices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_currentNumDevices",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_currentNumDevices(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_currentNumDevices",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_currentStateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_currentStateSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_averageLagTimePerEvent(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_averageLagTimePerEvent",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_currentControlCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_currentControlCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_maxStateSizeInBytes(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maxStateSizeInBytes",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_currentLayoutCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_currentLayoutCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_totalUpdateCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_totalUpdateCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_totalEventCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalEventCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_totalEventBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalEventBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_totalEventProcessingTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_totalEventProcessingTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_currentStateSizeInBytes(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_currentStateSizeInBytes",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_totalEventBytes(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_totalEventBytes",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_maxNumDevices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxNumDevices",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_averageEventBytesPerFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_averageEventBytesPerFrame",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_currentLayoutCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_currentLayoutCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_totalUpdateCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalUpdateCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_totalEventLagTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_totalEventLagTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_totalEventLagTime(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_totalEventLagTime",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_maxStateSizeInBytes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxStateSizeInBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_averageProcessingTimePerEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_averageProcessingTimePerEvent",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_currentControlCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_currentControlCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_totalEventCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_totalEventCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_maxNumDevices(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maxNumDevices",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
