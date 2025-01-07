#[cfg(feature = "FlyingScoreEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreEffect {
    __cordl_parent: crate::GlobalNamespace::FlyingObjectEffect,
    pub _fadeAnimationCurve: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::AnimationCurve,
    >,
    pub _maxCutDistanceScoreIndicator: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::SpriteRenderer,
    >,
    pub _text: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshPro>,
    pub _color: crate::UnityEngine::Color,
    pub _colorAMultiplier: f32,
    pub _registeredToCallbacks: bool,
    pub _cutScoreBuffer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IReadonlyCutScoreBuffer,
    >,
}
#[cfg(feature = "FlyingScoreEffect")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::FlyingScoreEffect {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FlyingScoreEffect";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingScoreEffect {
    type Target = crate::GlobalNamespace::FlyingObjectEffect;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlyingScoreEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl crate::GlobalNamespace::FlyingScoreEffect {
    #[cfg(feature = "FlyingScoreEffect+Pool")]
    pub type Pool = crate::GlobalNamespace::FlyingScoreEffect_Pool;
    pub fn HandleCutScoreBufferDidChange(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CutScoreBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCutScoreBufferDidChange", (cutScoreBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCutScoreBufferDidFinish(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::CutScoreBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCutScoreBufferDidFinish", (cutScoreBuffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitAndPresent(
        &mut self,
        cutScoreBuffer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IReadonlyCutScoreBuffer,
        >,
        duration: f32,
        targetPos: crate::UnityEngine::Vector3,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitAndPresent", (cutScoreBuffer, duration, targetPos, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn ManualUpdate(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ManualUpdate", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallbacksIfNeeded(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterCallbacksIfNeeded", ())?;
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
#[cfg(feature = "FlyingScoreEffect")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FlyingScoreEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl AsRef<crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver>
for crate::GlobalNamespace::FlyingScoreEffect {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl AsMut<crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver>
for crate::GlobalNamespace::FlyingScoreEffect {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ICutScoreBufferDidChangeReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl AsRef<crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver>
for crate::GlobalNamespace::FlyingScoreEffect {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FlyingScoreEffect")]
impl AsMut<crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver>
for crate::GlobalNamespace::FlyingScoreEffect {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ICutScoreBufferDidFinishReceiver {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FlyingScoreEffect+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct FlyingScoreEffect_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FlyingScoreEffect>,
    >,
}
#[cfg(feature = "FlyingScoreEffect+Pool")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FlyingScoreEffect_Pool {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Pool";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "FlyingScoreEffect+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::FlyingScoreEffect_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FlyingScoreEffect>,
    >;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDespawned(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::FlyingScoreEffect>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDespawned", (item))?;
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
