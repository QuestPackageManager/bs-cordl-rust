#[cfg(feature = "cordl_class_IBeatmapLightEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeatmapLightEventConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_IBeatmapLightEventConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IBeatmapLightEventConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IBeatmapLightEventConverter";
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
#[cfg(feature = "IBeatmapLightEventConverter")]
impl std::ops::Deref for crate::GlobalNamespace::IBeatmapLightEventConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLightEventConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBeatmapLightEventConverter {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLightEventConverter")]
impl crate::GlobalNamespace::IBeatmapLightEventConverter {
    pub fn ConvertBasicBeatmapEvent(
        &mut self,
        subtypeIdentifier: i32,
        _cordl_time: f32,
        basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        value: i32,
        floatValue: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            f32,
                            crate::GlobalNamespace::BasicBeatmapEventType,
                            i32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapEventData,
                                >,
                            >,
                        >,
                        5usize,
                    >("ConvertBasicBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertBasicBeatmapEvent", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        subtypeIdentifier,
                        _cordl_time,
                        basicBeatmapEventType,
                        value,
                        floatValue,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightColorBeatmapEvent(
        &mut self,
        subtypeIdentifier: i32,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousValue: bool,
        easeType: crate::GlobalNamespace::EaseType,
        colorType: crate::GlobalNamespace::EnvironmentColorType,
        brightness: f32,
        strobeBeatFrequency: i32,
        strobeBrightness: f32,
        strobeFade: bool,
        nextEventBrightness: crate::System::Nullable_1<f32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            f32,
                            i32,
                            i32,
                            bool,
                            crate::GlobalNamespace::EaseType,
                            crate::GlobalNamespace::EnvironmentColorType,
                            f32,
                            i32,
                            f32,
                            bool,
                            crate::System::Nullable_1<f32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapEventData,
                                >,
                            >,
                        >,
                        12usize,
                    >("ConvertLightColorBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertLightColorBeatmapEvent", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        subtypeIdentifier,
                        _cordl_time,
                        groupId,
                        elementId,
                        usePreviousValue,
                        easeType,
                        colorType,
                        brightness,
                        strobeBeatFrequency,
                        strobeBrightness,
                        strobeFade,
                        nextEventBrightness,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightRotationBeatmapEvent(
        &mut self,
        subtypeIdentifier: i32,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
        easeType: crate::GlobalNamespace::EaseType,
        axis: crate::GlobalNamespace::LightAxis,
        rotation: f32,
        loopCount: i32,
        rotationDirection: crate::GlobalNamespace::LightRotationDirection,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            f32,
                            i32,
                            i32,
                            bool,
                            crate::GlobalNamespace::EaseType,
                            crate::GlobalNamespace::LightAxis,
                            f32,
                            i32,
                            crate::GlobalNamespace::LightRotationDirection,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapEventData,
                                >,
                            >,
                        >,
                        10usize,
                    >("ConvertLightRotationBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertLightRotationBeatmapEvent", 10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        subtypeIdentifier,
                        _cordl_time,
                        groupId,
                        elementId,
                        usePreviousEventValue,
                        easeType,
                        axis,
                        rotation,
                        loopCount,
                        rotationDirection,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightTranslationBeatmapEvent(
        &mut self,
        subtypeIdentifier: i32,
        _cordl_time: f32,
        groupId: i32,
        elementId: i32,
        usePreviousEventValue: bool,
        easeType: crate::GlobalNamespace::EaseType,
        axis: crate::GlobalNamespace::LightAxis,
        translation: f32,
        distribution: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            i32,
                            f32,
                            i32,
                            i32,
                            bool,
                            crate::GlobalNamespace::EaseType,
                            crate::GlobalNamespace::LightAxis,
                            f32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapEventData,
                                >,
                            >,
                        >,
                        9usize,
                    >("ConvertLightTranslationBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConvertLightTranslationBeatmapEvent", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        subtypeIdentifier,
                        _cordl_time,
                        groupId,
                        elementId,
                        usePreviousEventValue,
                        easeType,
                        axis,
                        translation,
                        distribution,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertActivateOnStartEvents(
        &mut self,
        beatmapData: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InsertActivateOnStartEvents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InsertActivateOnStartEvents", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (beatmapData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_convertBoxGroups(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_convertBoxGroups")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_convertBoxGroups", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_convertBoxGroupsEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_convertBoxGroupsEvents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_convertBoxGroupsEvents", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasConversions(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_hasConversions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_hasConversions", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_useRotationDirection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_useRotationDirection")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_useRotationDirection", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_IBeatmapLightEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IBeatmapLightEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
