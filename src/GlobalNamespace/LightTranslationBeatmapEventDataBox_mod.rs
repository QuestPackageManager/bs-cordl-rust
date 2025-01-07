#[cfg(feature = "LightTranslationBeatmapEventDataBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationBeatmapEventDataBox {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventDataBox,
    pub _lightTranslationBaseDataList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightTranslationBaseData>,
        >,
    >,
    pub _axis: crate::GlobalNamespace::LightAxis,
    pub _translationDirection: f32,
    pub _beatStep: f32,
}
#[cfg(feature = "LightTranslationBeatmapEventDataBox")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightTranslationBeatmapEventDataBox {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightTranslationBeatmapEventDataBox";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "LightTranslationBeatmapEventDataBox")]
impl std::ops::Deref for crate::GlobalNamespace::LightTranslationBeatmapEventDataBox {
    type Target = crate::GlobalNamespace::BeatmapEventDataBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationBeatmapEventDataBox")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightTranslationBeatmapEventDataBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationBeatmapEventDataBox")]
impl crate::GlobalNamespace::LightTranslationBeatmapEventDataBox {
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        axis: crate::GlobalNamespace::LightAxis,
        flipTranslation: bool,
        gapDistributionParam: f32,
        gapDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        gapDistributionShouldAffectFirstBaseEvent: bool,
        gapDistributionEaseType: crate::GlobalNamespace::EaseType,
        lightTranslationBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightTranslationBaseData,
                >,
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
                    flipTranslation,
                    gapDistributionParam,
                    gapDistributionParamType,
                    gapDistributionShouldAffectFirstBaseEvent,
                    gapDistributionEaseType,
                    lightTranslationBaseDataList,
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
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
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
        flipTranslation: bool,
        gapDistributionParam: f32,
        gapDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        gapDistributionShouldAffectFirstBaseEvent: bool,
        gapDistributionEaseType: crate::GlobalNamespace::EaseType,
        lightTranslationBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::LightTranslationBaseData,
                >,
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
                    flipTranslation,
                    gapDistributionParam,
                    gapDistributionParamType,
                    gapDistributionShouldAffectFirstBaseEvent,
                    gapDistributionEaseType,
                    lightTranslationBaseDataList,
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
#[cfg(feature = "LightTranslationBeatmapEventDataBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightTranslationBeatmapEventDataBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
