#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderData {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
    >,
    pub _colorType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
    pub _headTime: f32,
    pub _headLineIndex: i32,
    pub _headLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _headControlPointLengthMultiplier: f32,
    pub _headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    pub _tailTime: f32,
    pub _tailLineIndex: i32,
    pub _tailLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    pub _tailControlPointLengthMultiplier: f32,
    pub _tailCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    pub _sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData =>
    "BeatmapSaveDataVersion2_6_0AndEarlier"."SliderData"
);
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    type Target = quest_hook::libil2cpp::Gc<
        crate::BeatmapSaveDataVersion2_6_0AndEarlier::BeatmapSaveDataItem,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    pub fn New(
        colorType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
        headTime: f32,
        headLineIndex: i32,
        headLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailTime: f32,
        tailLineIndex: i32,
        tailLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    colorType,
                    headTime,
                    headLineIndex,
                    headLineLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    tailTime,
                    tailLineIndex,
                    tailLineLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    sliderMidAnchorMode,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        colorType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
        headTime: f32,
        headLineIndex: i32,
        headLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailTime: f32,
        tailLineIndex: i32,
        tailLineLayer: crate::BeatmapSaveDataCommon::NoteLineLayer,
        tailControlPointLengthMultiplier: f32,
        tailCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    colorType,
                    headTime,
                    headLineIndex,
                    headLineLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    tailTime,
                    tailLineIndex,
                    tailLineLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    sliderMidAnchorMode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType = __cordl_object
            .invoke("get_colorType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_headControlPointLengthMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = __cordl_object
            .invoke("get_headCutDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_headLineIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_headLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = __cordl_object
            .invoke("get_headLineLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sliderMidAnchorMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::SliderMidAnchorMode = __cordl_object
            .invoke("get_sliderMidAnchorMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tailControlPointLengthMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_tailControlPointLengthMultiplier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tailCutDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteCutDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteCutDirection = __cordl_object
            .invoke("get_tailCutDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tailLineIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_tailLineIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tailLineLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatmapSaveDataCommon::NoteLineLayer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatmapSaveDataCommon::NoteLineLayer = __cordl_object
            .invoke("get_tailLineLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tailTime(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_tailTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_time", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapSaveDataVersion2_6_0AndEarlier+SliderData")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapSaveDataVersion2_6_0AndEarlier::SliderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
