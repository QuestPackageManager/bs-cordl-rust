#[cfg(feature = "ScreenCaptureCache")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenCaptureCache {
    __cordl_parent: crate::System::Object,
    pub _cache: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::ScreenCaptureCache_ScreenshotType,
        *mut crate::UnityEngine::Texture2D,
    >,
}
#[cfg(feature = "ScreenCaptureCache")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ScreenCaptureCache => ""
    ."ScreenCaptureCache"
);
#[cfg(feature = "ScreenCaptureCache")]
impl std::ops::Deref for crate::GlobalNamespace::ScreenCaptureCache {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenCaptureCache")]
impl std::ops::DerefMut for crate::GlobalNamespace::ScreenCaptureCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ScreenCaptureCache")]
impl crate::GlobalNamespace::ScreenCaptureCache {
    #[cfg(feature = "ScreenCaptureCache+ScreenshotType")]
    pub type ScreenshotType = crate::GlobalNamespace::ScreenCaptureCache_ScreenshotType;
    pub fn GetLastScreenshot(
        &mut self,
        screenshotType: crate::GlobalNamespace::ScreenCaptureCache_ScreenshotType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("GetLastScreenshot", (screenshotType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn StoreScreenshot(
        &mut self,
        screenshotType: crate::GlobalNamespace::ScreenCaptureCache_ScreenshotType,
        texture: *mut crate::UnityEngine::Texture2D,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StoreScreenshot", (screenshotType, texture))?;
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
#[cfg(feature = "ScreenCaptureCache")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ScreenCaptureCache {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ScreenCaptureCache+ScreenshotType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenCaptureCache_ScreenshotType {
    Game = 0i32,
    Menu = 1i32,
    Other = 2i32,
}
#[cfg(feature = "ScreenCaptureCache+ScreenshotType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ScreenCaptureCache_ScreenshotType => ""
    ."ScreenCaptureCache/ScreenshotType"
);
