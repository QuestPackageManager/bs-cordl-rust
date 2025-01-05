#[cfg(feature = "MovementHistoryRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct MovementHistoryRecorder {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _averagingValueRecorer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AveragingValueRecorder,
    >,
    pub _increaseSpeed: f32,
    pub _decreaseSpeed: f32,
    pub _accum: f32,
}
#[cfg(feature = "MovementHistoryRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MovementHistoryRecorder => ""
    ."MovementHistoryRecorder"
);
#[cfg(feature = "MovementHistoryRecorder")]
impl std::ops::Deref for crate::GlobalNamespace::MovementHistoryRecorder {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MovementHistoryRecorder")]
impl std::ops::DerefMut for crate::GlobalNamespace::MovementHistoryRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MovementHistoryRecorder")]
impl crate::GlobalNamespace::MovementHistoryRecorder {
    pub fn AddMovement(
        &mut self,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddMovement", (distance))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        averageWindowDuration: f32,
        historyValuesPerSecond: f32,
        increaseSpeed: f32,
        decreaseSpeed: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_averagingValueRecorer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AveragingValueRecorder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::AveragingValueRecorder,
        > = __cordl_object.invoke("get_averagingValueRecorer", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MovementHistoryRecorder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MovementHistoryRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
