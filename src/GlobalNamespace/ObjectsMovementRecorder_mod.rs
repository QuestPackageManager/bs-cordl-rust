#[cfg(feature = "ObjectsMovementRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectsMovementRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _poseObjects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::PoseObject>,
    >,
    pub _livPoseObjectId: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PoseObjectIdSO,
    >,
    pub _recorder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecorder>,
    pub _playback: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesPlayback>,
    pub _playbackScreenshotRecorder: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlaybackScreenshotRecorder,
    >,
    pub _playbackRender: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlaybackRenderer,
    >,
    pub _externalCameraPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _hmdCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ObjectsMovementRecorder_InitData,
    >,
    pub _mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    pub _recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    pub _externalCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _playbackScreenshots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
    >,
    pub _logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    pub _posesSerializer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPosesSerializer,
    >,
}
#[cfg(feature = "ObjectsMovementRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObjectsMovementRecorder => ""
    ."ObjectsMovementRecorder"
);
#[cfg(feature = "ObjectsMovementRecorder")]
impl std::ops::Deref for crate::GlobalNamespace::ObjectsMovementRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObjectsMovementRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder")]
impl crate::GlobalNamespace::ObjectsMovementRecorder {
    #[cfg(feature = "ObjectsMovementRecorder+CameraView")]
    pub type CameraView = crate::GlobalNamespace::ObjectsMovementRecorder_CameraView;
    #[cfg(feature = "ObjectsMovementRecorder+InitData")]
    pub type InitData = crate::GlobalNamespace::ObjectsMovementRecorder_InitData;
    #[cfg(feature = "ObjectsMovementRecorder+Mode")]
    pub type Mode = crate::GlobalNamespace::ObjectsMovementRecorder_Mode;
    pub fn HandleGameStateChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameStateChanged", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObjectsMovementRecorder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObjectsMovementRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ObjectsMovementRecorder+CameraView")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectsMovementRecorder_CameraView {
    FirstPerson = 0i32,
    ThirdPerson = 1i32,
}
#[cfg(feature = "ObjectsMovementRecorder+CameraView")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ObjectsMovementRecorder_CameraView => ""
    ."ObjectsMovementRecorder/CameraView"
);
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectsMovementRecorder_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    pub recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    pub addDateTimeSuffixToRecordingName: bool,
    pub screenshotRecording: bool,
    pub screenshotWidth: i32,
    pub screenshotHeight: i32,
    pub framerate: i32,
    pub playbackScreenshots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
    >,
    pub saveToOldFormat: bool,
    pub posesSerializer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IPosesSerializer,
    >,
    pub logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
}
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ObjectsMovementRecorder_InitData => ""
    ."ObjectsMovementRecorder/InitData"
);
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::ObjectsMovementRecorder_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObjectsMovementRecorder_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
impl crate::GlobalNamespace::ObjectsMovementRecorder_InitData {
    pub fn New(
        mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
        >,
        saveToOldFormat: bool,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
        posesSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPosesSerializer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    mode,
                    recordingPath,
                    cameraView,
                    addDateTimeSuffixToRecordingName,
                    screenshotRecording,
                    screenshotWidth,
                    screenshotHeight,
                    framerate,
                    playbackScreenshots,
                    saveToOldFormat,
                    logger,
                    posesSerializer,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
            >,
        >,
        saveToOldFormat: bool,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
        posesSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPosesSerializer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    mode,
                    recordingPath,
                    cameraView,
                    addDateTimeSuffixToRecordingName,
                    screenshotRecording,
                    screenshotWidth,
                    screenshotHeight,
                    framerate,
                    playbackScreenshots,
                    saveToOldFormat,
                    logger,
                    posesSerializer,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObjectsMovementRecorder_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ObjectsMovementRecorder+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectsMovementRecorder_Mode {
    Off = 2i32,
    Playback = 1i32,
    Record = 0i32,
}
#[cfg(feature = "ObjectsMovementRecorder+Mode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObjectsMovementRecorder_Mode =>
    ""."ObjectsMovementRecorder/Mode"
);
