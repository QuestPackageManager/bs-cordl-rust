#[cfg(feature = "ObstacleSaberSparkleEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct ObstacleSaberSparkleEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _sparkleParticleSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ParticleSystem,
    >,
    pub _burnParticleSystem: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ParticleSystem,
    >,
    pub _sparkleParticleSystemEmissionModule: crate::UnityEngine::ParticleSystem_EmissionModule,
    pub _burnParticleSystemEmissionModule: crate::UnityEngine::ParticleSystem_EmissionModule,
}
#[cfg(feature = "ObstacleSaberSparkleEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ObstacleSaberSparkleEffect =>
    ""."ObstacleSaberSparkleEffect"
);
#[cfg(feature = "ObstacleSaberSparkleEffect")]
impl std::ops::Deref for crate::GlobalNamespace::ObstacleSaberSparkleEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::ObstacleSaberSparkleEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffect")]
impl crate::GlobalNamespace::ObstacleSaberSparkleEffect {
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
    pub fn IsEmitting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEmitting", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetPositionAndRotation(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        rot: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPositionAndRotation", (pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn StartEmission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartEmission", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopEmission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopEmission", ())?;
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
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ObstacleSaberSparkleEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ObstacleSaberSparkleEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
