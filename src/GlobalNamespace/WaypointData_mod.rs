#[cfg(feature = "WaypointData")]
#[repr(C)]
#[derive(Debug)]
pub struct WaypointData {
    __cordl_parent: BeatmapObjectData,
    pub _offsetDirection_k__BackingField: OffsetDirection,
    pub _lineIndex_k__BackingField: i32,
    pub _lineLayer_k__BackingField: NoteLineLayer,
}
#[cfg(feature = "WaypointData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for WaypointData => ""."WaypointData"
);
#[cfg(feature = "WaypointData")]
impl std::ops::Deref for WaypointData {
    type Target = BeatmapObjectData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointData")]
impl std::ops::DerefMut for WaypointData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointData")]
impl WaypointData {
    pub fn Mirror(
        &mut self,
        lineCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mirror", (lineCount))?;
        Ok(__cordl_ret)
    }
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn MirrorTransformOffsetDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MirrorTransformOffsetDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: NoteLineLayer,
        offsetDirection: OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_time, lineIndex, lineLayer, offsetDirection))?;
        Ok(__cordl_ret)
    }
    pub fn get_offsetDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<OffsetDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: OffsetDirection = __cordl_object
            .invoke("get_offsetDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_offsetDirection(
        &mut self,
        value: OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_offsetDirection", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_lineIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_lineLayer(&mut self) -> quest_hook::libil2cpp::Result<NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteLineLayer = __cordl_object.invoke("get_lineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_lineLayer(
        &mut self,
        value: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineLayer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        _cordl_time: f32,
        lineIndex: i32,
        lineLayer: NoteLineLayer,
        offsetDirection: OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_time, lineIndex, lineLayer, offsetDirection))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "WaypointData")]
impl quest_hook::libil2cpp::ObjectType for WaypointData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
