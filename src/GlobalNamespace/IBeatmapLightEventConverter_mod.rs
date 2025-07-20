#[cfg(feature = "IBeatmapLightEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeatmapLightEventConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBeatmapLightEventConverter")]
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
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLightEventConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBeatmapLightEventConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatmapLightEventConverter")]
impl crate::GlobalNamespace::IBeatmapLightEventConverter {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBeatmapLightEventConverter as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBeatmapLightEventConverter as
                    quest_hook::libil2cpp::Type > ::class(), "ConvertBasicBeatmapEvent",
                    6usize
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBeatmapLightEventConverter as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBeatmapLightEventConverter as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ConvertLightColorBeatmapEvent", 12usize
                )
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IBeatmapLightEventConverter as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IBeatmapLightEventConverter as
                    quest_hook::libil2cpp::Type > ::class(),
                    "ConvertLightRotationBeatmapEvent", 11usize
                )
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IBeatmapLightEventConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IBeatmapLightEventConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
