#[cfg(feature = "LightRotationBeatmapEventDataBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightRotationBeatmapEventDataBox {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventDataBox,
    pub _lightRotationBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::GlobalNamespace::LightRotationBaseData,
    >,
    pub _axis: crate::GlobalNamespace::LightAxis,
    pub _rotationDirection: f32,
    pub _rotationStep: f32,
    pub _beatStep: f32,
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LightRotationBeatmapEventDataBox => ""
    ."LightRotationBeatmapEventDataBox"
);
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl std::ops::Deref for crate::GlobalNamespace::LightRotationBeatmapEventDataBox {
    type Target = crate::GlobalNamespace::BeatmapEventDataBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightRotationBeatmapEventDataBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl crate::GlobalNamespace::LightRotationBeatmapEventDataBox {
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        axis: crate::GlobalNamespace::LightAxis,
        flipRotation: bool,
        rotationDistributionParam: f32,
        rotationDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        rotationDistributionShouldAffectFirstBaseEvent: bool,
        rotationDistributionEaseType: crate::GlobalNamespace::EaseType,
        lightRotationBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::LightRotationBaseData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn Unpack(
        &mut self,
        groupBoxBeat: f32,
        groupId: i32,
        elementId: i32,
        durationOrderIndex: i32,
        distributionOrderIndex: i32,
        maxBeat: f32,
        beatToTimeConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatToTimeConverter,
        >,
        lightEventConverter: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatmapLightEventConverter,
        >,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapEventData,
            >,
        >,
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
                    lightEventConverter,
                    output,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        axis: crate::GlobalNamespace::LightAxis,
        flipRotation: bool,
        rotationDistributionParam: f32,
        rotationDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        rotationDistributionShouldAffectFirstBaseEvent: bool,
        rotationDistributionEaseType: crate::GlobalNamespace::EaseType,
        lightRotationBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::LightRotationBaseData,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_beatStep(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatStep", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeIdentifier", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightRotationBeatmapEventDataBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightRotationBeatmapEventDataBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
