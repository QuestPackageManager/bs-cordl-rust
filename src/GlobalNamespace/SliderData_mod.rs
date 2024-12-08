#[cfg(feature = "SliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderData {
    __cordl_parent: BeatmapObjectData,
    pub _colorType_k__BackingField: ColorType,
    pub _sliderType_k__BackingField: crate::GlobalNamespace::SliderData_Type,
    pub _hasHeadNote_k__BackingField: bool,
    pub _headControlPointLengthMultiplier_k__BackingField: f32,
    pub _headLineIndex_k__BackingField: i32,
    pub _headLineLayer_k__BackingField: NoteLineLayer,
    pub _headBeforeJumpLineLayer_k__BackingField: NoteLineLayer,
    pub _headCutDirection_k__BackingField: NoteCutDirection,
    pub _headCutDirectionAngleOffset_k__BackingField: f32,
    pub _hasTailNote_k__BackingField: bool,
    pub _tailTime_k__BackingField: f32,
    pub _tailLineIndex_k__BackingField: i32,
    pub _tailControlPointLengthMultiplier_k__BackingField: f32,
    pub _tailLineLayer_k__BackingField: NoteLineLayer,
    pub _tailBeforeJumpLineLayer_k__BackingField: NoteLineLayer,
    pub _tailCutDirection_k__BackingField: NoteCutDirection,
    pub _tailCutDirectionAngleOffset_k__BackingField: f32,
    pub _midAnchorMode_k__BackingField: SliderMidAnchorMode,
    pub _sliceCount_k__BackingField: i32,
    pub _squishAmount_k__BackingField: f32,
}
#[cfg(feature = "SliderData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SliderData => ""."SliderData"
);
#[cfg(feature = "SliderData")]
impl std::ops::Deref for SliderData {
    type Target = BeatmapObjectData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderData")]
impl std::ops::DerefMut for SliderData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderData")]
impl SliderData {
    #[cfg(feature = "SliderData+Type")]
    pub type Type = crate::GlobalNamespace::SliderData_Type;
    pub fn set_headLineLayer(
        &mut self,
        value: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headLineLayer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hasTailNote(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasTailNote", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_tailLineLayer(&mut self) -> quest_hook::libil2cpp::Result<NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteLineLayer = __cordl_object.invoke("get_tailLineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_tailCutDirectionAngleOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tailCutDirectionAngleOffset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sliceCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sliceCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_headCutDirectionAngleOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_headCutDirectionAngleOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_tailLineIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tailLineIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_tailTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_tailTime", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_hasHeadNote(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasHeadNote", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorType(
        &mut self,
        value: ColorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_tailCutDirection(
        &mut self,
        value: NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tailCutDirection", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn SetHeadBeforeJumpLineLayer(
        &mut self,
        lineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHeadBeforeJumpLineLayer", (lineLayer))?;
        Ok(__cordl_ret)
    }
    pub fn set_squishAmount(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_squishAmount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_headLineIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headLineIndex", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_subtypeGroupIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeGroupIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasHeadNote(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasHeadNote", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliderType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderData_Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::SliderData_Type = __cordl_object
            .invoke("get_sliderType", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_headCutDirectionAngleOffset(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headCutDirectionAngleOffset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_tailLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_tailLineIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_tailBeforeJumpLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteLineLayer = __cordl_object
            .invoke("get_tailBeforeJumpLineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_squishAmount(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_squishAmount", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetCutDirectionAngleOffset(
        &mut self,
        headCutDirectionAngleOffset: f32,
        tailCutDirectionAngleOffset: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetCutDirectionAngleOffset",
                (headCutDirectionAngleOffset, tailCutDirectionAngleOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_headLineLayer(&mut self) -> quest_hook::libil2cpp::Result<NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteLineLayer = __cordl_object.invoke("get_headLineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_headCutDirection(
        &mut self,
        value: NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headCutDirection", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_hasTailNote(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasTailNote", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_sliderType(
        &mut self,
        value: crate::GlobalNamespace::SliderData_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sliderType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_headLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_headLineIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_midAnchorMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<SliderMidAnchorMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: SliderMidAnchorMode = __cordl_object
            .invoke("get_midAnchorMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetHasTailNote(
        &mut self,
        hasTailNote: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHasTailNote", (hasTailNote))?;
        Ok(__cordl_ret)
    }
    pub fn GetCopy(&mut self) -> quest_hook::libil2cpp::Result<*mut BeatmapDataItem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapDataItem = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTailBeforeJumpLineLayer(
        &mut self,
        lineLayer: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTailBeforeJumpLineLayer", (lineLayer))?;
        Ok(__cordl_ret)
    }
    pub fn set_headBeforeJumpLineLayer(
        &mut self,
        value: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headBeforeJumpLineLayer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_tailTime(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tailTime", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_tailControlPointLengthMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tailControlPointLengthMultiplier", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_headBeforeJumpLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteLineLayer = __cordl_object
            .invoke("get_headBeforeJumpLineLayer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_headControlPointLengthMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_tailLineLayer(
        &mut self,
        value: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tailLineLayer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_headControlPointLengthMultiplier(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_headControlPointLengthMultiplier", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_tailCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteCutDirection = __cordl_object
            .invoke("get_tailCutDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorType(&mut self) -> quest_hook::libil2cpp::Result<ColorType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: ColorType = __cordl_object.invoke("get_colorType", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetHasHeadNote(
        &mut self,
        hasHeadNote: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHasHeadNote", (hasHeadNote))?;
        Ok(__cordl_ret)
    }
    pub fn get_tailCutDirectionAngleOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_tailCutDirectionAngleOffset", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_headCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: NoteCutDirection = __cordl_object
            .invoke("get_headCutDirection", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_midAnchorMode(
        &mut self,
        value: SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_midAnchorMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_tailBeforeJumpLineLayer(
        &mut self,
        value: NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tailBeforeJumpLineLayer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        sliderType: crate::GlobalNamespace::SliderData_Type,
        colorType: ColorType,
        hasHeadNote: bool,
        headTime: f32,
        headLineIndex: i32,
        headLineLayer: NoteLineLayer,
        headBeforeJumpLineLayer: NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: NoteCutDirection,
        headCutDirectionAngleOffset: f32,
        hasTailNote: bool,
        tailTime: f32,
        tailLineIndex: i32,
        tailLineLayer: NoteLineLayer,
        tailBeforeJumpLineLayer: NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: NoteCutDirection,
        tailCutDirectionAngleOffset: f32,
        midAnchorMode: SliderMidAnchorMode,
        sliceCount: i32,
        squishAmount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    sliderType,
                    colorType,
                    hasHeadNote,
                    headTime,
                    headLineIndex,
                    headLineLayer,
                    headBeforeJumpLineLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    headCutDirectionAngleOffset,
                    hasTailNote,
                    tailTime,
                    tailLineIndex,
                    tailLineLayer,
                    tailBeforeJumpLineLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    tailCutDirectionAngleOffset,
                    midAnchorMode,
                    sliceCount,
                    squishAmount,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_tailControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_tailControlPointLengthMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sliceCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_sliceCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        sliderType: crate::GlobalNamespace::SliderData_Type,
        colorType: ColorType,
        hasHeadNote: bool,
        headTime: f32,
        headLineIndex: i32,
        headLineLayer: NoteLineLayer,
        headBeforeJumpLineLayer: NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: NoteCutDirection,
        headCutDirectionAngleOffset: f32,
        hasTailNote: bool,
        tailTime: f32,
        tailLineIndex: i32,
        tailLineLayer: NoteLineLayer,
        tailBeforeJumpLineLayer: NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: NoteCutDirection,
        tailCutDirectionAngleOffset: f32,
        midAnchorMode: SliderMidAnchorMode,
        sliceCount: i32,
        squishAmount: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    sliderType,
                    colorType,
                    hasHeadNote,
                    headTime,
                    headLineIndex,
                    headLineLayer,
                    headBeforeJumpLineLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    headCutDirectionAngleOffset,
                    hasTailNote,
                    tailTime,
                    tailLineIndex,
                    tailLineLayer,
                    tailBeforeJumpLineLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    tailCutDirectionAngleOffset,
                    midAnchorMode,
                    sliceCount,
                    squishAmount,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SliderData")]
impl quest_hook::libil2cpp::ObjectType for SliderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SliderData+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderData_Type {
    Burst = 1i32,
    Normal = 0i32,
}
#[cfg(feature = "SliderData+Type")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SliderData_Type => ""
    ."SliderData/Type"
);
