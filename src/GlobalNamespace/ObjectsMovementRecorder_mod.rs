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
    __cordl_parent: crate::System::Object,
    pub mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    pub recordingPath: *mut crate::System::String,
    pub cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    pub addDateTimeSuffixToRecordingName: bool,
    pub screenshotRecording: bool,
    pub screenshotWidth: i32,
    pub screenshotHeight: i32,
    pub framerate: i32,
    pub playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
    >,
    pub saveToOldFormat: bool,
    pub posesSerializer: *mut IPosesSerializer,
    pub logger: *mut IBeatSaberLogger,
}
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ObjectsMovementRecorder_InitData => ""
    ."ObjectsMovementRecorder/InitData"
);
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::ObjectsMovementRecorder_InitData {
    type Target = crate::System::Object;
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
        recordingPath: *mut crate::System::String,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
        saveToOldFormat: bool,
        logger: *mut IBeatSaberLogger,
        posesSerializer: *mut IPosesSerializer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
        recordingPath: *mut crate::System::String,
        cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
        addDateTimeSuffixToRecordingName: bool,
        screenshotRecording: bool,
        screenshotWidth: i32,
        screenshotHeight: i32,
        framerate: i32,
        playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
        >,
        saveToOldFormat: bool,
        logger: *mut IBeatSaberLogger,
        posesSerializer: *mut IPosesSerializer,
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
        Ok(__cordl_ret)
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
#[cfg(feature = "ObjectsMovementRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectsMovementRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _poseObjects: *mut quest_hook::libil2cpp::Il2CppArray<*mut PoseObject>,
    pub _livPoseObjectId: *mut PoseObjectIdSO,
    pub _recorder: *mut PosesRecorder,
    pub _playback: *mut PosesPlayback,
    pub _playbackScreenshotRecorder: *mut PlaybackScreenshotRecorder,
    pub _playbackRender: *mut PlaybackRenderer,
    pub _externalCameraPrefab: *mut crate::UnityEngine::Camera,
    pub _audioTimeSyncController: *mut AudioTimeSyncController,
    pub _hmdCamera: *mut crate::UnityEngine::Camera,
    pub _initData: *mut crate::GlobalNamespace::ObjectsMovementRecorder_InitData,
    pub _mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    pub _recordingPath: *mut crate::System::String,
    pub _cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    pub _externalCamera: *mut crate::UnityEngine::Camera,
    pub _playbackScreenshots: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
    >,
    pub _logger: *mut IBeatSaberLogger,
    pub _posesSerializer: *mut IPosesSerializer,
}
#[cfg(feature = "ObjectsMovementRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ObjectsMovementRecorder => ""."ObjectsMovementRecorder"
);
#[cfg(feature = "ObjectsMovementRecorder")]
impl std::ops::Deref for ObjectsMovementRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder")]
impl std::ops::DerefMut for ObjectsMovementRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder")]
impl ObjectsMovementRecorder {
    #[cfg(feature = "ObjectsMovementRecorder+InitData")]
    pub type InitData = crate::GlobalNamespace::ObjectsMovementRecorder_InitData;
    #[cfg(feature = "ObjectsMovementRecorder+Mode")]
    pub type Mode = crate::GlobalNamespace::ObjectsMovementRecorder_Mode;
    #[cfg(feature = "ObjectsMovementRecorder+CameraView")]
    pub type CameraView = crate::GlobalNamespace::ObjectsMovementRecorder_CameraView;
    pub fn HandleGameStateChanged(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleGameStateChanged", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ObjectsMovementRecorder")]
impl quest_hook::libil2cpp::ObjectType for ObjectsMovementRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
