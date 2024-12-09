#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CurvedCanvasSettingsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cachedCanvas: *mut crate::UnityEngine::Canvas,
    pub _cachedCanvasIsRootCanvas: bool,
    pub _curvedCanvasSettings: *mut crate::HMUI::CurvedCanvasSettings,
    pub _hasCachedData: bool,
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::CurvedCanvasSettingsHelper => "HMUI"
    ."CurvedCanvasSettingsHelper"
);
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl std::ops::Deref for crate::HMUI::CurvedCanvasSettingsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl std::ops::DerefMut for crate::HMUI::CurvedCanvasSettingsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl crate::HMUI::CurvedCanvasSettingsHelper {
    pub fn GetCurvedCanvasSettings(
        &mut self,
        canvas: *mut crate::UnityEngine::Canvas,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::CurvedCanvasSettings> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::CurvedCanvasSettings = __cordl_object
            .invoke("GetCurvedCanvasSettings", (canvas))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
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
#[cfg(feature = "HMUI+CurvedCanvasSettingsHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::CurvedCanvasSettingsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
