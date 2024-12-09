#[cfg(feature = "FxBeatmapEventDataBox_2")]
#[repr(C)]
#[derive(Debug)]
pub struct FxBeatmapEventDataBox_2<
    TIn: quest_hook::libil2cpp::Type,
    TOut: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventDataBox,
    pub _beatStep: f32,
    pub _fxBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<TIn>,
    __cordl_phantom_TIn: std::marker::PhantomData<TIn>,
    __cordl_phantom_TOut: std::marker::PhantomData<TOut>,
}
#[cfg(feature = "FxBeatmapEventDataBox_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FxBeatmapEventDataBox_2 < TIn,
    TOut > => ""."FxBeatmapEventDataBox`2" < TIn, TOut >
);
#[cfg(feature = "FxBeatmapEventDataBox_2")]
impl<TIn: quest_hook::libil2cpp::Type, TOut: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::FxBeatmapEventDataBox_2<TIn, TOut> {
    type Target = crate::GlobalNamespace::BeatmapEventDataBox;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FxBeatmapEventDataBox_2")]
impl<
    TIn: quest_hook::libil2cpp::Type,
    TOut: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::GlobalNamespace::FxBeatmapEventDataBox_2<TIn, TOut> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FxBeatmapEventDataBox_2")]
impl<
    TIn: quest_hook::libil2cpp::Type,
    TOut: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::FxBeatmapEventDataBox_2<TIn, TOut> {
    pub fn CreateVfxBeatmapEventData(
        &mut self,
        data: TIn,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        distributionOffset: f32,
    ) -> quest_hook::libil2cpp::Result<TOut>
    where
        TIn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOut: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TOut = __cordl_object
            .invoke(
                "CreateVfxBeatmapEventData",
                (data, _cordl_time, groupId, elementId, distributionOffset),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        indexFilter: *mut crate::GlobalNamespace::IndexFilter,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        beatDistributionParam: f32,
        eventDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        eventDistributionParam: f32,
        eventDistributionShouldAffectFirstBaseEvent: bool,
        eventDistributionEaseType: crate::GlobalNamespace::EaseType,
        fxBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<TIn>,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TIn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOut: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
        beatToTimeConverter: *mut crate::GlobalNamespace::IBeatToTimeConverter,
        output: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TIn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOut: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
        indexFilter: *mut crate::GlobalNamespace::IndexFilter,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        beatDistributionParam: f32,
        eventDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        eventDistributionParam: f32,
        eventDistributionShouldAffectFirstBaseEvent: bool,
        eventDistributionEaseType: crate::GlobalNamespace::EaseType,
        fxBaseDataList: *mut crate::System::Collections::Generic::IReadOnlyList_1<TIn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TIn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOut: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "FxBeatmapEventDataBox_2")]
impl<
    TIn: quest_hook::libil2cpp::Type,
    TOut: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FxBeatmapEventDataBox_2<TIn, TOut> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
