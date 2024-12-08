#[cfg(feature = "BeatmapDataItemExecutionOrderConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataItemExecutionOrderConstants {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataItemExecutionOrderConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapDataItemExecutionOrderConstants => ""
    ."BeatmapDataItemExecutionOrderConstants"
);
#[cfg(feature = "BeatmapDataItemExecutionOrderConstants")]
impl std::ops::Deref for BeatmapDataItemExecutionOrderConstants {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataItemExecutionOrderConstants")]
impl std::ops::DerefMut for BeatmapDataItemExecutionOrderConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataItemExecutionOrderConstants")]
impl BeatmapDataItemExecutionOrderConstants {
    pub const kBPMChangeBeatmapEventExecutionOrder: i32 = -1001i32;
    pub const kCommonBeatmapEventExecutionOrder: i32 = -100i32;
    pub const kCommonBeatmapObjectDataExecutionOrder: i32 = 100i32;
    pub const kEarlySpawnRotationBeatmapEventExecutionOrder: i32 = -1000i32;
    pub const kLateSpawnRotationBeatmapEventExecutionOrder: i32 = 1000i32;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapDataItemExecutionOrderConstants")]
impl quest_hook::libil2cpp::ObjectType for BeatmapDataItemExecutionOrderConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
