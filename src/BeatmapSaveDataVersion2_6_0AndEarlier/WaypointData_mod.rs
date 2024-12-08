#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
#[repr(C)]
#[derive(Debug)]
pub struct WaypointData {
    __cordl_parent: crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
    pub _time: f32,
    pub _lineIndex: i32,
    pub _lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."WaypointData"
);
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    type Target = crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, lineIndex, lineLayer, offsetDirection))?;
        Ok(__cordl_ret)
    }
    pub fn get_lineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = __cordl_object
            .invoke("get_lineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_offsetDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::OffsetDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::OffsetDirection = __cordl_object
            .invoke("get_offsetDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, lineIndex, lineLayer, offsetDirection))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+WaypointData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::WaypointData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
