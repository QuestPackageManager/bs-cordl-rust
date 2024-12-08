#[cfg(feature = "FxBeatmapEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct FxBeatmapEventData {
    __cordl_parent: BeatmapEventData,
    pub groupId: i32,
    pub elementId: i32,
    pub usePreviousEventValue: bool,
}
#[cfg(feature = "FxBeatmapEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FxBeatmapEventData => ""."FxBeatmapEventData"
);
#[cfg(feature = "FxBeatmapEventData")]
impl std::ops::Deref for FxBeatmapEventData {
    type Target = BeatmapEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FxBeatmapEventData")]
impl std::ops::DerefMut for FxBeatmapEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FxBeatmapEventData")]
impl FxBeatmapEventData {
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, groupId, elementId, usePreviousEventValue))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, groupId, elementId, usePreviousEventValue),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "FxBeatmapEventData")]
impl quest_hook::libil2cpp::ObjectType for FxBeatmapEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
