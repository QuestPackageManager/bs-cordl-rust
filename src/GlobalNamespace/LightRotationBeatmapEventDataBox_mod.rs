#[cfg(feature = "LightRotationBeatmapEventDataBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationBeatmapEventDataBox {
    __cordl_parent: BeatmapEventDataBox,
    pub _lightRotationBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut LightRotationBaseData,
    >,
    pub _axis: LightAxis,
    pub _rotationDirection: f32,
    pub _rotationStep: f32,
    pub _beatStep: f32,
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightRotationBeatmapEventDataBox => ""
    ."LightRotationBeatmapEventDataBox"
);
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl std::ops::Deref for LightRotationBeatmapEventDataBox {
    type Target = BeatmapEventDataBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl std::ops::DerefMut for LightRotationBeatmapEventDataBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl LightRotationBeatmapEventDataBox {
    pub fn New(
        indexFilter: *mut IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        axis: LightAxis,
        flipRotation: bool,
        rotationDistributionParam: f32,
        rotationDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        rotationDistributionShouldAffectFirstBaseEvent: bool,
        rotationDistributionEaseType: EaseType,
        lightRotationBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut LightRotationBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    axis,
                    flipRotation,
                    rotationDistributionParam,
                    rotationDistributionParamType,
                    rotationDistributionShouldAffectFirstBaseEvent,
                    rotationDistributionEaseType,
                    lightRotationBaseDataList,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn Unpack(
        &mut self,
        groupBoxBeat: f32,
        groupId: i32,
        elementId: i32,
        durationOrderIndex: i32,
        distributionOrderIndex: i32,
        maxBeat: f32,
        beatToTimeConverter: *mut IBeatToTimeConverter,
        output: *mut crate::System::Collections::Generic::List_1<*mut BeatmapEventData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Unpack",
                (
                    groupBoxBeat,
                    groupId,
                    elementId,
                    durationOrderIndex,
                    distributionOrderIndex,
                    maxBeat,
                    beatToTimeConverter,
                    output,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        indexFilter: *mut IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        axis: LightAxis,
        flipRotation: bool,
        rotationDistributionParam: f32,
        rotationDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        rotationDistributionShouldAffectFirstBaseEvent: bool,
        rotationDistributionEaseType: EaseType,
        lightRotationBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut LightRotationBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    axis,
                    flipRotation,
                    rotationDistributionParam,
                    rotationDistributionParamType,
                    rotationDistributionShouldAffectFirstBaseEvent,
                    rotationDistributionEaseType,
                    lightRotationBaseDataList,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_beatStep(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatStep", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_subtypeIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeIdentifier", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl quest_hook::libil2cpp::ObjectType for LightRotationBeatmapEventDataBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}