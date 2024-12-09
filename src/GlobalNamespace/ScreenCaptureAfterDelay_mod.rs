#[cfg(feature = "ScreenCaptureAfterDelay")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenCaptureAfterDelay {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _mainEffectController: *mut crate::GlobalNamespace::MainEffectController,
    pub _screenCaptureCache: *mut crate::GlobalNamespace::ScreenCaptureCache,
    pub _initData: *mut crate::GlobalNamespace::ScreenCaptureAfterDelay_InitData,
    pub _captureTexture: *mut crate::UnityEngine::Texture2D,
    pub _captureRenderTexture: *mut crate::UnityEngine::RenderTexture,
}
#[cfg(feature = "ScreenCaptureAfterDelay")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ScreenCaptureAfterDelay => ""
    ."ScreenCaptureAfterDelay"
);
#[cfg(feature = "ScreenCaptureAfterDelay")]
impl std::ops::Deref for crate::GlobalNamespace::ScreenCaptureAfterDelay {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenCaptureAfterDelay")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScreenCaptureAfterDelay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenCaptureAfterDelay")]
impl crate::GlobalNamespace::ScreenCaptureAfterDelay {
    #[cfg(feature = "ScreenCaptureAfterDelay+InitData")]
    pub type InitData = crate::GlobalNamespace::ScreenCaptureAfterDelay_InitData;
    #[cfg(feature = "ScreenCaptureAfterDelay+_Start_d__6")]
    pub type _Start_d__6 = crate::GlobalNamespace::ScreenCaptureAfterDelay__Start_d__6;
    pub fn HandleMainEffectControllerAfterImageEffectEvent(
        &mut self,
        renderTexture: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleMainEffectControllerAfterImageEffectEvent", (renderTexture))?;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("Start", ())?;
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
#[cfg(feature = "ScreenCaptureAfterDelay")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScreenCaptureAfterDelay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScreenCaptureAfterDelay+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenCaptureAfterDelay_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub screenshotType: crate::GlobalNamespace::ScreenCaptureCache_ScreenshotType,
    pub screenCaptureTime: f32,
    pub pixelsWidth: i32,
    pub pixelsHeight: i32,
}
#[cfg(feature = "ScreenCaptureAfterDelay+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScreenCaptureAfterDelay_InitData => ""
    ."ScreenCaptureAfterDelay/InitData"
);
#[cfg(feature = "ScreenCaptureAfterDelay+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::ScreenCaptureAfterDelay_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenCaptureAfterDelay+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScreenCaptureAfterDelay_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenCaptureAfterDelay+InitData")]
impl crate::GlobalNamespace::ScreenCaptureAfterDelay_InitData {
    pub fn New(
        screenshotType: crate::GlobalNamespace::ScreenCaptureCache_ScreenshotType,
        screenCaptureTime: f32,
        pixelsWidth: i32,
        pixelsHeight: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (screenshotType, screenCaptureTime, pixelsWidth, pixelsHeight),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        screenshotType: crate::GlobalNamespace::ScreenCaptureCache_ScreenshotType,
        screenCaptureTime: f32,
        pixelsWidth: i32,
        pixelsHeight: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (screenshotType, screenCaptureTime, pixelsWidth, pixelsHeight),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ScreenCaptureAfterDelay+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ScreenCaptureAfterDelay_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
