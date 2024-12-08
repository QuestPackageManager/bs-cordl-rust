#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectsMovementRecorderSerializedMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ObjectsMovementRecorderSerializedMethods => ""
    ."ObjectsMovementRecorderSerializedMethods"
);
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl std::ops::Deref
for crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl std::ops::DerefMut
for crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    pub const kBackgroundPlaybackScreenshotType: &'static str = "Background";
    pub const kFirstPersonCameraView: &'static str = "FirstPerson";
    pub const kForegroundPlaybackScreenshotType: &'static str = "Foreground";
    pub const kOffRecordingMode: &'static str = "Off";
    pub const kPlaybackRecordingMode: &'static str = "Playback";
    pub const kRecordRecordingMode: &'static str = "Record";
    pub const kThirdPersonCameraView: &'static str = "ThirdPerson";
}
#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObjectsMovementRecorderSerializedMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
