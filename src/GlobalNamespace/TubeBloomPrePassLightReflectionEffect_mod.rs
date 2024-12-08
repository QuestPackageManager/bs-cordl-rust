#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TubeBloomPrePassLightReflectionEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _mainTubeBloomPrePassLight: *mut crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint,
    pub _tubeBloomPrePassLightBounces: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint,
    >,
    pub _environmentLayerMask: crate::UnityEngine::LayerMask,
    pub _environmentCollisionRepository: *mut EnvironmentCollisionRepository,
    pub _transform: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TubeBloomPrePassLightReflectionEffect => ""
    ."TubeBloomPrePassLightReflectionEffect"
);
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
impl std::ops::Deref for TubeBloomPrePassLightReflectionEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
impl std::ops::DerefMut for TubeBloomPrePassLightReflectionEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
impl TubeBloomPrePassLightReflectionEffect {
    #[cfg(
        feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
    )]
    pub type TubeBloomPrePassLightWithHitPoint = crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint;
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
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
    pub fn RaycastAndSetLightLength(
        &mut self,
        bounce: *mut crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint,
        rayWorldOrigin: crate::UnityEngine::Vector3,
        rayDirection: crate::UnityEngine::Vector3,
        hitWorldPosition: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        hitReflection: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        length: quest_hook::libil2cpp::ByRefMut<f32>,
        endAlpha: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "RaycastAndSetLightLength",
                (
                    bounce,
                    rayWorldOrigin,
                    rayDirection,
                    hitWorldPosition,
                    hitReflection,
                    length,
                    endAlpha,
                ),
            )?;
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
#[cfg(feature = "TubeBloomPrePassLightReflectionEffect")]
impl quest_hook::libil2cpp::ObjectType for TubeBloomPrePassLightReflectionEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    __cordl_parent: crate::System::Object,
    pub light: *mut TubeBloomPrePassLight,
    pub showHitPoint: bool,
    pub hitPointGameObject: *mut crate::UnityEngine::GameObject,
    pub hitPointTransform: *mut crate::UnityEngine::Transform,
    pub hitPointLightWithId: *mut InstancedMaterialLightWithId,
    pub hitPointDistanceToAlphaCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _hitPointState: bool,
}
#[cfg(
    feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint
    => ""."TubeBloomPrePassLightReflectionEffect/TubeBloomPrePassLightWithHitPoint"
);
#[cfg(
    feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
impl std::ops::Deref
for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
impl std::ops::DerefMut
for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
impl crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    pub fn SetActive(
        &mut self,
        enabled: bool,
        mainLightColor: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActive", (enabled, mainLightColor))?;
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
    pub fn SetCollisionLength(
        &mut self,
        rayHitGeometry: bool,
        hit: crate::UnityEngine::RaycastHit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCollisionLength", (rayHitGeometry, hit))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        remainingLength: f32,
        startAlpha: f32,
        hitWorldPosition: crate::UnityEngine::Vector3,
        hitReflection: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (remainingLength, startAlpha, hitWorldPosition, hitReflection),
            )?;
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
#[cfg(
    feature = "TubeBloomPrePassLightReflectionEffect+TubeBloomPrePassLightWithHitPoint"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TubeBloomPrePassLightReflectionEffect_TubeBloomPrePassLightWithHitPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
