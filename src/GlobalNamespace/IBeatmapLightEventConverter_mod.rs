#[cfg(feature = "IBeatmapLightEventConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeatmapLightEventConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IBeatmapLightEventConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IBeatmapLightEventConverter =>
    ""."IBeatmapLightEventConverter"
);
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
                *mut crate::GlobalNamespace::BeatmapEventData,
            >,
        >,
        subtypeIdentifier: i32,
        _cordl_time: f32,
        basicBeatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
        value: i32,
        floatValue: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ConvertBasicBeatmapEvent",
                (
                    output,
                    subtypeIdentifier,
                    _cordl_time,
                    basicBeatmapEventType,
                    value,
                    floatValue,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightColorBeatmapEvent(
        &mut self,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapEventData,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ConvertLightColorBeatmapEvent",
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
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertLightRotationBeatmapEvent(
        &mut self,
        output: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::BeatmapEventData,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ConvertLightRotationBeatmapEvent",
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
            )?;
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