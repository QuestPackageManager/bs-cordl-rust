#[cfg(feature = "LightColorBeatmapEventDataBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBeatmapEventDataBox {
    __cordl_parent: BeatmapEventDataBox,
    pub _lightColorBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut LightColorBaseData,
    >,
    pub _brightnessStep: f32,
    pub _beatStep: f32,
    pub _brightnessDistributionShouldAffectFirstBaseEvent: bool,
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightColorBeatmapEventDataBox => ""
    ."LightColorBeatmapEventDataBox"
);
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl std::ops::Deref for LightColorBeatmapEventDataBox {
    type Target = BeatmapEventDataBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl std::ops::DerefMut for LightColorBeatmapEventDataBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl LightColorBeatmapEventDataBox {
    pub fn _ctor(
        &mut self,
        indexFilter: *mut IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionParam: f32,
        brightnessDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionShouldAffectFirstBaseEvent: bool,
        brightnessDistributionEaseType: EaseType,
        lightColorBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut LightColorBaseData,
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
                    brightnessDistributionParam,
                    brightnessDistributionParamType,
                    brightnessDistributionShouldAffectFirstBaseEvent,
                    brightnessDistributionEaseType,
                    lightColorBaseDataList,
                ),
            )?;
        Ok(__cordl_ret)
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
    pub fn get_subtypeIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subtypeIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_beatStep(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_beatStep", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        indexFilter: *mut IndexFilter,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionParam: f32,
        brightnessDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionShouldAffectFirstBaseEvent: bool,
        brightnessDistributionEaseType: EaseType,
        lightColorBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut LightColorBaseData,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    indexFilter,
                    beatDistributionParam,
                    beatDistributionParamType,
                    brightnessDistributionParam,
                    brightnessDistributionParamType,
                    brightnessDistributionShouldAffectFirstBaseEvent,
                    brightnessDistributionEaseType,
                    lightColorBaseDataList,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl quest_hook::libil2cpp::ObjectType for LightColorBeatmapEventDataBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
