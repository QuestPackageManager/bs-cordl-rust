#[cfg(feature = "BloomPrePassBackgroundParticleSystemRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundParticleSystemRenderer {
    __cordl_parent: BloomPrePassBackgroundNonLightRendererCore,
    pub _particleSystem: *mut crate::UnityEngine::ParticleSystem,
    pub _renderer: *mut crate::UnityEngine::Renderer,
}
#[cfg(feature = "BloomPrePassBackgroundParticleSystemRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BloomPrePassBackgroundParticleSystemRenderer => ""
    ."BloomPrePassBackgroundParticleSystemRenderer"
);
#[cfg(feature = "BloomPrePassBackgroundParticleSystemRenderer")]
impl std::ops::Deref for BloomPrePassBackgroundParticleSystemRenderer {
    type Target = BloomPrePassBackgroundNonLightRendererCore;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundParticleSystemRenderer")]
impl std::ops::DerefMut for BloomPrePassBackgroundParticleSystemRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundParticleSystemRenderer")]
impl BloomPrePassBackgroundParticleSystemRenderer {
    pub fn get_renderer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Renderer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Renderer = __cordl_object
            .invoke("get_renderer", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BloomPrePassBackgroundParticleSystemRenderer")]
impl quest_hook::libil2cpp::ObjectType for BloomPrePassBackgroundParticleSystemRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
