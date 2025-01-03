#[cfg(feature = "FireworkItemController")]
#[repr(C)]
#[derive(Debug)]
pub struct FireworkItemController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _particleSystems: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem,
    >,
    pub _lights: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::TubeBloomPrePassLight,
    >,
    pub _audioSource: *mut crate::UnityEngine::AudioSource,
    pub _lightFlashDuration: f32,
    pub _lightIntensityCurve: *mut crate::UnityEngine::AnimationCurve,
    pub _lightIntensityMultiplier: f32,
    pub _randomizeColor: bool,
    pub _lightsColor: crate::UnityEngine::Color,
    pub _lightsColorGradient: *mut crate::UnityEngine::Gradient,
    pub _randomizeSpeed: bool,
    pub _minSpeedMultiplier: f32,
    pub _maxSpeedMultiplier: f32,
    pub _explosionClips: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _randomAudioPicker: *mut crate::GlobalNamespace::RandomObjectPicker_1<
        *mut crate::UnityEngine::AudioClip,
    >,
    pub _directionalLight: *mut crate::GlobalNamespace::DirectionalLight,
    pub _directionalLightIntensity: f32,
    pub _initialized: bool,
    pub didFinishEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::FireworkItemController,
    >,
}
#[cfg(feature = "FireworkItemController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FireworkItemController => ""
    ."FireworkItemController"
);
#[cfg(feature = "FireworkItemController")]
impl std::ops::Deref for crate::GlobalNamespace::FireworkItemController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController")]
impl std::ops::DerefMut for crate::GlobalNamespace::FireworkItemController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController")]
impl crate::GlobalNamespace::FireworkItemController {
    #[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
    pub type FireworkItemParticleSystem = crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem;
    #[cfg(feature = "FireworkItemController+Pool")]
    pub type Pool = crate::GlobalNamespace::FireworkItemController_Pool;
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
    pub fn Fire(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fire", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FireCoroutine(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("FireCoroutine", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeParticleSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeParticleSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PlayExplosionSound(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PlayExplosionSound", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLightsColor(
        &mut self,
        intensity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLightsColor", (intensity))?;
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
    pub fn add_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::FireworkItemController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didFinishEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::FireworkItemController>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didFinishEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_directionalLight(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DirectionalLight>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_directionalLight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_directionalLightIntensity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_directionalLightIntensity", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FireworkItemController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FireworkItemController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct FireworkItemController_FireworkItemParticleSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _particleSystem: *mut crate::UnityEngine::ParticleSystem,
    pub _isSubemitter: bool,
    pub _useMainColor: bool,
    pub _useOwnGradient: bool,
    pub _particleColorGradient: *mut crate::UnityEngine::Gradient,
    pub _randomizeSpeed: bool,
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem => ""
    ."FireworkItemController/FireworkItemParticleSystem"
);
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl std::ops::Deref
for crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl std::ops::DerefMut
for crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "FireworkItemController+FireworkItemParticleSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FireworkItemController_FireworkItemParticleSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FireworkItemController+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct FireworkItemController_Pool {
    __cordl_parent: crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::FireworkItemController,
    >,
}
#[cfg(feature = "FireworkItemController+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FireworkItemController_Pool =>
    ""."FireworkItemController/Pool"
);
#[cfg(feature = "FireworkItemController+Pool")]
impl std::ops::Deref for crate::GlobalNamespace::FireworkItemController_Pool {
    type Target = crate::Zenject::MonoMemoryPool_1<
        *mut crate::GlobalNamespace::FireworkItemController,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+Pool")]
impl std::ops::DerefMut for crate::GlobalNamespace::FireworkItemController_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FireworkItemController+Pool")]
impl crate::GlobalNamespace::FireworkItemController_Pool {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "FireworkItemController+Pool")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FireworkItemController_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
