#[cfg(feature = "HMUI+ScreenModeController")]
#[repr(C)]
#[derive(Debug)]
pub struct ScreenModeController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _curvedCanvases: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::HMUI::CurvedCanvasSettings>,
    >,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _defaultModeData: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenModeData>,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "HMUI+ScreenModeController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::ScreenModeController => "HMUI"
    ."ScreenModeController"
);
#[cfg(feature = "HMUI+ScreenModeController")]
impl std::ops::Deref for crate::HMUI::ScreenModeController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeController")]
impl std::ops::DerefMut for crate::HMUI::ScreenModeController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+ScreenModeController")]
impl crate::HMUI::ScreenModeController {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetDefaultMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaultMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMode(
        &mut self,
        screenModeData: quest_hook::libil2cpp::Gc<crate::HMUI::ScreenModeData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMode", (screenModeData))?;
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
#[cfg(feature = "HMUI+ScreenModeController")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::ScreenModeController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
