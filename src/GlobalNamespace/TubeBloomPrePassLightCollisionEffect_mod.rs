#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeBloomPrePassLightCollisionEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tubeBloomPrePassLight: *mut TubeBloomPrePassLight,
    pub _environmentLayerMask: crate::UnityEngine::LayerMask,
    pub _showHitPoint: bool,
    pub _hitPointGameObject: *mut crate::UnityEngine::GameObject,
    pub _hitPointTransform: *mut crate::UnityEngine::Transform,
    pub _hitPointLightWithId: *mut InstancedMaterialLightWithId,
    pub _hitPointDistanceToAlphaCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _transform: *mut crate::UnityEngine::Transform,
    pub _hitPointState: bool,
}
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TubeBloomPrePassLightCollisionEffect => ""
    ."TubeBloomPrePassLightCollisionEffect"
);
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl std::ops::Deref for TubeBloomPrePassLightCollisionEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl std::ops::DerefMut for TubeBloomPrePassLightCollisionEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl TubeBloomPrePassLightCollisionEffect {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "TubeBloomPrePassLightCollisionEffect")]
impl quest_hook::libil2cpp::ObjectType for TubeBloomPrePassLightCollisionEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
