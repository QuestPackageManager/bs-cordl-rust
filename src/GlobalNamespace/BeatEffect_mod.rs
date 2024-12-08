#[cfg(feature = "BeatEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spriteRenderer: *mut crate::UnityEngine::SpriteRenderer,
    pub _spriteTransform: *mut crate::UnityEngine::Transform,
    pub _tubeBloomPrePassLight: *mut TubeBloomPrePassLight,
    pub _lightIntensityCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _spriteXScaleCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _spriteYScaleCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _transparencyCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _didFinishEvent: *mut LazyCopyHashSet_1<*mut IBeatEffectDidFinishEvent>,
    pub _animationDuration: f32,
    pub _elapsedTime: f32,
    pub _color: crate::UnityEngine::Color,
}
#[cfg(feature = "BeatEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatEffect => ""."BeatEffect"
);
#[cfg(feature = "BeatEffect")]
impl std::ops::Deref for BeatEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect")]
impl std::ops::DerefMut for BeatEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect")]
impl BeatEffect {
    #[cfg(feature = "BeatEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::BeatEffect_Pool;
    pub fn Init(
        &mut self,
        color: crate::UnityEngine::Color,
        animationDuration: f32,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (color, animationDuration, rotation))?;
        Ok(__cordl_ret)
    }
    pub fn ManualUpdate(
        &mut self,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (deltaTime))?;
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
    pub fn get_didFinishEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut ILazyCopyHashSet_1<*mut IBeatEffectDidFinishEvent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut ILazyCopyHashSet_1<*mut IBeatEffectDidFinishEvent> = __cordl_object
            .invoke("get_didFinishEvent", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatEffect")]
impl quest_hook::libil2cpp::ObjectType for BeatEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatEffect+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatEffect_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut BeatEffect>,
}
#[cfg(feature = "BeatEffect+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatEffect_Pool => ""
    ."BeatEffect/Pool"
);
#[cfg(feature = "BeatEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::BeatEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut BeatEffect>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatEffect_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatEffect+Pool")]
impl crate::GlobalNamespace::BeatEffect_Pool {
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
}
#[cfg(feature = "BeatEffect+Pool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatEffect_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
