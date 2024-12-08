#[cfg(feature = "ObstacleMaterialSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleMaterialSetter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lwCoreMaterial: *mut crate::UnityEngine::Material,
    pub _hwCoreMaterial: *mut crate::UnityEngine::Material,
    pub _texturedCoreMaterial: *mut crate::UnityEngine::Material,
    pub _fakeGlowLWMaterial: *mut crate::UnityEngine::Material,
    pub _fakeGlowTexturedMaterial: *mut crate::UnityEngine::Material,
    pub _obstacleCoreRenderer: *mut crate::UnityEngine::Renderer,
    pub _obstacleFakeGlowRenderer: *mut crate::UnityEngine::Renderer,
    pub _graphicSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _currentPreset: *mut crate::BeatSaber::PerformancePresets::PerformancePreset,
}
#[cfg(feature = "ObstacleMaterialSetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ObstacleMaterialSetter => ""."ObstacleMaterialSetter"
);
#[cfg(feature = "ObstacleMaterialSetter")]
impl std::ops::Deref for ObstacleMaterialSetter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleMaterialSetter")]
impl std::ops::DerefMut for ObstacleMaterialSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleMaterialSetter")]
impl ObstacleMaterialSetter {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetCoreMaterial(
        &mut self,
        renderer: *mut crate::UnityEngine::Renderer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCoreMaterial", (renderer))?;
        Ok(__cordl_ret)
    }
    pub fn SetFakeGlowMaterial(
        &mut self,
        renderer: *mut crate::UnityEngine::Renderer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFakeGlowMaterial", (renderer))?;
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
#[cfg(feature = "ObstacleMaterialSetter")]
impl quest_hook::libil2cpp::ObjectType for ObstacleMaterialSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
