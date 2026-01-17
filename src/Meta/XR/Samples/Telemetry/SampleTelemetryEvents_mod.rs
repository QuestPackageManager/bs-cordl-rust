#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SampleTelemetryEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.Samples.Telemetry";
    const CLASS_NAME: &'static str = "SampleTelemetryEvents";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl std::ops::Deref for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl std::ops::DerefMut for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents {
    #[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
    pub type AnnotationTypes =
        crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes;
    #[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
    pub type EventTypes = crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes;
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SampleTelemetryEvents_AnnotationTypes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.Samples.Telemetry";
    const CLASS_NAME: &'static str = "SampleTelemetryEvents/AnnotationTypes";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
impl std::ops::Deref
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
impl std::ops::DerefMut
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
impl crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes {
    pub const BuildTarget: &'static str = "BuildTarget";
    pub const InEditor: &'static str = "InEditor";
    pub const RuntimePlatform: &'static str = "RuntimePlatform";
    pub const Sample: &'static str = "Sample";
    pub const TimeSinceEditorStart: &'static str = "TimeSinceEditorStart";
    pub const TimeSpent: &'static str = "TimeSpent";
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SampleTelemetryEvents_EventTypes {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.Samples.Telemetry";
    const CLASS_NAME: &'static str = "SampleTelemetryEvents/EventTypes";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl std::ops::Deref for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl std::ops::DerefMut for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes {
    pub const Close: i32 = 163056880i32;
    pub const Open: i32 = 163055403i32;
    pub const Run: i32 = 163061602i32;
}
#[cfg(feature = "cordl_class_Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
