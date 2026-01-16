#[cfg(feature = "cordl_class_ObjectsMovementRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectsMovementRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _poseObjects: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PoseObject>,
        >,
    >,
    pub _livPoseObjectId: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PoseObjectIdSO>,
    pub _recorder: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesRecorder>,
    pub _playback: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PosesPlayback>,
    pub _playbackScreenshotRecorder:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlaybackScreenshotRecorder>,
    pub _playbackRender: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlaybackRenderer>,
    pub _externalCameraPrefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _audioTimeSyncController:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioTimeSyncController>,
    pub _hmdCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _initData:
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ObjectsMovementRecorder_InitData>,
    pub _mode: crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
    pub _recordingPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _cameraView: crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
    pub _externalCamera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _playbackScreenshots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot>,
        >,
    >,
    pub _logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
    pub _posesSerializer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPosesSerializer>,
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ObjectsMovementRecorder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObjectsMovementRecorder";
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
#[cfg(feature = "ObjectsMovementRecorder")]
impl std::ops::Deref for crate::GlobalNamespace::ObjectsMovementRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObjectsMovementRecorder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "HandleGameStateChanged",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "HandleGameStateChanged",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Init",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnDestroy",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ObjectsMovementRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+CameraView")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ObjectsMovementRecorder_CameraView {
    #[default]
    FirstPerson = 0i32,
    ThirdPerson = 1i32,
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+CameraView")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ObjectsMovementRecorder_CameraView
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObjectsMovementRecorder/CameraView";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+CameraView")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ObjectsMovementRecorder_CameraView
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+CameraView")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ObjectsMovementRecorder_CameraView
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+CameraView")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ObjectsMovementRecorder_CameraView
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+CameraView")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::ObjectsMovementRecorder_CameraView
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+InitData")]
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
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot>,
        >,
    >,
    pub saveToOldFormat: bool,
    pub posesSerializer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPosesSerializer>,
    pub logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+InitData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::ObjectsMovementRecorder_InitData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObjectsMovementRecorder/InitData";
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
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::ObjectsMovementRecorder_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObjectsMovementRecorder+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObjectsMovementRecorder_InitData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
                >,
            >,
        >,
        saveToOldFormat: bool,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
        posesSerializer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPosesSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
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
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
                >,
            >,
        >,
        saveToOldFormat: bool,
        logger: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
        posesSerializer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPosesSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::GlobalNamespace::ObjectsMovementRecorder_Mode,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        crate::GlobalNamespace::ObjectsMovementRecorder_CameraView,
                        bool,
                        bool,
                        i32,
                        i32,
                        i32,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::PlaybackRenderer_PlaybackScreenshot,
                                >,
                            >,
                        >,
                        bool,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberLogger>,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IPosesSerializer>,
                    ), quest_hook::libil2cpp::Void, 12usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
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
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+InitData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::ObjectsMovementRecorder_InitData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+Mode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ObjectsMovementRecorder_Mode {
    #[default]
    Off = 2i32,
    Playback = 1i32,
    Record = 0i32,
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+Mode")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::ObjectsMovementRecorder_Mode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ObjectsMovementRecorder/Mode";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+Mode")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::ObjectsMovementRecorder_Mode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+Mode")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::ObjectsMovementRecorder_Mode
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+Mode")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::ObjectsMovementRecorder_Mode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_ObjectsMovementRecorder+Mode")]
unsafe impl quest_hook::libil2cpp::Return for crate::GlobalNamespace::ObjectsMovementRecorder_Mode {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
