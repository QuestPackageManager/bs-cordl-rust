#[cfg(feature = "BeatmapTypeConverters")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapTypeConverters {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "BeatmapTypeConverters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapTypeConverters => ""
    ."BeatmapTypeConverters"
);
#[cfg(feature = "BeatmapTypeConverters")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapTypeConverters {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapTypeConverters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl crate::GlobalNamespace::BeatmapTypeConverters {
    pub fn ConvertBasicBeatmapEventType(
        beatmapEventType: crate::BeatmapSaveDataCommon::BeatmapEventType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::BasicBeatmapEventType> {
        let __cordl_ret: crate::GlobalNamespace::BasicBeatmapEventType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertBasicBeatmapEventType", (beatmapEventType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertDistributionParamType(
        distributionParamType: crate::BeatmapSaveDataCommon::DistributionParamType,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
    > {
        let __cordl_ret: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertDistributionParamType", (distributionParamType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEaseType(
        easeType: crate::BeatmapSaveDataCommon::EaseType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EaseType> {
        let __cordl_ret: crate::GlobalNamespace::EaseType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertEaseType", (easeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertEnvironmentColorType(
        environmentColorType: crate::BeatmapSaveDataCommon::EnvironmentColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EnvironmentColorType> {
        let __cordl_ret: crate::GlobalNamespace::EnvironmentColorType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertEnvironmentColorType", (environmentColorType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightAxis(
        axis: crate::BeatmapSaveDataCommon::Axis,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::LightAxis> {
        let __cordl_ret: crate::GlobalNamespace::LightAxis = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertLightAxis", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightRotationDirection(
        rotationDirection: crate::BeatmapSaveDataCommon::RotationDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::LightRotationDirection> {
        let __cordl_ret: crate::GlobalNamespace::LightRotationDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertLightRotationDirection", (rotationDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteColorType_ColorType0(
        noteType: crate::BeatmapSaveDataVersion2_6_0AndEarlier::ColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        let __cordl_ret: crate::GlobalNamespace::ColorType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertNoteColorType", (noteType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteColorType_NoteColorType1(
        noteType: crate::BeatmapSaveDataCommon::NoteColorType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ColorType> {
        let __cordl_ret: crate::GlobalNamespace::ColorType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertNoteColorType", (noteType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteCutDirection(
        noteCutDirection: crate::BeatmapSaveDataCommon::NoteCutDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteCutDirection> {
        let __cordl_ret: crate::GlobalNamespace::NoteCutDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertNoteCutDirection", (noteCutDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteLineLayer_NoteLineLayer1(
        layer: crate::BeatmapSaveDataCommon::NoteLineLayer,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertNoteLineLayer", (layer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertNoteLineLayer_i32_0(
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteLineLayer> {
        let __cordl_ret: crate::GlobalNamespace::NoteLineLayer = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertNoteLineLayer", (layer))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertOffsetDirection(
        offsetDirection: crate::BeatmapSaveDataCommon::OffsetDirection,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OffsetDirection> {
        let __cordl_ret: crate::GlobalNamespace::OffsetDirection = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertOffsetDirection", (offsetDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertSliderDataType(
        sliderType: crate::BeatmapSaveDataVersion3::SliderType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderData_Type> {
        let __cordl_ret: crate::GlobalNamespace::SliderData_Type = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertSliderDataType", (sliderType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertSliderMidAnchorMode(
        sliderMidAnchorMode: crate::BeatmapSaveDataCommon::SliderMidAnchorMode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SliderMidAnchorMode> {
        let __cordl_ret: crate::GlobalNamespace::SliderMidAnchorMode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertSliderMidAnchorMode", (sliderMidAnchorMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTransitionTypeToEaseType(
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::EaseType> {
        let __cordl_ret: crate::GlobalNamespace::EaseType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertTransitionTypeToEaseType", (transitionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertTransitionTypeToExtension(
        transitionType: crate::BeatmapSaveDataVersion3::TransitionType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertTransitionTypeToExtension", (transitionType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapTypeConverters")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapTypeConverters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
