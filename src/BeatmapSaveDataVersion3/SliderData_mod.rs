#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderData {
    __cordl_parent: crate::BeatmapSaveDataVersion3::BaseSliderData,
    pub mu: f32,
    pub tmu: f32,
    pub tc: crate::BeatmapSaveDataCommon::NoteCutDirection,
    pub m: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapSaveDataVersion3::SliderData =>
    "BeatmapSaveDataVersion3"."SliderData"
);
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl std::ops::Deref for crate::BeatmapSaveDataVersion3::SliderData {
    type Target = crate::BeatmapSaveDataVersion3::BaseSliderData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl std::ops::DerefMut for crate::BeatmapSaveDataVersion3::SliderData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl crate::BeatmapSaveDataVersion3::SliderData {
    pub fn New(
        colorType: crate::BeatmapSaveDataCommon::NoteColorType,
        headBeat: f32,
        headLine: i32,
        headLayer: i32,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailBeat: f32,
        tailLine: i32,
        tailLayer: i32,
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
                    headBeat,
                    headLine,
                    headLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    tailBeat,
                    tailLine,
                    tailLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    sliderMidAnchorMode,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        colorType: crate::BeatmapSaveDataCommon::NoteColorType,
        headBeat: f32,
        headLine: i32,
        headLayer: i32,
        headControlPointLengthMultiplier: f32,
        headCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
        tailBeat: f32,
        tailLine: i32,
        tailLayer: i32,
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
                    headBeat,
                    headLine,
                    headLayer,
                    headControlPointLengthMultiplier,
                    headCutDirection,
                    tailBeat,
                    tailLine,
                    tailLayer,
                    tailControlPointLengthMultiplier,
                    tailCutDirection,
                    sliderMidAnchorMode,
                ),
            )?;
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
}
#[cfg(feature = "BeatmapSaveDataVersion3+SliderData")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatmapSaveDataVersion3::SliderData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
