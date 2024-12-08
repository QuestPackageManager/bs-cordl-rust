#[cfg(feature = "ParticleSystemLightWithId")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemLightWithId {
    __cordl_parent: crate::GlobalNamespace::LightWithIdMonoBehaviour,
    pub _particleSystem: *mut crate::UnityEngine::ParticleSystem,
    pub setOnlyOnce: bool,
    pub _setColorOnly: bool,
    pub _intensity: f32,
    pub _minAlpha: f32,
    pub _mainModule: crate::UnityEngine::ParticleSystem_MainModule,
    pub _particles: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::ParticleSystem_Particle,
    >,
}
#[cfg(feature = "ParticleSystemLightWithId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ParticleSystemLightWithId => ""
    ."ParticleSystemLightWithId"
);
#[cfg(feature = "ParticleSystemLightWithId")]
impl std::ops::Deref for crate::GlobalNamespace::ParticleSystemLightWithId {
    type Target = crate::GlobalNamespace::LightWithIdMonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemLightWithId")]
impl std::ops::DerefMut for crate::GlobalNamespace::ParticleSystemLightWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemLightWithId")]
impl crate::GlobalNamespace::ParticleSystemLightWithId {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (color))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "ParticleSystemLightWithId")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ParticleSystemLightWithId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
