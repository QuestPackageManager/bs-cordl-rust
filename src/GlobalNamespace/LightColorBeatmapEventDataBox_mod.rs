#[cfg(feature = "LightColorBeatmapEventDataBox")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBeatmapEventDataBox {
    __cordl_parent: crate::GlobalNamespace::BeatmapEventDataBox,
    pub _lightColorBaseDataList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightColorBaseData>,
        >,
    >,
    pub _brightnessStep: f32,
    pub _beatStep: f32,
    pub _brightnessDistributionShouldAffectFirstBaseEvent: bool,
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightColorBeatmapEventDataBox {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightColorBeatmapEventDataBox";
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
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl std::ops::Deref for crate::GlobalNamespace::LightColorBeatmapEventDataBox {
    type Target = crate::GlobalNamespace::BeatmapEventDataBox;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightColorBeatmapEventDataBox {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl crate::GlobalNamespace::LightColorBeatmapEventDataBox {
    pub fn New(
        indexFilter: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IndexFilter>,
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionParam: f32,
        brightnessDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionShouldAffectFirstBaseEvent: bool,
        brightnessDistributionEaseType: crate::GlobalNamespace::EaseType,
        lightColorBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightColorBaseData>,
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
                    brightnessDistributionParam,
                    brightnessDistributionParamType,
                    brightnessDistributionShouldAffectFirstBaseEvent,
                    brightnessDistributionEaseType,
                    lightColorBaseDataList,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
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
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapEventData,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("Unpack")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Unpack", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
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
        beatDistributionParam: f32,
        beatDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionParam: f32,
        brightnessDistributionParamType: crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
        brightnessDistributionShouldAffectFirstBaseEvent: bool,
        brightnessDistributionEaseType: crate::GlobalNamespace::EaseType,
        lightColorBaseDataList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::LightColorBaseData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IndexFilter,
                            >,
                            f32,
                            crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                            f32,
                            crate::GlobalNamespace::BeatmapEventDataBox_DistributionParamType,
                            bool,
                            crate::GlobalNamespace::EaseType,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyList_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::LightColorBaseData,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        8usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_beatStep(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), f32, 0usize>("get_beatStep")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_beatStep", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_subtypeIdentifier(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), i32, 0usize>("get_subtypeIdentifier")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_subtypeIdentifier", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightColorBeatmapEventDataBox")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightColorBeatmapEventDataBox {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
