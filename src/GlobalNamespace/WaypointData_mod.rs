#[cfg(feature = "WaypointData")]
#[repr(C)]
#[derive(Debug)]
pub struct WaypointData {
    __cordl_parent: crate::GlobalNamespace::BeatmapObjectData,
    pub _offsetDirection_k__BackingField: crate::GlobalNamespace::OffsetDirection,
    pub _lineIndex_k__BackingField: i32,
    pub _lineLayer_k__BackingField: crate::GlobalNamespace::NoteLineLayer,
}
#[cfg(feature = "WaypointData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::WaypointData => ""
    ."WaypointData"
);
#[cfg(feature = "WaypointData")]
impl std::ops::Deref for crate::GlobalNamespace::WaypointData {
    type Target = crate::GlobalNamespace::BeatmapObjectData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointData")]
impl std::ops::DerefMut for crate::GlobalNamespace::WaypointData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "WaypointData")]
impl crate::GlobalNamespace::WaypointData {
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        > = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Mirror(
        &mut self,
        lineCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Mirror", (lineCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn MirrorTransformOffsetDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MirrorTransformOffsetDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        lineIndex: i32,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
        offsetDirection: crate::GlobalNamespace::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_time, beat, rotation, lineIndex, lineLayer, offsetDirection),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        beat: f32,
        rotation: i32,
        lineIndex: i32,
        lineLayer: crate::GlobalNamespace::NoteLineLayer,
        offsetDirection: crate::GlobalNamespace::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (_cordl_time, beat, rotation, lineIndex, lineLayer, offsetDirection),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_lineIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = __cordl_object
            .invoke("get_lineLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OffsetDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OffsetDirection = __cordl_object
            .invoke("get_offsetDirection", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_lineLayer(
        &mut self,
        value: crate::GlobalNamespace::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lineLayer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_offsetDirection(
        &mut self,
        value: crate::GlobalNamespace::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_offsetDirection", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "WaypointData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::WaypointData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
