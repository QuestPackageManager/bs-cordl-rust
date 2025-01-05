#[cfg(feature = "ParticleSystemLightWithIds")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemLightWithIds {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::RuntimeLightWithIds,
    >,
    pub _particleSystem: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _setOnlyOnce: bool,
    pub _setColorOnly: bool,
    pub _minAlpha: f32,
    pub _mainModule: crate::UnityEngine::ParticleSystem_MainModule,
    pub _particles: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ParticleSystem_Particle>,
    >,
}
#[cfg(feature = "ParticleSystemLightWithIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ParticleSystemLightWithIds =>
    ""."ParticleSystemLightWithIds"
);
#[cfg(feature = "ParticleSystemLightWithIds")]
impl std::ops::Deref for crate::GlobalNamespace::ParticleSystemLightWithIds {
    type Target = quest_hook::libil2cpp::Gc<crate::GlobalNamespace::RuntimeLightWithIds>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemLightWithIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::ParticleSystemLightWithIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ParticleSystemLightWithIds")]
impl crate::GlobalNamespace::ParticleSystemLightWithIds {
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
    pub fn ColorWasSet(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ColorWasSet", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "ParticleSystemLightWithIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ParticleSystemLightWithIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
