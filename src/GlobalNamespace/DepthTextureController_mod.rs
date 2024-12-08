#[cfg(feature = "DepthTextureController")]
#[repr(C)]
#[derive(Debug)]
pub struct DepthTextureController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _handler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _sceneSetupData: *mut crate::GlobalNamespace::GameplayCoreSceneSetupData,
    pub _camera: *mut crate::UnityEngine::Camera,
    pub _cachedPreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
}
#[cfg(feature = "DepthTextureController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DepthTextureController => ""
    ."DepthTextureController"
);
#[cfg(feature = "DepthTextureController")]
impl std::ops::Deref for crate::GlobalNamespace::DepthTextureController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DepthTextureController")]
impl std::ops::DerefMut for crate::GlobalNamespace::DepthTextureController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DepthTextureController")]
impl crate::GlobalNamespace::DepthTextureController {
    pub const kDepthTextureEnabledKeyword: &'static str = "DEPTH_TEXTURE_ENABLED";
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnPreRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPreRender", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetShaderKeyword(
        &mut self,
        keyword: *mut crate::System::String,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetShaderKeyword", (keyword, value))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
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
#[cfg(feature = "DepthTextureController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DepthTextureController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
