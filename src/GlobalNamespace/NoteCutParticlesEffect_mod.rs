#[cfg(feature = "NoteCutParticlesEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteCutParticlesEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _sparklesPS: *mut crate::UnityEngine::ParticleSystem,
    pub _explosionPS: *mut crate::UnityEngine::ParticleSystem,
    pub _explosionCorePS: *mut crate::UnityEngine::ParticleSystem,
    pub _explosionPrePassBloomPS: *mut crate::UnityEngine::ParticleSystem,
    pub _sparklesPSEmitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _sparklesPSMainModule: crate::UnityEngine::ParticleSystem_MainModule,
    pub _sparklesPSShapeModule: crate::UnityEngine::ParticleSystem_ShapeModule,
    pub _sparklesLifetimeMinMaxCurve: crate::UnityEngine::ParticleSystem_MinMaxCurve,
    pub _explosionPSEmitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _explosionCorePSEmitParams: crate::UnityEngine::ParticleSystem_EmitParams,
    pub _explosionCorePSMainModule: crate::UnityEngine::ParticleSystem_MainModule,
    pub _explosionCorePSShapeModule: crate::UnityEngine::ParticleSystem_ShapeModule,
    pub _explosionPrePassBloomPSShapeModule: crate::UnityEngine::ParticleSystem_ShapeModule,
}
#[cfg(feature = "NoteCutParticlesEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NoteCutParticlesEffect => ""
    ."NoteCutParticlesEffect"
);
#[cfg(feature = "NoteCutParticlesEffect")]
impl std::ops::Deref for crate::GlobalNamespace::NoteCutParticlesEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutParticlesEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoteCutParticlesEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteCutParticlesEffect")]
impl crate::GlobalNamespace::NoteCutParticlesEffect {
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SpawnParticles(
        &mut self,
        cutPoint: crate::UnityEngine::Vector3,
        cutNormal: crate::UnityEngine::Vector3,
        saberDir: crate::UnityEngine::Vector3,
        saberSpeed: f32,
        noteMovementVec: crate::UnityEngine::Vector3,
        color: crate::UnityEngine::Color32,
        sparkleParticlesCount: i32,
        explosionParticlesCount: i32,
        lifetimeMultiplier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SpawnParticles",
                (
                    cutPoint,
                    cutNormal,
                    saberDir,
                    saberSpeed,
                    noteMovementVec,
                    color,
                    sparkleParticlesCount,
                    explosionParticlesCount,
                    lifetimeMultiplier,
                ),
            )?;
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
#[cfg(feature = "NoteCutParticlesEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteCutParticlesEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
