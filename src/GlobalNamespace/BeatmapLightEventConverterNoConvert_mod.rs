#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLightEventConverterNoConvert {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapLightEventConverterNoConvert {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapLightEventConverterNoConvert";
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
#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLightEventConverterNoConvert {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLightEventConverterNoConvert {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
impl crate::GlobalNamespace::BeatmapLightEventConverterNoConvert {
    pub fn ConvertBasicBeatmapEvent(
        &mut self,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
        subtypeIdentifier: i32,
        _cordl_time: f32,
        basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        value: i32,
        floatValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapEventData,
                                    >,
                                >,
                            >,
                            i32,
                            f32,
                            crate::GlobalNamespace::BasicBeatmapEventType,
                            i32,
                            f32,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("ConvertBasicBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertBasicBeatmapEvent", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        output,
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
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapEventData,
                                    >,
                                >,
                            >,
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
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >("ConvertLightColorBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertLightColorBeatmapEvent", 12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        output,
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
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightRotationBeatmapEvent(
        &mut self,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatmapEventData,
                                    >,
                                >,
                            >,
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
                        quest_hook::libil2cpp::Void,
                        11usize,
                    >("ConvertLightRotationBeatmapEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertLightRotationBeatmapEvent", 11usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        output,
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLightEventConverterNoConvert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
impl AsRef<crate::GlobalNamespace::IBeatmapLightEventConverter>
for crate::GlobalNamespace::BeatmapLightEventConverterNoConvert {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatmapLightEventConverter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapLightEventConverterNoConvert")]
impl AsMut<crate::GlobalNamespace::IBeatmapLightEventConverter>
for crate::GlobalNamespace::BeatmapLightEventConverterNoConvert {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatmapLightEventConverter {
        unsafe { std::mem::transmute(self) }
    }
}
