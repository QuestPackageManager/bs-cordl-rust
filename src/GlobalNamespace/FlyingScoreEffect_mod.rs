#[cfg(feature = "FlyingScoreEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreEffect {
    __cordl_parent: FlyingObjectEffect,
    pub _fadeAnimationCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _maxCutDistanceScoreIndicator: *mut crate::UnityEngine::SpriteRenderer,
    pub _text: *mut crate::TMPro::TextMeshPro,
    pub _color: crate::UnityEngine::Color,
    pub _colorAMultiplier: f32,
    pub _registeredToCallbacks: bool,
    pub _cutScoreBuffer: *mut IReadonlyCutScoreBuffer,
}
#[cfg(feature = "FlyingScoreEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for FlyingScoreEffect => ""."FlyingScoreEffect"
);
#[cfg(feature = "FlyingScoreEffect")]
impl std::ops::Deref for FlyingScoreEffect {
    type Target = FlyingObjectEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl std::ops::DerefMut for FlyingScoreEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl FlyingScoreEffect {
    #[cfg(feature = "FlyingScoreEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::FlyingScoreEffect_Pool;
    pub fn ManualUpdate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (t))?;
        Ok(__cordl_ret)
    }
    pub fn HandleCutScoreBufferDidChange(
        &mut self,
        cutScoreBuffer: *mut CutScoreBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCutScoreBufferDidChange", (cutScoreBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterCallbacksIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacksIfNeeded", ())?;
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
    pub fn InitAndPresent(
        &mut self,
        cutScoreBuffer: *mut IReadonlyCutScoreBuffer,
        duration: f32,
        targetPos: crate::UnityEngine::Vector3,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitAndPresent", (cutScoreBuffer, duration, targetPos, color))?;
        Ok(__cordl_ret)
    }
    pub fn HandleCutScoreBufferDidFinish(
        &mut self,
        cutScoreBuffer: *mut CutScoreBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCutScoreBufferDidFinish", (cutScoreBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshScore(
        &mut self,
        score: i32,
        maxPossibleCutScore: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshScore", (score, maxPossibleCutScore))?;
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
#[cfg(feature = "FlyingScoreEffect")]
impl quest_hook::libil2cpp::ObjectType for FlyingScoreEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingScoreEffect+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreEffect_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<*mut FlyingScoreEffect>,
}
#[cfg(feature = "FlyingScoreEffect+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FlyingScoreEffect_Pool => ""
    ."FlyingScoreEffect/Pool"
);
#[cfg(feature = "FlyingScoreEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingScoreEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<*mut FlyingScoreEffect>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreEffect+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingScoreEffect_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreEffect+Pool")]
impl crate::GlobalNamespace::FlyingScoreEffect_Pool {
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
    pub fn OnDespawned(
        &mut self,
        item: *mut FlyingScoreEffect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDespawned", (item))?;
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
#[cfg(feature = "FlyingScoreEffect+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FlyingScoreEffect_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
