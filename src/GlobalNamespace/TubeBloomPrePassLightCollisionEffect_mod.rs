#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeBloomPrePassLightCollisionEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tubeBloomPrePassLight: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TubeBloomPrePassLight,
    >,
    pub _environmentLayerMask: crate::UnityEngine::LayerMask,
    pub _showHitPoint: bool,
    pub _hitPointGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _hitPointTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _hitPointLightWithId: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::InstancedMaterialLightWithId,
    >,
    pub _hitPointDistanceToAlphaCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _hitPointState: bool,
}
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TubeBloomPrePassLightCollisionEffect => ""
    ."TubeBloomPrePassLightCollisionEffect"
);
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl std::ops::Deref for crate::GlobalNamespace::TubeBloomPrePassLightCollisionEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl std::ops::DerefMut
for crate::GlobalNamespace::TubeBloomPrePassLightCollisionEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl crate::GlobalNamespace::TubeBloomPrePassLightCollisionEffect {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TubeBloomPrePassLightCollisionEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
