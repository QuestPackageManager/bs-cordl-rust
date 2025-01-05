#[cfg(feature = "BombExplosionEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct BombExplosionEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _debrisPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _explosionPS: quest_hook::libil2cpp::Gc<crate::UnityEngine::ParticleSystem>,
    pub _debrisCount: i32,
    pub _explosionParticlesCount: i32,
    pub _emitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _explosionPSEmitParams: crate::UnityEngine::ParticleSystem_EmitParams,
}
#[cfg(feature = "BombExplosionEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BombExplosionEffect => ""
    ."BombExplosionEffect"
);
#[cfg(feature = "BombExplosionEffect")]
impl std::ops::Deref for crate::GlobalNamespace::BombExplosionEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BombExplosionEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::BombExplosionEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BombExplosionEffect")]
impl crate::GlobalNamespace::BombExplosionEffect {
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
    pub fn SpawnExplosion(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnExplosion", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnExplosionWithOffset(
        &mut self,
        pos: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SpawnExplosionWithOffset", (pos))?;
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
#[cfg(feature = "BombExplosionEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BombExplosionEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
