#[cfg(feature = "ScreenshotRecorder+RecordingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenshotRecorder_RecordingType {
    F10ForScreenshot = 3i32,
    Interval = 4i32,
    Mono360Sequence = 2i32,
    ScreenshotOnPause = 5i32,
    Sequence = 0i32,
    Stereo360Sequence = 1i32,
}
#[cfg(feature = "ScreenshotRecorder+RecordingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScreenshotRecorder_RecordingType => ""
    ."ScreenshotRecorder/RecordingType"
);
#[cfg(feature = "ScreenshotRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenshotRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _directory: *mut crate::System::String,
    pub _camera: *mut crate::UnityEngine::Camera,
    pub _frameRate: i32,
    pub _forceFixedFramerate: bool,
    pub _interval: i32,
    pub _recordingType: crate::GlobalNamespace::ScreenshotRecorder_RecordingType,
    pub _pauseWithPButton: bool,
    pub _antiAlias: i32,
    pub _screenshotWidth: i32,
    pub _screenshotHeight: i32,
    pub _counter: i32,
    pub _originalTimeScale: f32,
    pub _paused: bool,
    pub _frameNum: i32,
    pub _cubemapLeftEye: *mut crate::UnityEngine::RenderTexture,
    pub _cubemapRighEye: *mut crate::UnityEngine::RenderTexture,
    pub _equirectTexture: *mut crate::UnityEngine::RenderTexture,
    pub _cameraRenderTexture: *mut crate::UnityEngine::RenderTexture,
}
#[cfg(feature = "ScreenshotRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ScreenshotRecorder => ""."ScreenshotRecorder"
);
#[cfg(feature = "ScreenshotRecorder")]
impl std::ops::Deref for ScreenshotRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenshotRecorder")]
impl std::ops::DerefMut for ScreenshotRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenshotRecorder")]
impl ScreenshotRecorder {
    #[cfg(feature = "ScreenshotRecorder+RecordingType")]
    pub type RecordingType = crate::GlobalNamespace::ScreenshotRecorder_RecordingType;
    pub fn ConvertRenderTexture(
        &mut self,
        renderTexture: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("ConvertRenderTexture", (renderTexture))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnApplicationFocus(
        &mut self,
        hasFocus: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnApplicationFocus", (hasFocus))?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveCameraScreenshot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveCameraScreenshot", ())?;
        Ok(__cordl_ret)
    }
    pub fn SaveTextureScreenshot(
        &mut self,
        tex: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveTextureScreenshot", (tex))?;
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
    pub fn get_directory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_directory", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_directory(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_directory", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ScreenshotRecorder")]
impl quest_hook::libil2cpp::ObjectType for ScreenshotRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
