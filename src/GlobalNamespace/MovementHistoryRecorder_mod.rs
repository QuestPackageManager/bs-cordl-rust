#[cfg(feature = "MovementHistoryRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct MovementHistoryRecorder {
    __cordl_parent: crate::System::Object,
    pub _averagingValueRecorer: *mut AveragingValueRecorder,
    pub _increaseSpeed: f32,
    pub _decreaseSpeed: f32,
    pub _accum: f32,
}
#[cfg(feature = "MovementHistoryRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MovementHistoryRecorder => ""."MovementHistoryRecorder"
);
#[cfg(feature = "MovementHistoryRecorder")]
impl std::ops::Deref for MovementHistoryRecorder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MovementHistoryRecorder")]
impl std::ops::DerefMut for MovementHistoryRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MovementHistoryRecorder")]
impl MovementHistoryRecorder {
    pub fn AddMovement(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMovement", (distance))?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (deltaTime))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        averageWindowDuration: f32,
        historyValuesPerSecond: f32,
        increaseSpeed: f32,
        decreaseSpeed: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    averageWindowDuration,
                    historyValuesPerSecond,
                    increaseSpeed,
                    decreaseSpeed,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        averageWindowDuration: f32,
        historyValuesPerSecond: f32,
        increaseSpeed: f32,
        decreaseSpeed: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    averageWindowDuration,
                    historyValuesPerSecond,
                    increaseSpeed,
                    decreaseSpeed,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_averagingValueRecorer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut AveragingValueRecorder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut AveragingValueRecorder = __cordl_object
            .invoke("get_averagingValueRecorer", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MovementHistoryRecorder")]
impl quest_hook::libil2cpp::ObjectType for MovementHistoryRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
