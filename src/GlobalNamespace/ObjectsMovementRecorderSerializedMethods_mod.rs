#[cfg(feature = "ObjectsMovementRecorderSerializedMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectsMovementRecorderSerializedMethods {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn CameraViewFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraView: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CameraViewFromSerializedName", (name, cameraView))?;
        Ok(__cordl_ret.into())
    }
    pub fn ModeFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ModeFromSerializedName", (name, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlaybackScreenshotTypeFromSerializedName(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlaybackScreenshotTypeFromSerializedName", (name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName_ObjectsMovementRecorder_CameraView1(
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializedName", (cameraView))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName_ObjectsMovementRecorder_Mode0(
        mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializedName", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializedName_PlaybackScreenshot_PlaybackRenderer_Type2(
        _cordl_type: crate::GlobalNamespace::PlaybackScreenshot_PlaybackRenderer_Type,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SerializedName", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
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
