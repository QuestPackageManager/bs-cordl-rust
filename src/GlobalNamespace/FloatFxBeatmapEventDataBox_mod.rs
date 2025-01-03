#[cfg(feature = "FloatFxBeatmapEventDataBox")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxBeatmapEventDataBox {
    __cordl_parent: crate::GlobalNamespace::FxBeatmapEventDataBox_2<
        *mut crate::GlobalNamespace::FloatFxBaseData,
        *mut crate::GlobalNamespace::FloatFxBeatmapEventData,
    >,
    pub _vfxBaseDataList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::FloatFxBaseData,
        >,
    >,
    pub _beatStep: f32,
}
#[cfg(feature = "FloatFxBeatmapEventDataBox")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatFxBeatmapEventDataBox =>
    ""."FloatFxBeatmapEventDataBox"
);
#[cfg(feature = "FloatFxBeatmapEventDataBox")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxBeatmapEventDataBox {
    type Target = crate::GlobalNamespace::FxBeatmapEventDataBox_2<
        *mut crate::GlobalNamespace::FloatFxBaseData,
        *mut crate::GlobalNamespace::FloatFxBeatmapEventData,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBeatmapEventDataBox")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxBeatmapEventDataBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxBeatmapEventDataBox")]
impl crate::GlobalNamespace::FloatFxBeatmapEventDataBox {
    pub fn CreateVfxBeatmapEventData(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBaseData>,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        distributionOffset: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FloatFxBeatmapEventData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::FloatFxBeatmapEventData,
        > = __cordl_object
            .invoke(
                "CreateVfxBeatmapEventData",
                (data, _cordl_time, groupId, elementId, distributionOffset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        beatDistributionParam: f32,
        eventDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        eventDistributionParam: f32,
        eventDistributionShouldAffectFirstBaseEvent: bool,
        eventDistributionEaseType: crate::GlobalNamespace::EaseType,
        fxBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::FloatFxBaseData,
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
                    beatDistributionParamType,
                    beatDistributionParam,
                    eventDistributionParamType,
                    eventDistributionParam,
                    eventDistributionShouldAffectFirstBaseEvent,
                    eventDistributionEaseType,
                    fxBaseDataList,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        beatDistributionParam: f32,
        eventDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        eventDistributionParam: f32,
        eventDistributionShouldAffectFirstBaseEvent: bool,
        eventDistributionEaseType: crate::GlobalNamespace::EaseType,
        fxBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::FloatFxBaseData,
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
                    beatDistributionParamType,
                    beatDistributionParam,
                    eventDistributionParamType,
                    eventDistributionParam,
                    eventDistributionShouldAffectFirstBaseEvent,
                    eventDistributionEaseType,
                    fxBaseDataList,
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
#[cfg(feature = "FloatFxBeatmapEventDataBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FloatFxBeatmapEventDataBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
