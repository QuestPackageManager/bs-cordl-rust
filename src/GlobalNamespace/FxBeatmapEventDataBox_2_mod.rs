#[cfg(feature = "cordl_class_FxBeatmapEventDataBox_2")]
#[repr(C)]
#[derive(Debug)]
pub struct FxBeatmapEventDataBox_2<
    TIn: quest_hook::libil2cpp::Type,
    TOut: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventDataBox,
    pub _beatStep: f32,
    pub _fxBaseDataList: quest_hook::libil2cpp::Gc<TIn>,
    __cordl_phantom_TIn: std::marker::PhantomData<TIn>,
    __cordl_phantom_TOut: std::marker::PhantomData<TOut>,
}
#[cfg(feature = "cordl_class_FxBeatmapEventDataBox_2")]
unsafe impl<
    TIn: quest_hook::libil2cpp::Type,
    TOut: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FxBeatmapEventDataBox_2<TIn, TOut> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FxBeatmapEventDataBox`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("", "FxBeatmapEventDataBox`2")
                    .unwrap()
                    .make_generic::<(TIn, TOut)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "FxBeatmapEventDataBox_2")]
impl<TIn: quest_hook::libil2cpp::Type, TOut: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::FxBeatmapEventDataBox_2<TIn, TOut> {
    type Target = crate::GlobalNamespace::BeatmapEventDataBox;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FxBeatmapEventDataBox_2")]
impl<
    TIn: quest_hook::libil2cpp::Type,
    TOut: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::GlobalNamespace::FxBeatmapEventDataBox_2<TIn, TOut> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TIn, f32, i32, i32, f32),
                        TOut,
                        5usize,
                    >("CreateVfxBeatmapEventData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateVfxBeatmapEventData", 5usize
                        )
                    })
            });
        let __cordl_ret: TOut = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (data, _cordl_time, groupId, elementId, distributionOffset),
                )?
        };
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
        fxBaseDataList: quest_hook::libil2cpp::Gc<TIn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
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
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TIn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOut: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            f32,
                            i32,
                            i32,
                            i32,
                            i32,
                            f32,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatToTimeConverter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatmapLightEventConverter,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapEventData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("Unpack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Unpack",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
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
        fxBaseDataList: quest_hook::libil2cpp::Gc<TIn>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TIn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TOut: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IndexFilter,
                            >,
                            crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                            f32,
                            crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                            f32,
                            bool,
                            crate::GlobalNamespace::EaseType,
                            quest_hook::libil2cpp::Gc<TIn>,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_FxBeatmapEventDataBox_2")]
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
