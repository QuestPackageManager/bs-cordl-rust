#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
#[repr(C)]
#[derive(Debug)]
pub struct SampleTelemetryEvents_AnnotationTypes {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes =>
    "Meta.XR.Samples.Telemetry"."SampleTelemetryEvents/AnnotationTypes"
);
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
impl std::ops::Deref
for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
impl std::ops::DerefMut
for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
#[repr(C)]
#[derive(Debug)]
pub struct SampleTelemetryEvents_EventTypes {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes =>
    "Meta.XR.Samples.Telemetry"."SampleTelemetryEvents/EventTypes"
);
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl std::ops::Deref
for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl std::ops::DerefMut
for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes {
    pub const Close: i32 = 163056880i32;
    pub const Open: i32 = 163055403i32;
    pub const Run: i32 = 163061602i32;
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct SampleTelemetryEvents {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Meta::XR::Samples::Telemetry::SampleTelemetryEvents => "Meta.XR.Samples.Telemetry"
    ."SampleTelemetryEvents"
);
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl std::ops::Deref for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl std::ops::DerefMut for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents {
    #[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+EventTypes")]
    pub type EventTypes = crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_EventTypes;
    #[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents+AnnotationTypes")]
    pub type AnnotationTypes = crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents_AnnotationTypes;
}
#[cfg(feature = "Meta+XR+Samples+Telemetry+SampleTelemetryEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::Meta::XR::Samples::Telemetry::SampleTelemetryEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
